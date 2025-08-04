use tokio::fs::File;
use tracing::{error, info};
use std::{error::Error, fmt::Display};

// AGENT NEEDS TO RECEIVE TASKS VIA CLI, REST OR MCP, SO THIS MODULE CONFIGURE EACH TYPE OF THE INTERFACE

#[derive(Debug)]
pub enum Interface {
    CLI,
    API,
    MCP,
}

impl Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Interface::CLI => write!(f, "CLI"),
            Interface::API => write!(f, "API"),
            Interface::MCP => write!(f, "MCP server"),
        }
    }
}

pub async fn interface_configuration(interface: Interface) -> Result<(), Box<dyn Error>> {
    info!("Using: {} interface.", interface);

    match interface {
        Interface::CLI => {
            let _ = cli().await.map_err(|e|{
                error!("Failed to use CLI interface");
                e
            });
        },
        Interface::MCP => {
            // rmcp library for this arm
        },
        Interface::API => {
            // gonna use Axum here
        }
    }

    info!("Successfully used {} interface.", interface);
    Ok(())
}

async fn cli() -> Result<(), Box<dyn Error>> {
    text_commands("--start");
    text_commands("--commands");
    loop {
        println!("\n\nEnter command:");
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).expect("Failed to read line!");

        match command.trim() {
            "--commands" | "-c" => text_commands("--commands"),
            "--start" | "-s" => text_commands("--start"),
            "--quit" | "-q" => {
                info!("Agent shutdown complete.");
                println!("Quitting...");
                break;
            }
            _ => {
                if command.trim().contains("-p") || command.trim().contains("--plan") {
                    let command: Vec<&str> = command.split_whitespace().collect();
                    let arg = command[1].trim();
                    match arg {
                        "--gen" => {
                            // Generating the plan by the given input.
                        },
                        _ => {
                            if command.len() < 2 {
                                let file = File::open(arg).await.map_err(|e| {
                                    error!("Failed to open file {}: {}", arg, e);
                                    e
                                })?;
                                
                            } else {
                            println!("Too many arguments. You can only specify file or --gen fo plan generation.");
                            continue;
                            }
                        }
                    }
                } else if command.trim().contains("-l") || command.trim().contains("--log") {

                } else {
                    println!("Unknown command");
                    continue; 
                }
            },
        };
    }

    Ok(())
}

fn text_commands(command: &str) {
    match command {
        "--start" => {
            println!("\n\nDevOps Agent - Intelligent CI/CD Workflow Orchestration

    Description:
    DevOps Agent is a context-aware automation tool designed to orchestrate complex CI/CD pipelines.
    It integrates with system tools, GitHub Actions, and external CI services to execute, monitor,
    and adapt CI/CD workflows. Inspired by MCP server-client models, it provides intelligent 
    coordination between delivery stages such as linting, testing, building, and deploying.

    The agent is capable of recovering from failures, escalating issues, and ensuring traceability 
    through robust logging.");
        },
        "--commands" => {
            println!("\n\nAvailable commands: ");
            println!("
    -p, --plan 'file' or --gen   Generate or load the pland -gen flag for generate a plan, file to load the already existed plan
    -l, --log <file>            Specify a custom log file (default: agent.log)
    
    -s, --start                 Information about the agent
    -c, --commands              See all available commands
    -q, --quit                  Quit the agent");
        }
        _ => (),
    }
}
