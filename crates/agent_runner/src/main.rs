// https://github.com/letv1nnn/DevOps-Agent

use clap::Parser;
use dotenv::dotenv;
use std::env::var;
use std::{
    error::Error, thread, time::Duration
};
use agent_core::{
    agent_structs::{
        AgentInput, DevOpsAgent, Step
    }, run_agent
};
use tracing::{
    info, error
};

use crate::utils::{
    cli::{
        start_cli, Mode, CLI
    }, get_env::get_pipeline, logging::init_logging
};

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    match CLI::parse().mode {
        Mode::Agent => {
            init_logging();
            info!("Agent has been started");
    
            let steps: Vec<Step> = match get_pipeline() {
                Some(pipeline) => pipeline,
                None => {
                    error!("No PIPELINE environment variable found");
                    return Err("No PIPELINE environment variable found".into());
                }
            };

            let timeout_hour = match var("TIMEOUT_HOUR") {
                Ok(val) => val.parse::<u64>().unwrap_or(2),
                Err(_) => 2,
            };

            let mut agent = DevOpsAgent::new(steps);
            
            loop {
                let input = AgentInput {
                    message: String::from("Executing the planned steps."),
                    context: None,
                };
                run_agent(&mut agent, input).await;

                thread::sleep(Duration::from_secs(timeout_hour * 3600));
            }
        }
        Mode::Interaction => {
            start_cli().await?;
        }
    }

    Ok(())
}