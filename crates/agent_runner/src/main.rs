// https://github.com/letv1nnn/DevOps-Agent

use std::error::Error;
use clap::Parser;
use dotenv::dotenv;
use crate::utils::agent::start_agent;
use crate::utils::{
    cli::{
        start_cli, Mode, CLI
    }, logging::init_logging
};

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    match CLI::parse().mode {
        Mode::Agent => {
            init_logging();
            start_agent().await?;
        }
        Mode::Interaction => {
            start_cli().await?;
        }
    }

    Ok(())
}