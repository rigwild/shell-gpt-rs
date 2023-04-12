use crate::errors::ShellGptError;
use reqwest::blocking::Client as HttpClient;
use serde::{Deserialize, Serialize};

const PROMPT: &'static str = "I want you to generate a valid bash script following a specific request. \
                    You must only answer with the script that will be run on the target system. \
                    Do not write something like \"this is the script you asked:\", just print the script ONLY.
                    Do not write a warning message, only print the script itself.";

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

fn request_chatgpt(messages: Vec<Message>, api_key: &str) -> anyhow::Result<String> {
    let body = ChatRequestInput {
        model: "gpt-3.5-turbo".to_string(),
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

pub fn ask_chatgpt(input: &str, api_key: &str) -> anyhow::Result<String> {
    let messages = vec![Message::User(PROMPT), Message::User(input)];
    request_chatgpt(messages, api_key)
}
