use shell_gpt_rs::config::{Config};
use shell_gpt_rs::{parse_cli_args, run};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli_args = parse_cli_args(args);

    if cli_args.show_help {
        println!("
Ask ChatGPT to generate a shell script for you.
Usage: gpt <your_question>

Options:
      --clear-config    Remove local config, including the OpenAI API key at `~/.config/shell-gpt-rs`   [boolean]
  -h, --help            Show help                                                                       [boolean]

Examples:
  gpt show the list of files in the current directory with human-readable file size
  gpt find the top 10 biggest files in the current directory
  gpt find the top 10 biggest files in the current tree recursive

shell-gpt-rs - https://github.com/rigwild/shell-gpt-rs
        ");
        process::exit(0);
    }

    if cli_args.show_help {
        match Config::clear_saved_config() {
            Ok(_) =>
            println!("Locally saved configurations were cleared!"),
                Err(_) => {}
        }
        process::exit(0);
    }


    let config = Config::load_config(&cli_args);

    if let Err(e) = run(&cli_args.input.as_str(), &config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
