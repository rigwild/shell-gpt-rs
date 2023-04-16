pub mod config;
pub mod encryption;
pub mod errors;
pub mod openai;

use crate::config::{CliArgs, Config};
use rand::Rng;
use spinoff::{spinners, Color, Spinner};

const LOADING_MESSAGES: [&'static str; 7] = [
    "Asking ChatGPT...",
    "ChatGPT is thinking...",
    "ChatGPT is writing...",
    "Assistant is thinking...",
    "Assistant is writing...",
    "Invoking the AI gods...",
    "Calling an AI friend...",
];

pub fn run(cli_args: &CliArgs, config: &Config) -> anyhow::Result<String> {
    let spinner = Spinner::new(spinners::Dots, get_loading_message(), Color::Blue);
    let response = openai::ask_chatgpt(
        cli_args.input.as_str(),
        cli_args.pre_prompt,
        config.openai_api_key.as_str(),
    );
    spinner.stop();
    let response = response?;
    println!("{response}");
    Ok(response)
}

fn get_loading_message() -> String {
    let i = rand::thread_rng().gen_range(0..LOADING_MESSAGES.len());
    LOADING_MESSAGES[i].to_string()
}
