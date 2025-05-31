// src/lib.rs
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Deserialize)]
pub struct ChatResponse {
    pub model: String,
    pub choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
}

pub fn check_connection(client: &Client, api_key: &str) -> Result<(), Box<dyn Error>> {
    let res = client
        .get("https://api.openai.com/v1/models")
        .bearer_auth(api_key)
        .send()?;
    let response_code = res.status().as_u16();
    if response_code == 200 {
        println!("Successfully connected to OpenAI API (Http status code: {}).", response_code);
        Ok(())
    } else {
        Err(format!("Unable to connect to OpenAI API (Http status code: {}).", response_code).into())
    }
}

pub fn read_user_input() -> Result<String, Box<dyn Error>> {
    print!("- : ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn send_chat_request(
    client: &Client,
    api_key: &str,
    model: &str,
    messages: &[ChatMessage],
) -> Result<ChatMessage, Box<dyn Error>> {
    let req_body = ChatRequest {
        model: model.to_string(),
        messages: messages.to_vec(),
    };

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&req_body)
        .send()?;

    let status = res.status();

    if !status.is_success() {
        let err_text = res.text()?;
        return Err(format!("API Error {}: {}", status, err_text).into());
    }

    let res_text = res.text()?;
    let parsed: ChatResponse = serde_json::from_str(&res_text)?;
    Ok(parsed.choices[0].message.clone())
}
