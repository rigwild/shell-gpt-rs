mod config;
mod encryption;
mod errors;
mod openai;

use crate::config::{CliArgs, Config};
use crate::openai::ask_chatgpt;

pub fn run(input: &str, config: Config) -> anyhow::Result<String> {
    let response = ask_chatgpt(input, config.openai_api_key.as_str())?;
    println!("{response}");
    Ok(response)
}

pub fn parse_cli_args(args: Vec<String>) -> CliArgs {
    // We only collect input for now
    let input = args.iter().skip(1).cloned().collect();
    CliArgs { input }
}
