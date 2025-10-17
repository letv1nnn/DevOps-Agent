use std::env::var;
use agent_core::agent_structs::Step;

pub fn get_pipeline() -> Option<Vec<Step>> {
    if let Some(pipeline) = var("PIPELINE").ok() {
        let pipeline = pipeline
        .split_ascii_whitespace()
        .map(|step_name| Step {
            name: step_name.to_string(),
            args: vec![],
        })
        .collect::<Vec<Step>>();
        return Some(pipeline);
    }
    None
}