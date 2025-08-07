use tokio::process::Command;
use std::time::Duration;
use tokio::time::timeout;

use crate::task_types_and_workflow_steps::Task;


pub async fn run_task(task: &Task) -> Result<String, String> {
    let child = Command::new(&task.command)
        .args(&task.args)
        .output();

    let output = timeout(Duration::from_secs(30), child).await;

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
