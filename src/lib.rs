pub mod config;
pub mod encryption;
pub mod errors;
pub mod openai;

use crate::config::{CliArgs, Config};
use crate::openai::ask_chatgpt;
use spinoff::{spinners, Color, Spinner};

pub fn run(input: &str, config: &Config) -> anyhow::Result<String> {
    let spinner = Spinner::new(spinners::Dots, "Asking to ChatGPT...", Color::Blue);
    let response = ask_chatgpt(input, config.openai_api_key.as_str());
    spinner.stop();
    let response = response?;
    println!("{response}");
    Ok(response)
}

pub fn parse_cli_args(args: Vec<String>) -> CliArgs {
    let args: Vec<String> = args.iter().skip(1).cloned().collect();

    let input = args.join(" ");
    let mut show_help = false;
    let mut clear_saved_config = false;

    args.iter().for_each(|x| match x.as_str() {
        "-h" => show_help = true,
        "--help" => show_help = true,
        "--remove-config" => clear_saved_config = true,
        "--delete-config" => clear_saved_config = true,
        "--clear-config" => clear_saved_config = true,
        _ => {}
    });

    CliArgs {
        input,
        show_help,
        clear_saved_config,
    }
}
