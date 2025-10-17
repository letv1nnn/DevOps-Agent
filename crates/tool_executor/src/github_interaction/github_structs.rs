use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WorkflowRunsResponse {
    pub workflow_runs: Vec<WorkflowRun>,
}

#[derive(Deserialize, Debug)]
pub struct WorkflowRun {
    pub id: u64,
    pub status: String,
    pub conclusion: Option<String>,
}
