use std::error::Error;

use agent_plan::{llm_plan_integration::{send_request, write_prompt_to_json_file}, task_pipeline_execution::execute_pipeline, task_types_and_workflow_steps::Task};
use tokio::{process::Command, time::Instant};

const AGENT_INFO: &str = "\n
+-----------------------------------------------------------+
|  DevOps Agent - Intelligent CI/CD Workflow Orchestration  |
+-----------------------------------------------------------+

    Description:
    DevOps Agent is a context-aware automation tool designed to orchestrate complex CI/CD pipelines.
    It integrates with system tools, GitHub Actions, and external CI services to execute, monitor,
    and adapt CI/CD workflows. Inspired by MCP server-client models, it provides intelligent 
    coordination between delivery stages such as linting, testing, building, and deploying.

    The agent is capable of recovering from failures, escalating issues, and ensuring traceability 
    through robust logging.";

const AGENT_CLI_COMMANDS: &str = "\n
+----------------------+
|  Available commands  |
+----------------------+
    
    --execute 'plan file'           Execute the pipeline (plan must be .json)
    --generate                      Generate plan via llm
    -p, --plan 'plan file'          Print plan
    -l, --log                       Print logs from the log file (agent.log)
    -s, --shell 'command' 'args'    Execute shell command
    
    -i, --info                      Information about the agent
    -c, --commands                  See all available commands
    -q, --quit                      Quit the agent";

const AGENT_LOG: &str = "\n
+---------+
|   Logs  |
+---------+
";

const AGENT_PLAN_EXE: &str = "\n
+--------------------+
|   Plan Execution   | 
+--------------------+
";

const AGENT_PLAN: &str = "\n
+----------+
|   Plan   |
+----------+
";

const AGENT_PLAN_GENERATION: &str = "\n
+-------------------+
|  Plan Generation  | 
+-------------------+
";

const AGENT_SHELL_COMMAND_INVOCATION: &str = "\n
+----------------------------+
|  Shell Command Invocation  | 
+----------------------------+
";

pub async fn cli_agent_interaction() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("{}", AGENT_INFO);
    println!("{}", AGENT_CLI_COMMANDS);
    loop {
        let user_input = user_input();
        let user_command: Vec<&str> = user_input.split_whitespace().collect();
        match user_command[0] {
            "-i" | "--info" => println!("{}", AGENT_INFO),
            "-q" | "--quit" => {
                println!("Quitting...");
                break;
            },    
            "-c" | "--commands" => println!("{}", AGENT_CLI_COMMANDS),
            "-l" | "--log" => {
                match shell_cat("agent.log").await {
                    Ok(logs) => println!("{}\n{}", AGENT_LOG, logs),
                    Err(e) => println!("ERROR: {}", e),
                };
            },
            "--generate" => {
                // Generate plan via llm
                println!("{}\nEnter a prompt:", AGENT_PLAN_GENERATION);
                let mut prompt = String::new();
                std::io::stdin().read_line(&mut prompt).expect("Failed to read line!");
                
                println!("Plan generation started in background...");
                println!("The message will pop up when the plan will be ready!");

                let prompt_clone = prompt.clone();
                let start: Instant = Instant::now();

                let handle = tokio::spawn(async move {
                    
                    match send_request(prompt_clone.as_str()).await {
                        Ok(resp) => {
                            if let Err(e) = write_prompt_to_json_file("tasks.json", resp.as_str()).await {
                                eprintln!("Error writing tasks.json: {}", e);
                            } else {
                                println!("\nGenerated plan is saved to tasks.json.");
                            }
                        }
                        Err(e) => eprintln!("LLM error: {}", e),
                    };
                    
                });
                
                match handle.await {
                    Ok(_) => {
                        let duration = start.elapsed();
                        println!("Generated plan is moved to tasks.json file for future execution.");
                        println!("Plan generation took {:?}", duration);
                    },
                    Err(e) => {
                        eprintln!("Task failed: {}", e);
                    }
                }
            },
            _ => {
                if user_command.len() == 2 && user_command[0] == "--execute" {
                    read_and_exec_plan(user_command[1]).await;
                } else if user_command.len() == 2 && (user_command[0] == "-p" || user_command[0] == "--plan") {
                    let path = std::path::Path::new(user_command[1]);

                    match tokio::fs::metadata(path).await {
                        Ok(_) => {
                            match shell_cat(user_command[1]).await {
                                Ok(logs) => println!("{}\n{}", AGENT_PLAN, logs),
                                Err(e) => println!("ERROR: {}", e),
                            };
                        },
                        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                            println!("File does not exist.");
                        }
                        Err(err) => {
                            println!("Error checking file: {}", err);
                        }
                    }
                } else if user_command.len() >= 2 && (user_command[0] == "-s" || user_command[0] == "--shell") {
                    let command = user_command[1];
                    if user_command.len() > 2 {
                        let args = &user_command[2..];
                        if let Ok(output) = execute_shell_command(command.to_string(), args.iter().map(|s| s.to_string()).collect()).await {
                            println!("{}\n{}", AGENT_SHELL_COMMAND_INVOCATION, output);
                        } else {
                            println!("{}\nShell command invocation failed!", AGENT_SHELL_COMMAND_INVOCATION);
                        }
                    } else {
                        if let Ok(output) = execute_shell_command(command.to_string(), vec![]).await {
                            println!("{}\n{}", AGENT_SHELL_COMMAND_INVOCATION, output);
                        } else {
                            println!("{}\nShell command invocation failed!", AGENT_SHELL_COMMAND_INVOCATION);
                        }
                    } 
                } else {
                    println!("Invalid command");
                    continue;
                }
            },
        }
    }    

    Ok(())
}    


// helper functions
fn user_input() -> String {
    println!("\nEnter command:");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input.to_lowercase()
}    

async fn shell_cat(path: &str) -> Result<String, String>{ 
    match Command::new("cat")
        .arg(&path)
        .output()
        .await {
        Ok(output) => Ok(String::from_utf8_lossy(&output.stdout).into_owned()),
        Err(e) => Err(e.to_string())
    }
}

// need to add the project specification, mandatory!
async fn read_and_exec_plan(path: &str) {
    if let Ok(file) = std::fs::read_to_string(path) {
        println!("{}\nExecuting plan...It might take some time.", AGENT_PLAN_EXE);
        let start = Instant::now();
        let tasks: Vec<Task> = serde_json::from_str(&file).expect("Invalid config");
        execute_pipeline(tasks).await;
        let duration = start.elapsed();
        println!("The given plan has been executed.\nExecution took {:?}.", duration);
        println!("You might want to review logs, enter -l or --log command to see logs.");
    } else {
        println!("Incorrect file path!");
    };
}    

async fn execute_shell_command(command: String, args: Vec<String>) -> Result<String, String> {
    match Command::new(&command)
        .args(&args)
        .output()
        .await {
            Ok(output) => Ok(String::from_utf8_lossy(&output.stdout).into_owned()),
            Err(e) => Err(e.to_string())
        }
}