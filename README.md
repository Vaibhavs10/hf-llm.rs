# HF-LLM.rs 🦀

HF-LLM.rs is a CLI tool for accessing Large Language Models (LLMs) like Llama 3.1, Mistral, Gemma 2, Cohere and much more hosted on Hugging Face. It allows you to interact with various models, provide input, and receive responses in a terminal environment.

Also, you can find the list of models supported by this CLI [here](https://huggingface.co/models?inference=warm&pipeline_tag=text-generation&sort=trending)—the list keeps getting updated as new models are released on the Hugging Face Hub. You might require a [Pro subscription](https://huggingface.co/subscribe/pro) to access some models.

## Features

- **Model Selection**: Choose from a variety of models available & deployed on Hugging Face infrastructure.
- **Input Prompt**: Provide an input prompt to generate responses.
- **Streaming Output**: Receive responses in real-time as the model generates output.
- **Chat Mode**: Start an interactive chat session with the LLM.

## Installation

1. **Clone the repository:**
   ```
   git clone https://github.com/vaibhavs10/hf-llm.rs.git
   ```

2. **Navigate to the project directory:**
   ```
   cd hf-llm.rs
   ```

3. **Build the project:**
   ```
   cargo build --release
   ```

4. **Verify the installation:**
   ```
   cargo run --release -- --help
   ```

## Usage

To use HF-LLM.rs, follow these steps:

```
cargo run --release -- -m "meta-llama/Meta-Llama-3.1-70B-Instruct" -p "How to make a dangerously spicy ramen?"
```

You can also use the chat mode to start an interactive chat session with the LLM.

```
cargo run --release -- -m "meta-llama/Meta-Llama-3.1-70B-Instruct" -c
```

Note: Make sure to run `huggingface-cli login` to login to your Hugging Face account to access some of the models.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.