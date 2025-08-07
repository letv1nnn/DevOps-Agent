use tracing::{info, error};

use crate::task_types_and_workflow_steps::Task;
use crate::secure_task_execution::run_task;

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
