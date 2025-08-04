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

use agent_core::task_receiving::{interface_configuration, Interface};
use agent_core::{logging, task_execution::execute_pipeline};
use agent_core::DevOpsAgent;
use agent_core::task_execution::Task;
use agent_core::logging::CLI;
use clap::Parser;
use rmcp::{transport::stdio, ServiceExt};
// use tracing::{error};

#[tokio::main]
async fn main() {
    /*
    let args = CLI::parse();
    let interface = match &args.config.as_str() {
        "CLI" => Interface::CLI,
        "API" => Interface::API,
        "MCP" => Interface::MCP,
        _ => {
            error!("Failed to configure the agent");
            
        },
    };
    */
    logging::init_logging();
    let interface = Interface::CLI;
    let _ = interface_configuration(interface).await;

    // run_cli().await;
}

async fn _run_cli() {
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
