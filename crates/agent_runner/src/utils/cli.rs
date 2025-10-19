use std::{
    error::Error, io::{self, Write}, path::PathBuf
};
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
    println!("{}", DEVOPS_AGENT.with(Color::Magenta).bold());

    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");    

        let command = input.trim();

        match command {
            "-l" | "--logs" => {
                let file_name = PathBuf::from("agent.log");

                let content = match read_file(file_name).await {
                        Ok(f) => f,
                        Err(e) => {
                            println!("{}: {}", "Error opening log file".with(Color::Red), e);
                            continue;
                        }
                    };
                
                println!("{}", "Agent Logs".with(Color::Blue));
                println!("{}", content);
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
    -l, --logs                      View the agent logs
    -h, --help                      See all available commands
    -q, --quit                      Quit the agent
"#;