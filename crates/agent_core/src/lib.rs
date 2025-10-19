use async_trait::async_trait;
use llm::request_llm;
use tool_executor::{
    github_interaction::github_api_client::{
        download_workflow_logs, get_github_env_data, list_workflow_runs
    }, process_execution::read_file
};
use tracing::{
    error, info
};
use std::{
    error::Error, path::PathBuf
};
use crate::agent_structs::{
    Agent, AgentInput, AgentResult, AgentStatus, DevOpsAgent, Step, ToolUser
};

pub mod agent_structs;

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
            "download_workflows_logs" => {
                match get_github_env_data() {
                    Some(data) => {
                        info!("Using tool 'download_workflows_logs' to download GitHub workflow logs");

                        let (token, owner, repo) = (&data[0], &data[1], &data[2]);
                        let response = list_workflow_runs(owner, repo, &token).await?;
                        
                        if let Some(first_run) = response.workflow_runs.first() {
                            download_workflow_logs(owner, repo, first_run.id, &token).await?;
                            info!("Downloaded logs for workflow run ID: {}", first_run.id);
                            Ok(format!("Downloaded logs for workflow run ID: {}", first_run.id))
                        } else {
                            error!("No workflow runs found");
                            Err("No workflow runs found".into())
                        }
                    }
                    None => {
                        error!("One of github environment variables is not found in environment variables");
                        Err("One of github environment variables is not found in environment variables".into())
                    }
                }
            }
            "list_workflows" => {
                match get_github_env_data() {
                    Some(data) => {
                        info!("Using tool 'list_workflows' to get GitHub workflow runs");

                        let (token, owner, repo) = (&data[0], &data[1], &data[2]);
                        let response = list_workflow_runs(owner, repo, &token).await?;
                        
                        let mut output = String::new();
                        for run in &response.workflow_runs {
                            output.push_str(&format!("ID: {}, Status: {}, Conclusion: {:?}\n", run.id, run.status, run.conclusion));
                        }

                        info!("Retrieved {} workflow runs", response.workflow_runs.len());
                        
                        Ok(output)
                    }
                    None => {
                        error!("One of github environment variables is not found in environment variables");
                        Err("One of github environment variables is not found in environment variables".into())
                    }
                }
            },
            "analize_agent_logs" => {
                info!("Using tool 'analize_logs' analize agent log file");

                let file_name = PathBuf::from("agent.log");
                let prompt = read_file(file_name).await?;

                let system_prompt = String::from(
                    "You are a helpful assistant that analizes and summarizes log files to human understandable format. You need to highlight any errors or warnings found in the logs."
                );

                let respond = request_llm(&prompt, &system_prompt).await?;

                Ok(respond)
            }
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