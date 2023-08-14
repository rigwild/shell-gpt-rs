pub mod config;
pub mod encryption;
pub mod errors;
pub mod openai;

use crate::config::{CliArgs, Config};
use rand::Rng;
use spinoff::{spinners, Color, Spinner};
use std::{io, process};

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
    if !cli_args.raw {
        let mut spinner = Spinner::new(spinners::Dots, get_loading_message(), Color::Blue);
        let response = openai::ask_chatgpt(
            cli_args.input.as_str(),
            cli_args.pre_prompt,
            config.openai_api_key.as_str(),
            cli_args.timeout,
        );
        spinner.stop();
        let response = response?;
        println!("\n{response}");

        if cli_args.pre_prompt == openai::PrePrompt::ShellScript {
            ask_then_run_script(response.as_str());
        }
        Ok(response)
    } else {
        let response = openai::ask_chatgpt(
            cli_args.input.as_str(),
            cli_args.pre_prompt,
            config.openai_api_key.as_str(),
            cli_args.timeout,
        );
        let mut response = response?;
        if cli_args.pre_prompt == openai::PrePrompt::ShellScript {
            response = openai::extract_code_block_if_needed(response.as_str())
        }
        println!("\n{response}");
        Ok(response)
    }
}

fn get_loading_message() -> String {
    let i = rand::thread_rng().gen_range(0..LOADING_MESSAGES.len());
    LOADING_MESSAGES[i].to_string()
}

fn ask_then_run_script(response: &str) {
    println!("\nDo you want to run this script? (y/N)");

    let mut yes_no = String::new();
    io::stdin()
        .read_line(&mut yes_no)
        .expect("Failed to read your input");
    yes_no = yes_no.trim().to_lowercase();
    if yes_no == "y" || yes_no == "yes" || yes_no == "o" || yes_no == "oui" {
        let script = openai::extract_code_block_if_needed(response);
        // let script = format!("set -x;{script}");
        println!("\nExecuting script...\n\n--------------\n\n");
        let output = process::Command::new("bash")
            .arg("-c")
            .arg(script)
            .output()
            .expect("Failed to execute script");
        if !output.stderr.is_empty() {
            eprintln!(
                "{}",
                String::from_utf8(output.stderr)
                    .expect("Could not parse script stderr output as UTF-8")
            );
        } else {
            println!(
                "{}",
                String::from_utf8(output.stdout)
                    .expect("Could not parse script stdout output as UTF-8")
            );
        }
        if !output.status.success() {
            process::exit(1);
        }
    }
}
