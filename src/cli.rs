use crate::inputs::{Actions, ChangelogIncludes, Format, GetInputs};

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
        n: 0,
        includes: ChangelogIncludes {
            body: false,
            date: false,
            version: false,
        },
        format: Format::JSON,
    };

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--head" => match args.next().map(|res| res.parse::<usize>()) {
                Some(Ok(n)) => {
                    ret.n = n;
                }
                Some(Err(msg)) => {
                    return Err(format!("Invalid parameter for --head. Error: {}", msg));
                }
                None => {
                    return Err(format!("Expecting parameter for --head."));
                }
            },
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
