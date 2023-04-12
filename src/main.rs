use std::{env, process};
use shell_gpt_rs::{parse_cli_args, run};
use crate::config::Config;

mod config;
mod encryption;
mod errors;
mod openai;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli_args = parse_cli_args(args);
    
    let config = Config::load_config(cli_args);
    
    if let Err(e) = run(cli_args.input.as_str(), config) {
        println!("Application error: {e}");
        process::exit(1);
    }
    
    println!("{:#?}", config);

}
