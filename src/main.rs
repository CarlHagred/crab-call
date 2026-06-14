use clap::Parser;
use std::fs;

mod models;

mod lexer;
use lexer::tokenize;

mod parser;
use parser::parse_requests;

mod engine;
use engine::send_request;

#[derive(Parser, Debug)]
#[command(name = "CrabCall")]
#[command(about = "A fast HTTP client and TUI", long_about = None)]
struct Cli {
    /// The .http file to execute (Optional, if left blank, interactive mode will be launched)
    file: Option<String>,

    /// The specific request index to run (Optional)
    index: Option<usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    if let Some(filepath) = cli.file {
        let raw_text = fs::read_to_string(&filepath)?;

        let my_tokens = tokenize(raw_text.trim());
        let parsed_request = parse_requests(my_tokens);

        if let Some(idx) = cli.index {
            if idx > 0 && idx <= parsed_request.len() {
                let target_req = &parsed_request[idx - 1];
                send_request(target_req);
            } else {
                println!(
                    "Error: Request index {} is out of bounds. This file has {} requests.",
                    idx,
                    parsed_request.len()
                );
            }
        } else {
            for req in &parsed_request {
                send_request(req);
            }
        }
    } else {
        println!("Coming soon...");
    }
    Ok(())
}
