use clap::{Command, Arg};
use hf_hub::Cache;
use reqwest::Client;
use serde_json::json;
use tokio;
use futures_util::StreamExt;
use std::io::{Write, stdin};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("hf-llm.rs")
        .version("0.1.0")
        .author("VB")
        .about("A CLI to access LLMs hosted on Hugging Face")
        .arg(Arg::new("model-name")
            .short('m')
            .long("model-name")
            .value_name("MODEL")
            .help("Specify the Hugging Face Hub ID of the model to use.")
            .required(true)
            )
        .arg(Arg::new("prompt")
            .short('p')
            .long("prompt")
            .value_name("PROMPT")
            .help("Specify the prompt to use.")
            .required(false)
            )
        .arg(Arg::new("chat")
            .short('c')
            .long("chat")
            .help("Start a chat session with the model.")
            .action(clap::ArgAction::SetTrue)
            )
        .get_matches();

    let model_name = matches.get_one::<String>("model-name").unwrap();
    let chat_mode = matches.get_flag("chat");

    let cache = Cache::default();

    if let Some(token) = cache.token() {
        let url = format!("https://api-inference.huggingface.co/models/{}/v1/chat/completions", model_name);
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

                if user_input.to_lowercase() == "exit" {
                    break;
                }

                messages.push(json!({"role": "user", "content": user_input}));

                let response = send_request(&client, &url, token.clone(), model_name, &messages).await?;
                messages.push(json!({"role": "assistant", "content": response}));
            }
        } else if let Some(prompt) = matches.get_one::<String>("prompt") {
            messages.push(json!({"role": "user", "content": prompt}));
            send_request(&client, &url, token, model_name, &messages).await?;
        } else {
            println!("Please provide either a prompt or use chat mode.");
            std::process::exit(1);
        }
        
        Ok(())
    } else {
        println!("Token not found, please run `huggingface-cli login`");
        std::process::exit(1);
    }
}

async fn send_request(client: &Client, url: &str, token: String, model_name: &str, messages: &Vec<serde_json::Value>) -> Result<String, Box<dyn std::error::Error>> {
    let res = client
        .post(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": model_name,
            "messages": messages,
            "max_tokens": 2048,
            "stream": true
        }))
        .send()
        .await?;

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
                            print!("{}", content);
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