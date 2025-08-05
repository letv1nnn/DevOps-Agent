use tokio::process::Command;
use tracing::{error, info};
use std::{error::Error, fmt::Display};

use crate::{plan_generating::{json_config_file, send_request}, task_execution::{execute_pipeline, Task}};

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
            // gonna use Axum here, do not think I actually need this kind of interaction with the agent
        }
    }

    info!("Successfully used {} interface.", interface);
    Ok(())
}

// +--------------------------+
// | PROCESSING CLI INTERFACE |
// +--------------------------+

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
                    if command.len() < 2 {
                        println!("Missing argument for --plan / -p");
                        continue;
                    }
                    let arg = command[1].trim();
                    match arg {
                        "--gen" => {
                            // GENERATING THE PLAN BY THE GIVEN INPUT
                            println!("Enter your prompt for plan generation:");
                            println!("Generating the plan...");
                            let mut prompt = String::new();
                            std::io::stdin().read_line(&mut prompt).expect("Failed to read line");

                            let plan = send_request(&prompt).await.map_err(|e| {
                                error!("Plan generation failed!");
                                e
                            })?;

                            let _config_file_configuration = json_config_file("set_of_tasks.json", plan.as_str()).await.map_err(|e| {
                                error!("Failed to set configuration file!");
                                e
                            })?;

                            info!("Using {} file.", arg);
                            let file = std::fs::read_to_string(arg).expect("Failed to read config");
                            let tasks: Vec<Task> = serde_json::from_str(&file).expect("Invalid config");

                            let out_flag = command[2].trim();
                            cat_plan(out_flag, arg).await;

                            pipeline_execution_confirmation(tasks).await;
                        },
                        _ => {
                            if command.len() < 4 {
                                // USE EXISTED PLAN
                                println!("Using {} file...", arg);
                                info!("Using {} file.", arg);
                                let file = std::fs::read_to_string(arg).expect("Failed to read config");
                                let tasks: Vec<Task> = serde_json::from_str(&file).expect("Invalid config");

                                let out_flag = command[2].trim();
                                cat_plan(out_flag, arg).await;

                                pipeline_execution_confirmation(tasks).await;
                            } else {
                                println!("Too many arguments. You can only specify file or --gen fo plan generation.");
                                continue;
                            }
                        }
                    }
                } else if command.trim().contains("-l") || command.trim().contains("--log") {
                    let output = Command::new("cat")
                        .arg("agent.log")
                        .output()
                        .await
                        .expect("Failed to execute command");

                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        println!("\n\n+------------------------+");
                        println!("|        LOG FILE        |");
                        println!("+------------------------+\n");
                        println!("{}", stdout);
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        eprintln!("Command failed: {}", stderr);
                    }

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
            println!("\n\n+-----------------------------------------------------------+");
            println!("|  DevOps Agent - Intelligent CI/CD Workflow Orchestration  |
+-----------------------------------------------------------+\n

    Description:
    DevOps Agent is a context-aware automation tool designed to orchestrate complex CI/CD pipelines.
    It integrates with system tools, GitHub Actions, and external CI services to execute, monitor,
    and adapt CI/CD workflows. Inspired by MCP server-client models, it provides intelligent 
    coordination between delivery stages such as linting, testing, building, and deploying.

    The agent is capable of recovering from failures, escalating issues, and ensuring traceability 
    through robust logging.");
        },
        "--commands" => {
            println!("\n\n+-----------------------+");
            println!("|  Available commands:  |");
            println!("+-----------------------+\n");
            println!("
    -p, --plan <file or --gen> <+out>   Generate or load the pland --gen flag for generate a plan, file to load the already existed plan, +out flag is used to output the plan at the end
    -l, --log                           Print logs from the log file (agent.log)
    
    -s, --start                         Information about the agent
    -c, --commands                      See all available commands
    -q, --quit                          Quit the agent");
        }
        _ => (),
    }
}


pub async fn cat(path: &str) {
    let output = Command::new("cat")
        .arg(path)
        .output()
        .await
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed: {}", stderr);
    }
}

pub async fn cat_plan(out_flag: &str, arg: &str) {
    if out_flag == "+out" {
        println!("\n\n+--------+");
        println!("|  Plan  |");
        println!("+--------+\n");
        cat(arg).await;
    }
}

pub async fn pipeline_execution_confirmation(tasks: Vec<Task>) {
    println!("\n\nExecute pipeline (Y/N)?");
    let mut ans = String::new();
    std::io::stdin().read_line(&mut ans).expect("Failed to read line");
    match ans.to_lowercase().trim() {
        "y" => {
            println!("Executing pipeline...");
            execute_pipeline(tasks).await;
        },
        "n" => {
            println!("NOT Executing pipeline");
        }
        _ => println!("Unavailable respond, using 'NO'"),
        }
}
