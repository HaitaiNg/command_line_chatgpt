# Command Line Interface ChatGPT

A simple command-line interface to interact with OpenAI's ChatGPT API directly from your terminal.  
Built with Rust using `reqwest` for HTTP requests, `clap` for CLI argument parsing, and `serde` for JSON serialization.

![Demo](media/demo.gif)

---

## Features

- Supports interactive chat with ChatGPT models (default: `gpt-3.5-turbo`)
- Gracefully handles API errors and connection issues
- Easily switch models via command line flags
- Maintains conversation context during a session

---

## Prerequisites

- Rust toolchain installed ([rustup](https://rustup.rs/))
- An OpenAI API key. Sign up at [OpenAI](https://platform.openai.com/signup) and get your API key.
- Create a `.env` file in the project root with your API key:

```env
OPENAI_API_KEY=your_api_key_here
```
## Installation 
```
git clone git@github.com:HaitaiNg/rust_.git
cd command_line_chatgpt
cargo build --release
```

## Usage
Run the CLI with default model:
```
cargo run --release
```

Once running:
- Type your messages and press Enter.
- Type exit to quit the chat.

## Testing 
Run all tests with 
```
cargo test
```
You can run specific files or modules using:
```
cargo test --test api_tests 
```

## Future enhancements
- Stream ChatGPT responses character-by-character.
Currently, the full response is printed after processing. Updating it to stream each character (or token) as it's received would make the interaction feel more natural and responsive, similar to the official ChatGPT interface.
- Support more OpenAI models dynamically. Allow users to configure and experiment with other models beyond gpt-3.5-turbo (e.g., gpt-4, gpt-4-turbo, or future releases), either via CLI flags or a config file. 
- Add optional user interfaces. While the CLI is great for developers, others may prefer a GUI or web-based interface. Building a simple web frontend or TUI (text-based UI) could broaden adoption.
