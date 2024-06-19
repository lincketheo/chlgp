use crate::inputs::{ChangelogIncludes, Format, GetInputs};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

pub struct ChangelogItem {
    body: String,
    version: String,
    date: String,
}

impl ChangelogItem {
    fn print_json(&self, includes: &ChangelogIncludes) {
        if includes.all_false() {
            print!("null");
            return;
        }
        if includes.get_count() > 1 {
            print!("{{");
        }
        let mut needs_comma_before = false;
        if includes.body {
            if includes.get_count() > 1 {
                print!("\"body\":");
            }
            print!("\"{}\"", self.body);
            needs_comma_before = true;
        }
        if includes.version {
            if needs_comma_before {
                print!(",");
            }
            if includes.get_count() > 1 {
                print!("  \"version\": ");
            }
            print!("\"{}\"", self.version);
            needs_comma_before = true;
        }
        if includes.date {
            if needs_comma_before {
                print!(",");
            }
            if includes.get_count() > 1 {
                print!("  \"date\": ");
            }
            print!("\"{}\"", self.date);
        }
        if includes.get_count() > 1 {
            print!("}}");
        }
    }
}

pub trait ChangelogItemIterator<R: Read> {
    fn changelog_elems(self) -> ChangelogItems<R>;
}

pub struct ChangelogItems<R: Read> {
    reader: BufReader<R>,
    first: bool,
}

impl<R: Read> ChangelogItems<R> {
    fn set_position_to_first_version(&mut self) -> io::Result<usize> {
        let mut buffer = Vec::new();
        self.reader.read_until(b'[', &mut buffer)
    }
    fn read_version(&mut self, buffer: &mut Vec<u8>) -> io::Result<usize> {
        let ret = self.reader.read_until(b']', buffer);
        buffer.pop();
        ret
    }
    fn read_date(&mut self, buffer: &mut Vec<u8>) -> io::Result<usize> {
        let _ = self.reader.read_until(b' ', buffer);
        let ret = self.reader.read_until(b' ', buffer);
        buffer.pop();
        ret
    }
    fn read_body(&mut self, buffer: &mut Vec<u8>) -> io::Result<usize> {
        let ret = self.reader.read_until(b'[', buffer);
        buffer.pop();
        ret
    }
}

impl<R: Read> ChangelogItemIterator<R> for BufReader<R> {
    fn changelog_elems(self) -> ChangelogItems<R> {
        ChangelogItems {
            reader: self,
            first: true,
        }
    }
}

fn escape_special_characters(input: &str) -> String {
    input
        .replace("\\", "\\\\") // This must be the first replacement
        .replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t")
        .replace("\x08", "\\b") // Backspace
        .replace("\x0c", "\\f") // Form feed
        .replace("\"", "\\\"") // Double quote
}

impl<R: Read> Iterator for ChangelogItems<R> {
    type Item = ChangelogItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            if let Ok(0) = self.set_position_to_first_version() {
                return None;
            }
            self.first = false;
        }
        let mut version: Vec<u8> = Vec::new();
        let mut date: Vec<u8> = Vec::new();
        let mut body: Vec<u8> = Vec::new();
        if let Ok(0) = self.read_version(&mut version) {
            return None;
        }
        if let Ok(0) = self.read_date(&mut date) {
            return None;
        }
        if let Ok(0) = self.read_body(&mut body) {
            return None;
        }

        let version = String::from_utf8_lossy(&version).trim().to_string();
        let date = String::from_utf8_lossy(&date).trim().to_string();
        let body = String::from_utf8_lossy(&body).trim().to_string();

        let version = escape_special_characters(&version);
        let date = escape_special_characters(&date);
        let body = escape_special_characters(&body);

        Some(ChangelogItem {
            version,
            date,
            body,
        })
    }
}

pub fn execute_get(args: GetInputs) -> Result<(), String> {
    let n = if args.n == 0 {
        usize::MAX
    } else {
        args.n
    };

    let file = match File::open(args.filename) {
        Ok(file) => file,
        Err(msg) => {
            return Err(format!("{}", msg));
        }
    };

    let reader = BufReader::new(file);

    if n > 1 {
        print!("[")
    }

    let mut iter = reader.changelog_elems();

    if let Some(item) = iter.next() {
        match args.format {
            Format::JSON => {
                item.print_json(&args.includes);
            }
        }
    }
    for item in iter.take(n - 1) {
        print!(",");
        match args.format {
            Format::JSON => {
                item.print_json(&args.includes);
            }
        }
    }

    if n > 1 {
        print!("]")
    }

    Ok(())
}
