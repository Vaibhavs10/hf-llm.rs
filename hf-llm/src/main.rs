use clap::{Command, Arg};

fn main() {
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

    println!("Model name: {}", model_name);
    println!("Prompt: {}", prompt);
}