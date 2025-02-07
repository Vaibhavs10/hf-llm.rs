use anyhow::anyhow;
use clap::{Arg, Command};
use colored::Colorize;
use futures_util::StreamExt;
use hf_hub::Cache;
use reqwest::Client;
use serde_json::json;
use std::io::{stdin, Write};
use std::process::Command as ProcessCommand;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let matches = Command::new("hf-llm.rs")
        .version("0.1.0")
        .author("VB")
        .about("A CLI to access LLMs hosted on Hugging Face")
        .arg(
            Arg::new("model-name")
                .short('m')
                .long("model-name")
                .value_name("MODEL")
                .help("Specify the Hugging Face Hub ID of the model to use.")
                .required(true),
        )
        .arg(
            Arg::new("provider")
                .short('r')
                .long("provider")
                .value_name("PROVIDER")
                .help("Specify the provider to use.")
                .required(true),
        )
        .arg(
            Arg::new("prompt")
                .short('p')
                .long("prompt")
                .value_name("PROMPT")
                .help("Specify the prompt to use.")
                .required(false),
        )
        .arg(
            Arg::new("chat")
                .short('c')
                .long("chat")
                .help("Start a chat session with the model.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("max-tokens")
                .short('t')
                .long("max-tokens")
                .value_name("MAX_TOKENS")
                .help("Specify the maximum number of tokens for the model's response.")
                .default_value("2048"),
        )
        .get_matches();

    let model_name = matches.get_one::<String>("model-name").unwrap();
    let provider = matches.get_one::<String>("provider").unwrap();
    let chat_mode = matches.get_flag("chat");
    let max_tokens = matches
        .get_one::<String>("max-tokens")
        .unwrap()
        .parse::<u32>()
        .expect("Failed to parse max-tokens as a positive integer");

    let cache = Cache::default();

    if let Some(token) = cache.token() {
        let url = format!(
            "https://router.huggingface.co/{}/models/v1/chat/completions",
            provider
        );
        let client = Client::new();

        let mut messages = Vec::new();

        if chat_mode {
            println!("Starting chat mode. Type 'exit' to end the conversation.");
            loop {
                print!("You: ");
                std::io::stdout().flush()?;
                let mut user_input = String::new();
                stdin().read_line(&mut user_input)?;
                user_input = user_input.trim().to_string();

                match user_input.to_lowercase().as_str() {
                    "exit" => break,
                    "clear" => {
                        messages.clear();
                        clear_terminal();
                        println!("Chat history cleared. Starting a new conversation.");
                        continue;
                    }
                    _ => {}
                }

                messages.push(json!({"role": "user", "content": user_input}));

                let response = send_request(
                    &client,
                    &url,
                    token.clone(),
                    model_name,
                    &messages,
                    max_tokens,
                )
                .await?;
                messages.push(json!({"role": "assistant", "content": response}));
            }
        } else if let Some(prompt) = matches.get_one::<String>("prompt") {
            messages.push(json!({"role": "user", "content": prompt}));
            send_request(&client, &url, token, model_name, &messages, max_tokens).await?;
        } else {
            println!("Please provide either a prompt or use chat mode.");
            std::process::exit(1);
        }

        anyhow::Ok(())
    } else {
        println!("Token not found, please run `huggingface-cli login`");
        std::process::exit(1);
    }
}

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        ProcessCommand::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    } else {
        ProcessCommand::new("clear").status().unwrap();
    }
}

async fn send_request(
    client: &Client,
    url: &str,
    token: String,
    model_name: &str,
    messages: &Vec<serde_json::Value>,
    max_tokens: u32,
) -> anyhow::Result<String> {
    let res = client
        .post(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": model_name,
            "messages": messages,
            "max_tokens": max_tokens,
            "stream": true
        }))
        .send()
        .await?;

    let status = res.status();
    if status != 200 {
        let error_message = res.text().await?;
        return Err(anyhow!("HTTP Error {}: {}", status, error_message));
    }

    let mut stream = res.bytes_stream();
    let mut full_response = String::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        if let Ok(text) = String::from_utf8(chunk.to_vec()) {
            if text.starts_with("data: ") {
                let json_str = text.trim_start_matches("data: ");
                if json_str != "[DONE]" {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                        if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                            print!("{}", content.green());
                            std::io::stdout().flush()?;
                            full_response.push_str(content);
                        }
                    }
                }
            }
        }
    }
    println!();
    Ok(full_response)
}
