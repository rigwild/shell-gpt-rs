use shell_gpt_rs::config::Config;
use shell_gpt_rs::{parse_cli_args, run};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli_args = parse_cli_args(args);

    let config = Config::load_config(&cli_args);

    if let Err(e) = run(&cli_args.input.as_str(), &config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
