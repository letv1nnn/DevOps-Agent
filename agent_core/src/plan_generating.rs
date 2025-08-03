use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde::ser::StdError;
use serde_json::json;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::info;

#[derive(Serialize, Deserialize, Debug)]
pub struct LLMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct LLMRequest {
    pub model: String,
    pub messages: Vec<LLMessage>,
    pub temperature: Option<f32>,
}

// LLM (Ollama) REQUEST TO GENERATE A TASK EXECUTION PIPELINE PLAN
#[derive(Debug, Deserialize)]
pub struct LLMResponse {
    pub message: Option<LLMessage>,
    pub done: bool,
}

pub async fn send_request(promt: &str) -> Result<String, Box<dyn StdError>> {
    let client = Client::new();

    let system_prompt = r#"
    You are a DevOps Agent you can execute task pipelines from a config file, so your purpose now is to generate a plan based on the user's request.
    Plan is only can be made of Lint, Test, Build, Deploy or Rollback. So, plan must consist of 1 to 5 tasks. For the command and args you have to decide which commands and args should be in the plan for each task,
    because user is going to provide only task_type and programming language that he uses.
    Example:
    User prompt: Run a lint check using cargo clippy, then execute tests with cargo test and retry if they fail. Finally, build a Docker image tagged my_app using docker build.
    LLM output:
    [
        {"task_type" : "Lint", "command" : "cargo", "args" : ["clippy"], "retry_on_failure" : false },
        {"task_type" : "Test", "command" : "cargo", "args" : ["test"], "retry_on_failure" : true },
        {"task_type" : "Build", "command" : "docker", "args" : ["build", "-t", "my_app", "."], "retry_on_failure" : false }
    ]
    "#;

    let messages = vec![
        LLMessage {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        },
        LLMessage {
            role: "user".to_string(),
            content: promt.to_string(),
        },
    ];

    let request = LLMRequest {
        model: "mistral".to_string(), // you might change this model to more faster and efficient llm
        messages,
        temperature: Some(0.3),
    };

    let response_result = client
        .post("http://localhost:11434/api/chat")
        .json(&request)
        .send()
        .await?;

    if !response_result.status().is_success() {
        return Err(format!("Server returned status code: {}", response_result.status()).into());
    }

    let body = response_result.text().await?;

    let mut response = String::new();
    for line in body.lines() {
        if let Ok(chat_response) = serde_json::from_str::<LLMResponse>(line) {
            if let Some(message) = chat_response.message {
                response.push_str(&message.content);
            }
        }  
    }

    Ok(response)
}


// VALIDATING AND WRITING LLM GENERATED PIPELINE TO THE JSON CONFIG FILE
// VALIDATION AND WRITING INTO FILE SHOULD BE LOGGED

pub async fn plan_validation(_plan: &str) -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}

pub async fn json_config_file(path: &str, plan: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Writing pipeline config to {}", path);
    
    let plan = match plan_validation(plan) {
        Ok(_) => (),
        Err(e) => {
            eprint!("Error: {e}");
            return std::error::Error;
        }
    }

    let plan = json!(plan);

    let mut file = File::create(path).await?;
    file.write_all(plan.to_string().as_bytes()).await?;

    info!("Successfully wrote pipeline config to {}", path);

    Ok(())
} 

