use std::error::Error;
use async_trait::async_trait;
use tracing::{
    error, info
};
use crate::{agent_structs::{
    Agent, AgentInput, AgentResult, AgentStatus, DevOpsAgent, Step, ToolUser
}, wrappers::{analize_agent_logs, analize_gh_workflows_logs, download_workflows_logs, list_workflows}};

pub mod agent_structs;
pub mod wrappers;

impl DevOpsAgent {
    pub fn new(steps: Vec<Step>) -> Self {
        DevOpsAgent {
            steps,
        }
    }
}

#[async_trait]
impl Agent for DevOpsAgent {
    async fn handle_input(&mut self, _input: AgentInput) -> AgentResult {
        if self.steps.is_empty() {
            return AgentResult {
                output: "No viable plan".into(),
                status: AgentStatus::Error("Planning failed".into())
            };
        }

        for step in &self.steps {
            match self.use_tool(&step.name, &step.args).await {
                Ok(output) => {
                    info!("Step '{}' executed successfully with output: {}", step.name, output);
                }
                Err(e) => {
                    error!("Error executing step '{}': {}", step.name, e);
                    return AgentResult {
                        output: format!("Error executing step '{}': {}", step.name, e),
                        status: AgentStatus::Error(format!("Step '{}' failed", step.name))
                    };
                }
            };
        }

        AgentResult {
            output: format!("Executed {} steps: {:?}", self.steps.len(), self.steps),
            status: AgentStatus::Success,
        }
    }
}

#[async_trait]
impl ToolUser for DevOpsAgent {
    async fn use_tool(&self, name: &str, _args: &[String]) -> Result<String, Box<dyn Error>> {
        match name {
            "download_workflows_logs" => download_workflows_logs().await,
            "list_workflows" => list_workflows().await,
            "analize_agent_logs" => analize_agent_logs().await,
            "analize_gh_workflows_logs" => analize_gh_workflows_logs().await,
            "notify" => {
                info!("Using tool 'notify' to send notification");
                return Ok("Given pipeline has been executed.".into());
            }
            _ => {
                error!("Tool '{}' not recognized", name);
                Err(format!("Tool '{}' not recognized", name).into())
            }
        }
    }
}

pub async fn run_agent<T>(agent: &mut T, input: AgentInput) 
where
    T: Agent + Send + ToolUser {
        info!("Running agent with input: {}", input.message);
        
        let result = agent.handle_input(input).await;

        if let AgentStatus::Success = result.status {
            let output = agent.use_tool("notify", &[result.output.clone()]).await;
            info!("Notification sent with output: {:?}", output);
        }
}
