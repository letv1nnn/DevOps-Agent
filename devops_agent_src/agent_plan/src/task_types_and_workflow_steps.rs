use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskType {
    Lint,
    Test,
    Build,
    Deploy,
    Rollback,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub task_type: TaskType,
    pub command: String,
    pub args: Vec<String>,
    pub retry_on_failure: bool,
    pub dir: String,
}

