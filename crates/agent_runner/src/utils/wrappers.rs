use std::{error::Error, path::PathBuf};
use crossterm::style::{Color, Stylize};
use llm::request_llm;
use tool_executor::process_execution::read_file;

const SYSTEM_PROMPT: &str = "You are a helpful assistant that analizes and summarizes log files to human understandable format. You need to highlight any errors or warnings found in the logs. Should not be too long, so human could read them in just 1 minute, and structure your respond with bullet points";

pub async fn analize_logs(file_path: PathBuf) -> Result<String, Box<dyn Error>> {
    let msg = format!("Analyzing the logs from {:?}", file_path);
    println!("{}", msg.with(Color::Blue));
    let prompt = read_file(file_path).await?;
    let respond = request_llm(&prompt, SYSTEM_PROMPT).await?;
    Ok(respond)
}