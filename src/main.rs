use shell_gpt_rs::config::Config;
use shell_gpt_rs::{run};
use std::{env, process};

fn main() {
    let cli_args = Config::parse_cli_args(env::args().collect());

    if cli_args.show_help {
        println!("
Ask ChatGPT for a shell script, code, or anything, directly from your terminal 🤖🧠👨‍💻

Usage: gpt <your_question>

Options:
  -s  --shell           Ask ChatGPT for a shell script                                                  [boolean]
      --clear-config    Remove local config, including the OpenAI API key at `~/.config/shell-gpt-rs`   [boolean]
  -h, --help            Show help                                                                       [boolean]

Examples:
  gpt is the earth flat?
  gpt --shell show the list of files in the current directory with human-readable file size
  gpt -s find the top 10 biggest files in the current directory
  gpt -s find the top 10 biggest files in the current tree recursive

shell-gpt-rs - https://github.com/rigwild/shell-gpt-rs
");
        process::exit(0);
    }

    if cli_args.clear_saved_config {
        Config::clear_saved_config();
        process::exit(0);
    }

    let config = Config::load_config(&cli_args);

    if let Err(e) = run(&cli_args, &config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
