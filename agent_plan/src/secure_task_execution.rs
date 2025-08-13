use tokio::process::Command;
use std::time::Duration;
use tokio::time::timeout;

use crate::{deployment_processing::{self, get_current_version}, task_types_and_workflow_steps::{Task, TaskType}};


pub async fn run_task(task: &Task) -> Result<String, String> {
    let child = Command::new(&task.command)
        .args(&task.args)
        .current_dir(&task.dir)
        .output();

    let output = timeout(Duration::from_secs(30), child).await;

    // handle specific task types
    if let TaskType::Deploy = task.task_type {
        // need to handle deploy
        let version = get_current_version(&task.dir).await?;
        deployment_processing::track_deployment(&version).await?;
    }
    if let TaskType::Rollback = task.task_type {
        // need to handle rollback functionality
    }

    match output {
        Ok(Ok(out)) if out.status.success() => {
            Ok(String::from_utf8_lossy(&out.stdout).into_owned())
        },
        Ok(Ok(out)) => {
            Err(String::from_utf8_lossy(&out.stderr).into_owned())
        },
        Ok(Err(e)) => {
            Err(format!("Failed to start command: {}", e))
        },
        Err(_) => {
            Err("Task timed out".into())
        }
    }
}
