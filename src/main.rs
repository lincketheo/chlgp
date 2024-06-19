mod cli;
mod get;
mod models;

use cli::parse_get;
use get::execute_get;
use models::Actions;

use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // Remove executable name

    match Actions::from(&mut args).and_then(|action| handle_action(&action, &mut args)) {
        Ok(_) => {

        }
        Err(msg) => {
            eprintln!("{}", msg);
        }
    }
}

pub fn handle_action(action: &Actions, args: &mut Vec<String>) -> Result<(), String> {
    match action {
        Actions::GET => parse_get(args).and_then(|cli_args| execute_get(cli_args)),
    }
}

