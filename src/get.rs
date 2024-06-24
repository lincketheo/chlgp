use crate::models::{ChangelogIncludes, GetInputs};
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};

pub struct ChangelogItem {
    body: String,
    version: String,
    date: String,
}

impl ChangelogItem {
    fn write_json<W: Write>(
        &self,
        includes: &ChangelogIncludes,
        writer: &mut BufWriter<W>,
    ) -> Result<(), std::io::Error> {
        writer.write_all(b"{")?;

        let mut writes: Vec<(String, String)> = Vec::new();

        if includes.body {
            writes.push(("body".to_string(), escape_special_characters(&self.body)));
        }
        if includes.version {
            writes.push(("version".to_string(), escape_special_characters(&self.version)));
        }
        if includes.date {
            writes.push(("date".to_string(), escape_special_characters(&self.date)));
        }

        let mut writes = writes.iter().peekable();

        while let Some((key, value)) = writes.next() {
            writer.write_all(format!("\"{}\":\"{}\"", key, value).as_bytes())?;
            if writes.peek().is_some() {
                writer.write_all(b",")?;
            }
        }

        writer.write_all(b"}")?;

        Ok(())
    }
}

fn escape_special_characters(input: &str) -> String {
    input
        .replace("\\", "\\\\")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t")
        .replace("\x08", "\\b")
        .replace("\x0c", "\\f")
        .replace("\"", "\\\"")
}

fn set_position_to_first_version<R: Read>(reader: &mut BufReader<R>) -> io::Result<usize> {
    let mut buffer = Vec::new();
    reader.read_until(b'[', &mut buffer)
}
fn read_version<R: Read>(reader: &mut BufReader<R>, buffer: &mut Vec<u8>) -> io::Result<usize> {
    let ret = reader.read_until(b']', buffer);
    buffer.pop();
    ret
}
fn read_date<R: Read>(reader: &mut BufReader<R>, buffer: &mut Vec<u8>) -> io::Result<usize> {
    let _ = reader.read_until(b' ', buffer);
    let ret = reader.read_until(b' ', buffer);
    buffer.pop();
    ret
}
fn read_body<R: Read>(reader: &mut BufReader<R>, buffer: &mut Vec<u8>) -> io::Result<usize> {
    let ret = reader.read_until(b'[', buffer);
    buffer.pop();
    ret
}

fn parse_changelog_get<R: Read>(
    changelog: &mut BufReader<R>,
) -> Result<Vec<ChangelogItem>, std::io::Error> {
    let mut ret = Vec::new();

    // Go to the start of the actual content (skips all the header information)
    let _ = set_position_to_first_version(changelog);

    loop {
        let mut version: Vec<u8> = Vec::new();
        let mut date: Vec<u8> = Vec::new();
        let mut body: Vec<u8> = Vec::new();

        // fail fast when we can't read anymore
        // TODO - think of a better way of doing this
        if read_version(changelog, &mut version)? == 0 {
            break;
        }
        if read_date(changelog, &mut date)? == 0 {
            break;
        }
        if read_body(changelog, &mut body)? == 0 {
            break;
        }

        let version = String::from_utf8_lossy(&version).trim().to_string();
        let date = String::from_utf8_lossy(&date).trim().to_string();
        let body = String::from_utf8_lossy(&body).trim().to_string();

        let item = ChangelogItem {
            version,
            date,
            body,
        };
        ret.push(item);
    }

    Ok(ret)
}

fn write_list_of_changelog_items_json<W: Write>(
    items: Vec<ChangelogItem>,
    includes: ChangelogIncludes,
    writer: &mut BufWriter<W>,
) -> Result<(), std::io::Error> {
    writer.write_all(b"[")?;
    let mut items = items.iter().peekable();
    while let Some(item) = items.next() {
        item.write_json(&includes, writer)?;
        if items.peek().is_some() {
            writer.write_all(b",")?;
        }
    }
    writer.write_all(b"]")?;
    Ok(())
}

pub fn execute_get(args: GetInputs) -> Result<(), String> {
    let file = match File::open(args.filename) {
        Ok(file) => file,
        Err(msg) => {
            return Err(format!("{}", msg));
        }
    };

    let mut reader = BufReader::new(file);
    let items = parse_changelog_get(&mut reader).map_err(|it| it.to_string())?;
    
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = BufWriter::new(handle);

    write_list_of_changelog_items_json(items, args.includes, &mut writer).map_err(|it| it.to_string())?;
    writer.flush().map_err(|it| it.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_escape_special_characters() {
        let tc = "foo\\bar\nbiz\rbuz\tbaz\x08fizz\x0cfazz\"".to_string();
        let exp = "foo\\\\bar\\nbiz\\rbuz\\tbaz\\bfizz\\ffazz\\\"".to_string();
        assert_eq!(escape_special_characters(&tc), exp);
    }
}
