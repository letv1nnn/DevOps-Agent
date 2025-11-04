use crossterm::style::Stylize;
use reqwest::Client;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};
use zip::ZipArchive;
use std::{
    env::var, error::Error, fs::create_dir_all, io::Cursor, path::PathBuf
};
use crate::github_interaction::github_structs::{
    WorkflowRunsResponse
};

pub fn get_github_env_data() -> Option<Vec<String>> {
    if let Some(token) = var("GITHUB_TOKEN").ok() &&
       let Some(owner) = var("OWNER").ok() &&
       let Some(repo) = var("REPO").ok() {
        return Some(vec![token, owner, repo]);
    }
    None
}

pub async fn list_workflow_runs(owner: &str, repo: &str, token: &str) -> Result<WorkflowRunsResponse, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/actions/runs", owner, repo);
    let client = Client::new();
    let res = client
        .get(&url)
        .header("User-Agent", "rust-agent")
        .bearer_auth(token)
        .send()
        .await?
        .json::<WorkflowRunsResponse>()
        .await?;

    Ok(res)
}

pub async fn download_workflow_logs(owner: &str, repo: &str, run_id: u64, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all("logs/workflows")?; // to store gh workflows
    
    let url = format!("https://api.github.com/repos/{}/{}/actions/runs/{}/logs", owner, repo, run_id);
    let client = Client::new();
    let bytes = client.get(&url)
        .header("User-Agent", "rust-agent")
        .bearer_auth(token)
        .send()
        .await?
        .bytes()
        .await?;

    let reader = Cursor::new(bytes);
    let mut zip = ZipArchive::new(reader)?;

    let mut workflows_logs = String::from(format!("\n\n{} {}\n", "WORKFLOW".with(crossterm::style::Color::Red), run_id));

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        println!("File: {}", file.name());
        let mut contents = String::new();
        use std::io::Read;
        file.read_to_string(&mut contents)?;
        // println!("{}", contents);
        workflows_logs.push_str(&contents);
    }

    write_workflows(workflows_logs.as_bytes()).await?;

    Ok(())
}

async fn write_workflows(workflow_logs: &[u8]) -> Result<(), Box<dyn Error>> {
    let file_name = PathBuf::from("logs/gh_workflows.log");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)
        .await?;

    file.write_all(workflow_logs).await?;

    Ok(())
}