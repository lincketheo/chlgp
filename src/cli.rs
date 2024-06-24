use crate::models::{Actions, ChangelogIncludes, Format, GetInputs};

impl Actions {
    pub fn from(args: &mut Vec<String>) -> Result<Self, String> {
        if args.len() == 0 {
            return Err(format!(
                "Invalid CLI Usage, expects one action of any: {}",
                Self::options_str()
            ));
        }

        let action_str = args.remove(0);

        match action_str.as_str() {
            "get" => Ok(Actions::GET),
            _ => Err(format!("Invalid action: {}", action_str)),
        }
    }

    fn options_str() -> String {
        return format!("[{}]", Actions::GET);
    }
}

pub fn parse_get(args: &mut Vec<String>) -> Result<GetInputs, String> {
    if args.len() == 0 {
        return Err(format!(
            "Invalid CLI Usage\nUsage:\n{} <filename>",
            Actions::GET
        ));
    }

    let mut args = args.iter();
    let mut ret = GetInputs {
        filename: args
            .next()
            .expect("Something went wrong") // Already verified has >= 1
            .clone(),
        includes: ChangelogIncludes {
            body: false,
            date: false,
            version: false,
        },
        format: Format::JSON,
    };

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "body" => ret.includes.body = true,
            "version" => ret.includes.version = true,
            "date" => ret.includes.date = true,
            "json" => ret.format = Format::JSON,
            _ => {
                return Err(format!("Invalid get parameter: {}", &arg));
            }
        }
    }

    if ret.includes.all_false() {
        ret.includes.make_all_true()
    }
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_get_just_filename() {
        let mut tc = Vec::new();
        tc.push("filename".to_string());
        let result = parse_get(&mut tc);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.filename, "filename".to_string());
        assert_eq!(result.format, Format::JSON);
        assert!(result.includes.body);
        assert!(result.includes.date);
        assert!(result.includes.version);
    }

    #[test]
    fn parse_get_include_1() {
        let mut tc = Vec::new();
        tc.push("filename".to_string());
        tc.push("body".to_string());
        let result = parse_get(&mut tc);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.filename, "filename".to_string());
        assert_eq!(result.format, Format::JSON);
        assert!(result.includes.body);
        assert!(!result.includes.date);
        assert!(!result.includes.version);
    }

    #[test]
    fn parse_get_include_2() {
        let mut tc = Vec::new();
        tc.push("filename".to_string());
        tc.push("body".to_string());
        tc.push("date".to_string());
        let result = parse_get(&mut tc);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.filename, "filename".to_string());
        assert_eq!(result.format, Format::JSON);
        assert!(result.includes.body);
        assert!(result.includes.date);
        assert!(!result.includes.version);
    }

    #[test]
    fn parse_get_all_args() {
        let mut tc = Vec::new();
        tc.push("filename".to_string());
        tc.push("body".to_string());
        tc.push("date".to_string());
        tc.push("json".to_string());
        let result = parse_get(&mut tc);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.filename, "filename".to_string());
        assert_eq!(result.format, Format::JSON);
        assert!(result.includes.body);
        assert!(result.includes.date);
        assert!(!result.includes.version);
    }

    #[test]
    fn parse_get_unrecognized_arg_1() {
        let mut tc = Vec::new();
        tc.push("filename".to_string());
        tc.push("foo".to_string());
        let result = parse_get(&mut tc);
        assert!(result.is_err());
    }

    #[test]
    fn parse_get_unrecognized_arg_2() {
        let mut tc = Vec::new();
        tc.push("filename".to_string());
        tc.push("body".to_string());
        tc.push("date".to_string());
        tc.push("biz".to_string());
        tc.push("json".to_string());
        let result = parse_get(&mut tc);
        assert!(result.is_err());
    }
}
