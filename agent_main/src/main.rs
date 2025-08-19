//  +-------------------------------------------------------+
//  |  ____    _______ __     __  ______  _____    _______  |
//  |  ||_\\   ||____| ||     || //____\\ ||___\  //_____|  |
//  |  ||  \\  ||      ||     || ||    || ||   \\ ||        |
//  |  ||   || ||____  ||     || ||    || ||___// ||_____   |
//  |  ||   || ||____|  \\    // ||    || ||__//  \\____\\  |
//  |  ||   // ||        ||  ||  ||    || ||            ||  |
//  |  ||_ //  ||____    ||  ||  ||____|| ||       _____||  |
//  |  ||_//   ||____|    \\//   \\____// ||      |_____//  |
//  +-------------------------------------------------------+
//
// DevOps MCP based Agent. 
// Read full documentation on the github (https://github.com/letv1nnn/DevOps-Agent).

use clap::Parser;

use crate::logging::init_logging;
use crate::cli::cli_agent_interaction;
use crate::mcp::run_server;

mod logging;
mod cli;
mod mcp;

#[tokio::main]
async fn main() {
    init_logging();

    let args = Cli::parse();

    match args.interface.trim().to_lowercase().as_str() {
        "cli" => {
            // I was thinking to implement a Tauri APP and integrate some parts from the CLI into it
            println!("Running CLI agent...");
            let _ = cli_agent_interaction().await.map_err(|e| {
                println!("FAILED to interact with CLI: {e}");
            });
        },
        "mcp" => {
            if let Ok(_) = run_server().await {
                println!("MCP server started successfully.");
            } else {
                println!("Failed to start MCP server.");
            }
        },
        "api" => {
            // I do not know about this one yet, but it is planned
            println!("Running API server...");
        },
        _ => {
            println!("User can only use CLI based agent, MCP server or API calls");
        },
    }
}

#[derive(Parser)]
pub struct Cli {
    #[arg(long)]
    interface: String,
}
