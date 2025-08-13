use tracing::{info, error};

use crate::task_types_and_workflow_steps::Task;
use crate::secure_task_execution::run_task;

const LINE: &str = "+----------------------------------------+";

pub async fn execute_pipeline(tasks: Vec<Task>) {
    if tasks.is_empty() {
        error!("No tasks provided for execution.");
        return;
    }
    info!("\n\n{}\n|Starting pipeline execution with {} tasks|\n{}", LINE, tasks.len(), LINE);
    for task in tasks {
        info!(task = ?task.task_type, "Starting task");
        
        match run_task(&task).await {
            Ok(output) => {
                info!(task = ?task.task_type, output = %output, "Task completed");
                info!("Task completed");
            },
            Err(err) => {
                error!(task = ?task.task_type, error = %err, "Task failed");

                if task.retry_on_failure {
                    info!("Retrying task after failure");
                    match run_task(&task).await {
                        Ok(output) => {
                            info!(task = ?task.task_type, output = %output, "Retry successful");
                        },
                        Err(retry_err) => {
                            error!(task = ?task.task_type, error = %retry_err, "Retry failed");
                            info!("Halting pipeline execution due to retry failure");
                            break;
                        }
                    }
                } else {
                    error!("Halting pipeline due to failure");
                    break;
                }
            },
        }
    }
}
