use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::{process::Command, time::timeout};
use tracing::{info, error};

// DEFINING TASK TYPES AND WORKFLOWS STEPS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Lint,
    Test,
    Build,
    Deploy,
    Rollback,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Task {
    pub task_type: TaskType,
    pub command: String,
    pub args: Vec<String>,
    pub retry_on_failure: bool,
}

// SECURE TASK EXECUTION
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


// EXECUTING A TASK PIPELINE
pub async fn execute_pipeline(tasks: Vec<Task>) {
    for task in tasks {
        info!(task = ?task.task_type, "Starting task");
        
        match run_task(&task).await {
            Ok(output) => {
                info!(task = ?task.task_type, output = %output, "Task completed");
            },
            Err(err) => {
                error!(task = ?task.task_type, error = %err, "Task failed");

                if task.retry_on_failure {
                    info!("Retrying task after failure");
                    let _ = run_task(&task).await;
                } else {
                    error!("Halting pipeline due to failure");
                    break;
                }
            },
        }
    }
}