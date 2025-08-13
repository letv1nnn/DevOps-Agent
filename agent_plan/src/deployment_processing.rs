use serde_json::to_string;
use tokio::{fs::write, process::Command};

// this file is used to process deployment tasks

pub async fn get_current_version(dir: &str) -> Result<String, String> {

    let git_result = Command::new("git")
        .args(&["rev-parse", "short", "HEAD"])
        .current_dir(dir)
        .output()
        .await;

    if let Ok(output) = git_result {
        if output.status.success() {
            return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
        } else {
            return Err(format!("Git command failed: {}", String::from_utf8_lossy(&output.stderr)));
        }
    }

    // implement similar logic for other version control systems
    // definitely need to add some docker handling here
    // for now, just return a timestamp as a placeholder

    Ok(chrono::Local::now().format("%Y%m%d%H%M%S").to_string())
}

pub async fn track_deployment(version: &str) -> Result<(), String> {
    let path = "deployment_log.txt";
    let mut history: Vec<String> = if let Ok(content) = tokio::fs::read_to_string(path).await {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Vec::new()
    };
    
    history.push(version.to_string());
    write(&path, to_string(&history).map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

