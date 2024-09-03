use clap::{Command, Arg};
use hf_hub::Cache;
use reqwest::Client;
use serde_json::json;
use tokio;
use futures_util::StreamExt;
use std::io::Write;

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
            .required(true)
            )
        .get_matches();

    let model_name = matches.get_one::<String>("model-name").unwrap();
    let prompt = matches.get_one::<String>("prompt").unwrap();

    let cache = Cache::default();

    if let Some(token) = cache.token() {
        let url = format!("https://api-inference.huggingface.co/models/{}/v1/chat/completions", model_name);
        
        let client = Client::new();
        let res = client
           .post(&url)
           .header("Authorization", format!("Bearer {}", token))
           .header("Content-Type", "application/json")
           .json(&json!({
                "model": model_name,
                "messages": [{"role": "user", "content": prompt}],
                "max_tokens": 2048,
                "stream": true
            }))
           .send()
           .await?;
        
        let mut stream = res.bytes_stream();
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
                            }
                        }
                    }
                }
            }
        }
        println!();
        
        Ok(())
    } else {
        println!("Token not found, please run `huggingface-cli login`");
        std::process::exit(1);
    }
}