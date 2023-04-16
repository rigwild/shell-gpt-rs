use crate::errors::ShellGptError;
use regex::Regex;
use reqwest::blocking::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::env;

const OPENAI_MODEL: &'static str = "gpt-3.5-turbo";

const PRE_PROMPT_SHELL_SCRIPT: &'static str = "You are an expert at creating bash scripts. \
                    I want you to generate a valid bash script following a specific request. \
                    You must only answer with the script that will be run on the target system. \
                    Do not write something like \"this is the script you asked:\", just print the script ONLY.
                    Do not write a warning message, only print the script itself.";

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PrePrompt {
    NoPrePrompt,
    ShellScript,
}

pub fn ask_chatgpt(input: &str, pre_prompt: PrePrompt, api_key: &str) -> anyhow::Result<String> {
    if let Ok(response) = env::var("OPENAI_API_RESPONSE_MOCK") {
        return Ok(response);
    }
    let pre_prompt = get_pre_prompt(pre_prompt);
    let messages = vec![Message::User(pre_prompt.as_str()), Message::User(input)];
    request_chatgpt_api(messages, api_key)
}

/// Even with a pre-prompt indicating to not use code blocks and not give explanations,
/// the model can output some. Extract it, if no code block found, just return the input string.
pub fn extract_code_block_if_needed(str: &str) -> String {
    let regex = Regex::new(r"```\w?\n([\s\S]*?)\n```").unwrap();
    match regex.captures(str) {
        Some(captures) if captures.len() > 0 => captures.get(0).unwrap().as_str().to_string(),
        _ => str.to_string(),
    }
}

fn get_pre_prompt(pre_prompt: PrePrompt) -> String {
    match pre_prompt {
        PrePrompt::NoPrePrompt => String::new(),
        PrePrompt::ShellScript => PRE_PROMPT_SHELL_SCRIPT.to_string(),
    }
}

fn request_chatgpt_api(messages: Vec<Message>, api_key: &str) -> anyhow::Result<String> {
    let body = ChatRequestInput {
        model: OPENAI_MODEL.to_string(),
        messages,
    };

    let client = HttpClient::new();
    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&body)
        .send()?;

    if resp.status().is_success() {
        let res: ChatResponse = resp.json()?;
        Ok(res.choices.get(0).unwrap().message.content.clone())
    } else {
        let err = format!(
            "Error when calling the OpenAI chat completion API - Status: {} - Body: {}",
            resp.status(),
            resp.text().unwrap()
        );
        Err(ShellGptError::ApiError(err))?
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "role", content = "content", rename_all = "lowercase")]
pub enum Message<'a> {
    System(&'a str),
    Assistant(&'a str),
    User(&'a str),
}

#[derive(Debug, Serialize)]
struct ChatRequestInput<'a> {
    model: String,
    messages: Vec<Message<'a>>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ChatResponseUsage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ChatResponseMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ChatResponseChoice {
    pub index: i64,
    pub message: ChatResponseMessage,
    pub finish_reason: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub choices: Vec<ChatResponseChoice>,
    pub usage: ChatResponseUsage,
}
