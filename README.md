# HF-LLM.rs

HF-LLM.rs is a CLI tool for accessing Large Language Models (LLMs) hosted on Hugging Face. It allows you to interact with various models, provide input, and receive responses in a terminal environment.

## Features

- **Model Selection**: Choose from a variety of models available on Hugging Face.
- **Input Prompt**: Provide an input prompt to generate responses.
- **Streaming Output**: Receive responses in real-time as the model generates output.

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
cargo run --release -- -m <model-name> -p "Hello, how are you?"
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.