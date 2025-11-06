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
    error::Error, fs::OpenOptions, path::PathBuf
};

const SYSTEM_PROMPT: &str = "You are a helpful assistant that analizes and summarizes log files to human understandable format. You need to highlight any errors or warnings found in the logs. Should not be too long, so human could read them in just 1 minute, and structure your respond with bullet points";

pub async fn download_workflows_logs() -> Result<String, Box<dyn Error>> {
    match get_github_env_data() {
        Some(data) => {
            info!("Using tool 'download_workflows_logs' to download GitHub workflow logs");

            let (token, owner, repo) = (&data[0], &data[1], &data[2]);
            let response = list_workflow_runs(owner, repo, &token).await?;
                        
            for workflow_run in &response.workflow_runs {
                download_workflow_logs(owner, repo, workflow_run.id, &token).await?;
                info!("Downloaded logs for workflow run ID: {}", workflow_run.id);
            }
            let workflows_ids = response.workflow_runs.iter().map(|wr| wr.id).collect::<Vec<u64>>();
            info!("Downloaded logs for workflow run IDs: {:?}", workflows_ids);

            Ok(format!("Downloaded logs for workflow run IDs: {:?}", workflows_ids))
        }
        None => {
            error!("One of github environment variables is not found in environment variables");
            Err("One of github environment variables is not found in environment variables".into())
        }
    }
}

pub async fn list_workflows() -> Result<String, Box<dyn Error>> {
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
}

pub async fn analize_agent_logs() -> Result<String, Box<dyn Error>> {
    info!("Using tool 'analize_agent_logs' to analize agent log file");

    let file_name = PathBuf::from("logs/agent.log");
    let prompt = read_file(file_name).await?;

    let respond = request_llm(&prompt, SYSTEM_PROMPT).await?;

    Ok(respond)
}

pub async fn analize_gh_workflows_logs() -> Result<String, Box<dyn Error>> {
    info!("Using tool 'analize_gh_workflows_logs' to analize gh workflows logs");
    let file_path = PathBuf::from("logs/gh_workflows.log");
    let prompt = read_file(file_path).await?;

    let respond = request_llm(&prompt, SYSTEM_PROMPT).await?;

    Ok(respond)
}

pub async fn clear_file(path: PathBuf) {
    let _ = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path);
}