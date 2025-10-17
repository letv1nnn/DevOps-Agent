use std::{env::var, io::Cursor};
use reqwest::Client;
use zip::ZipArchive;

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

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        println!("File: {}", file.name());
        let mut contents = String::new();
        use std::io::Read;
        file.read_to_string(&mut contents)?;
        println!("{}", contents);
    }

    Ok(())
}