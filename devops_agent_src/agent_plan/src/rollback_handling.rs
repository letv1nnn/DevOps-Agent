// need to handle rollback properly
// if there is something wrong with the released project agent needs to be able to rollback to the last version

use std::path::Path;

use tokio::process::Command;

use crate::task_types_and_workflow_steps::Task;


pub async fn get_previous_version() -> Result<Option<String>, String> {
    let path = "deployment_log.txt";
    if !Path::new(path).exists() {
        return Err("Deployment log file does not exist".to_string());
    }

    let content = tokio::fs::read_to_string(path).await.map_err(|e| e.to_string())?;
    let history: Vec<String> = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    if history .len() < 2 {
        return Err("No previous version available".to_string());
    }
    Ok(history.into_iter().rev().nth(1)) // Get the second last version
}

pub async fn rollback_to_previous_version(task: &Task) -> Result<String, String> {
    let prev_version = get_previous_version().await?;
    if let Some(previous_version) = prev_version {
        let output = Command::new("git")
            .args(&["reset", "--hard", &previous_version])
            .current_dir(&task.dir)
            .output()
            .await
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(format!("Rolled back to version: {}", previous_version))
        } else {
            Err(format!("Failed to rollback: {}", String::from_utf8_lossy(&output.stderr)))
        }
    } else {
        Err("No previous version found to rollback".to_string())
    }
}