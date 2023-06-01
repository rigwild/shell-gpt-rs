use crate::errors::ShellGptError;
use crate::{encryption, openai};
use anyhow::{anyhow, Context};
use directories::ProjectDirs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::Duration;
use std::{env, fs, io};

#[derive(Debug)]
pub struct CliArgs {
    pub input: String,
    pub show_help: bool,
    pub clear_saved_config: bool,
    pub pre_prompt: openai::PrePrompt,
    /// Just print the answer - No spinner, pretty-print, or interactive action
    pub raw: bool,
    pub timeout: Option<Duration>,
}

#[derive(Debug)]
pub struct Config {
    pub openai_api_key: String,
}

impl Config {
    pub fn load_config(_cli_args: &CliArgs) -> Config {
        let openai_api_key = load_api_key()
            .unwrap_or_else(|_| register_api_key().unwrap_or_else(|e| panic!("{:#?}", e)));
        Config { openai_api_key }
    }

    pub fn clear_saved_config() {
        let path = get_config_dir_path();
        fs::remove_dir_all(path).unwrap_or_else(|e| panic!("{:#?}", e));
        println!("Local configuration was cleared!");
    }

    pub fn parse_cli_args(args: Vec<String>) -> CliArgs {
        let args: Vec<String> = args.iter().skip(1).cloned().collect();

        let input = args.join(" ");
        let mut show_help = false;
        let mut clear_saved_config = false;
        let mut pre_prompt = openai::PrePrompt::NoPrePrompt;
        let mut raw = false;
        let mut timeout = None;

        let mut iter = args.iter();
        while let Some(x) = iter.next() {
            match x.trim() {
                "--help" | "-h" => show_help = true,
                "--shell" | "--bash" | "--script" | "-s" => pre_prompt = openai::PrePrompt::ShellScript,
                "--remove-config" | "--delete-config" | "--clear-config" => clear_saved_config = true,
                "--raw" => raw = true,
                "--timeout" => if let Some(arg) = iter.next() {
                    let seconds = arg.trim().parse::<u64>().expect("Invalid timeout value");
                    timeout = Some(Duration::from_secs(seconds));
                },
                _ => {}
            };
        }

        CliArgs {
            input,
            show_help,
            clear_saved_config,
            pre_prompt,
            raw,
            timeout,
        }
    }
}

fn load_api_key() -> anyhow::Result<String> {
    if let Ok(api_key) = read_api_key_from_env_var() {
        return Ok(api_key);
    }

    let config_path = get_config_openai_api_key_path();
    let mut file =
        File::open(&config_path).with_context(|| format!("Failed to open file {config_path:?}"))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .with_context(|| format!("Failed to read file {config_path:?}"))?;

    let api_key_encrypted = encryption::decrypt(buffer, get_config_encryption_password())?;

    let api_key = String::from_utf8(api_key_encrypted)
        .with_context(|| "Failed to convert decrypted API key to UTF-8")?;
    key_not_empty(api_key)
}

fn save_api_key(api_key: &str) -> anyhow::Result<()> {
    let config_path = get_config_openai_api_key_path();
    println!("config_path = {config_path:?}");

    // Basically `mkdir -p /home/<user>/.config/shell-gpt-rs`
    let prefix = config_path.parent().unwrap();
    fs::create_dir_all(prefix)?;

    // Write encrypted API key
    let mut file = File::create(&config_path)?;
    let api_key_encrypted =
        encryption::encrypt(api_key.as_bytes(), get_config_encryption_password())?;

    file.write_all(&*api_key_encrypted)
        .with_context(|| format!("Could not save the API key to {config_path:?}"))
}

pub fn register_api_key() -> anyhow::Result<String> {
    println!("{}", get_config_dir_path().to_str().unwrap().to_string());
    println!("You need to enter an OpenAI API key");
    println!();
    println!("You can create a new API key at https://platform.openai.com/account/api-keys");
    println!("Your key will be encrypted and saved locally for future use");
    println!();
    println!("If you don't want your key to be saved, you can pass it using the `OPENAI_KEY` environment variable");
    println!();
    println!();
    println!("Enter your OpenAI API key: ");

    let mut api_key = String::new();

    io::stdin()
        .read_line(&mut api_key)
        .context("Failed to read your input")?;

    match api_key.trim() {
        api_key if !api_key.is_empty() => {
            save_api_key(&api_key).context("Failed saving the API key")?;
            // save_api_key(&api_key).context("Failed saving the API key")?;
            Ok(api_key.to_string())
        }
        e => Err(anyhow!("Received an empty API key - {}", e)),
    }
}

fn read_api_key_from_env_var() -> anyhow::Result<String> {
    match env::var("OPENAI_KEY") {
        Ok(key) => key_not_empty(key),
        Err(_) => Err(ShellGptError::ApiKeyMissing.into()),
    }
}

fn key_not_empty(key: String) -> anyhow::Result<String> {
    if !key.trim().is_empty() {
        Ok(key.trim().to_string())
    } else {
        Err(ShellGptError::ApiKeyEmpty.into())
    }
}

fn get_config_dir_path<'a>() -> PathBuf {
    ProjectDirs::from("com", "shell-gpt-rs", "shell-gpt-rs")
        .unwrap()
        .config_dir()
        .to_path_buf()
}

fn get_config_openai_api_key_path() -> PathBuf {
    get_config_dir_path().join("openai_api_key.encrypted.txt")
}

/// Generate the password used to encrypt the configuration file with.
///
/// Password is `username_ENCRYPTION_PASSWORD_SUFFIX`
fn get_config_encryption_password() -> String {
    const ENCRYPTION_PASSWORD_SUFFIX: &'static str =
        "shell-gpt-rs-EL9Kaesj7Q6pc9BzsfxVpjPbNnuj8bGJ";
    format!(
        "{}_{}",
        whoami::username(),
        ENCRYPTION_PASSWORD_SUFFIX
    )
}
