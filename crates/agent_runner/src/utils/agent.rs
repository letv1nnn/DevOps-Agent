use std::{
    env::var, error::Error, thread, time::Duration
};
use agent_core::{
    agent_structs::{
        AgentInput, DevOpsAgent, Step
    }, run_agent
};
use tracing::{
    error, info, warn
};
use crate::utils::get_env::get_pipeline;


pub async fn start_agent() -> Result<(), Box<dyn Error>> {

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
        Err(_) => {
            warn!("TIMEOUT_HOUR not set, defaulting to 2 hours");
            2
        },
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