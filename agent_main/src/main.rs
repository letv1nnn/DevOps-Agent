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

mod logging;
mod cli;
mod mcp;

#[tokio::main]
async fn main() {
    init_logging();

    let args = Cli::parse();

    match args.interface.trim().to_lowercase().as_str() {
        "cli" => {
            let _ = cli_agent_interaction().await.map_err(|e| {
                println!("FAILED to interact with CLI: {e}");
            });
        },
        "mcp" => {},
        "api" => {},
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
