// src/main.rs
use std::env;
use std::error::Error;

use clap::Parser;
use dotenv::dotenv;
use reqwest::blocking::Client;
use std::{thread, time::Duration};
use std::io::Write;
use command_line_chatgpt::*; 

#[derive(Parser)]
#[command(name = "CLI ChatGPT")]
#[command(about = "Talk to ChatGPT from your terminal", long_about = None)]
struct Args {
    // Version gpt-4.1 is slower 
    // Version gpt-4-turbo
    // Version gpt-3.5-turbo is faster but less capable
    #[arg(short, long, default_value = "gpt-4-turbo")]
    model: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let args = Args::parse();
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = Client::new();

    check_connection(&client, &api_key)?;

    let mut messages = vec![ChatMessage {
        role: "system".to_string(),
        content: "You are a helpful assistant.".to_string(),
    }];

    println!(
        "ChatGPT Terminal — using model: {} — type 'exit' to quit",
        args.model
    );

    loop {
        let user_input = read_user_input()?;
        if user_input.eq_ignore_ascii_case("exit") {
            break;
        }

        messages.push(ChatMessage {
            role: "user".to_string(),
            content: user_input,
        });

        match send_chat_request(&client, &api_key, &args.model, &messages) {
            Ok(reply) => {
                println!("_____________________________________________");
                print!("ChatGPT: ");
                std::io::stdout().flush().unwrap(); 
                for c in reply.content.chars(){
                    print!("{}", c); 
                    std::io::stdout().flush().unwrap(); 
                    thread::sleep(Duration::from_millis(2));
                }
                println!("\n_____________________________________________");
                messages.push(reply);
            }
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(())
}
