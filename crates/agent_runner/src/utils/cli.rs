use std::{
    error::Error, io::{self, Write}, path::PathBuf
};
use agent_core::wrappers::clear_file;
use crossterm::{
    style::{Color, Stylize},
};
use clap::{
    Parser, ValueEnum
};
use tool_executor::process_execution::read_file;

#[derive(Parser)]
pub struct CLI {
    #[clap(long, value_enum)]
    pub mode: Mode,
}

#[derive(ValueEnum, Clone)]
pub enum Mode {
    Agent,
    Interaction,
}

pub async fn start_cli() -> Result<(), Box<dyn Error>> {
    println!("{}", DEVOPS_AGENT.with(Color::Rgb { r: 255, g: 70, b: 162 }).bold());

    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");    

        let command = input.trim();

        match command {
            "-al" | "--agent-logs" => {
                let file_name = PathBuf::from("logs/agent.log");

                let content = match read_file(file_name).await {
                        Ok(f) => f,
                        Err(e) => {
                            println!("{}: {}", "Error opening log file".with(Color::Red), e);
                            continue;
                        }
                    };
                
                if content.len() > 0 {
                    println!("{}", "Agent Logs".with(Color::Blue));
                    println!("{}", content);
                } else {
                    println!("{}", "Agent logs are empty".with(Color::Blue));
                }
            },
            "-wl" | "--workflow-logs" => {
                let file_name = PathBuf::from("logs/gh_workflows.log");
                
                let content = match read_file(file_name).await {
                        Ok(f) => f,
                        Err(e) => {
                            println!("{}: {}", "Error opening log file".with(Color::Red), e);
                            continue;
                        }
                    };

                if content.len() > 0 {
                    println!("{}", "Workflow Logs".with(Color::Blue));
                    println!("{}", content);
                } else {
                    println!("{}", "Workflow logs are empty".with(Color::Blue));
                }
            },
            "-cal" | "--clear-agent-logs" => {
                let file_path = PathBuf::from("logs/agent.log");
                clear_file(file_path).await;
                println!("{}", "Agent logs have been cleaned".with(Color::Blue));
            },
            "-cwl" | "--clear-workflow-logs" => {
                let file_path = PathBuf::from("logs/gh_workflows.log");
                clear_file(file_path).await;
                println!("{}", "GitHub Workflow logs have been cleaned".with(Color::Blue));
            },
            "-h" | "--help" => {
                println!("{}", "Available Commands:".with(Color::Blue));
                println!("{}", COMMANDS);
            },
            "-q" | "--quit" => {
                println!("{}", "quitting...".with(Color::Blue));
                break;
            },
            "" => continue,
            _ => {
                println!("{}", "Invalid input".with(Color::Red));
            }
        }
    }

    Ok(())
}

const DEVOPS_AGENT: &str = r#"
________              ________                    _____                         __   
\______ \   _______  _\_____  \ ______  ______   /  _  \    ____   ____   _____/  |_ 
 |    |  \_/ __ \  \/ //   |   \\____ \/  ___/  /  /_\  \  / ___\_/ __ \ /    \   __\
 |    `   \  ___/\   //    |    \  |_> >___ \  /    |    \/ /_/  >  ___/|   |  \  |  
/_______  /\___  >\_/ \_______  /   __/____  > \____|__  /\___  / \___  >___|  /__|  
        \/     \/             \/|__|       \/          \//_____/      \/     \/                                                                         
"#;

const COMMANDS: &str = r#"
    -a, --analize <file_name>      Analyze logs

    -wl, --workflow-logs           View GitHub workflows logs
    -al, --agent-logs              View the agent logs
    -cal, --clear-agent-logs       Clear agent logs
    -cwl, --clear-workflow-logs    Clear workflow logs
    
    -h, --help                     See all available commands
    -q, --quit                     Quit the agent
"#;