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

extern crate agent_core;

use agent_core::{logging, task_execution::execute_pipeline};
use agent_core::DevOpsAgent;
use agent_core::task_execution::Task;
use agent_core::logging::CLI;
use clap::Parser;
use rmcp::{transport::stdio, ServiceExt};

#[tokio::main]
async fn main() {
    logging::init_logging();
    run_cli().await;
}

async fn run_cli() {
    let args = CLI::parse();
    let file = std::fs::read_to_string(&args.config).expect("Failed to read config");
    let tasks: Vec<Task> = serde_json::from_str(&file).expect("Invalid config");

    execute_pipeline(tasks).await;
}

pub async fn run_mcp_server() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Running the program...");
    
    let server = DevOpsAgent::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            eprintln!("{e}");
        });
    
    server?.waiting().await?;
    
    Ok(())
}
