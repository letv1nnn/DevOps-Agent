use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Agent {
    async fn handle_input(&mut self, input: AgentInput) -> AgentResult;
}

#[async_trait]
pub trait ToolUser {
    async fn use_tool(&self, name: &str, args: &[String]) -> Result<String, Box<dyn Error>>;
}

pub struct AgentInput {
    pub message: String,
    pub context: Option<String>,
}

pub struct AgentResult {
    pub output: String,
    pub status: AgentStatus,
}

pub enum AgentStatus {
    Success,
    Error(String),
    InProgress,
}

pub struct DevOpsAgent {
    pub steps: Vec<Step>,
}

#[derive(Debug)]
pub struct Step {
    pub name: String,
    pub args: Vec<String>,
}