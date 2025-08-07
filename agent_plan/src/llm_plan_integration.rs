use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde::ser::StdError;
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

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

#[derive(Debug, Deserialize)]
pub struct LLMResponse {
    pub message: Option<LLMessage>,
    pub done: bool,
}

const LLM_PROMPT: &str = r#"
You are a DevOps Agent you can execute task pipelines from a config file, so your purpose now is to generate a plan based on the user's request.
Plan is only can be made of Lint, Test, Build, Deploy or Rollback. So, plan must consist of 1 to 5 tasks. For the command and args you have to decide which commands and args should be in the plan for each task,
because user is going to provide only task_type and programming language that he uses.
IMPORTANT: 
    1. Your response must be a raw JSON array only. Do NOT include any explanation or markdown formatting (like ```json or ```)!
    2. Each of the tasks must have "task_type", "command", "args" and "retry_on_failure" keys, you must include them!
    3. If user specified the project, you need to add appropriate arguments, for instance for cargo it's --manifest-path /home/letv1n/Projects/DevOps-Agent
Example:
User prompt: Lint and test rust /home/letv1n/Projects/DevOps-Agent/Cargo.toml project. Then, build a Docker image tagged my_app using docker build.
LLM output:
[
    {"task_type" : "Lint", "command" : "cargo", "args" : ["clippy", "--manifest-path", "/home/letv1n/Projects/DevOps-Agent/Cargo.toml"], "retry_on_failure" : false },
    {"task_type" : "Test", "command" : "cargo", "args" : ["test", "--manifest-path", "/home/letv1n/Projects/DevOps-Agent/Cargo.toml"], "retry_on_failure" : true },
    {"task_type" : "Build", "command" : "docker", "args" : ["build", "-t", "my_app", "."], "retry_on_failure" : false }
]

USER PROMPT:
"#;

pub async fn send_request(promt: &str) -> Result<String, Box<dyn StdError>> {
    let client = Client::new();

    let messages = vec![
        LLMessage {
            role: "system".to_string(),
            content: LLM_PROMPT.to_string(),
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

    let cleaned = sanitize_llm_response(&response);
    Ok(cleaned.to_string())
}


async fn llm_prompt_validation(plan: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let parsed: Value = serde_json::from_str(plan)
        .map_err(|e| {
            eprintln!("Failed to parse plan as JSON: {}", e);
            e
        })?;

    let tasks = parsed
        .as_array()
        .ok_or("Plan must be a JSON array")?
        .to_vec();

    for (i, task) in tasks.iter().enumerate() {
        let task_obj = task.as_object().ok_or(format!("Task {} is not an object", i))?;

        if !task_obj.contains_key("task_type") {
            return Err(format!("Task {} missing 'task_type' field", i).into());
        }

        if !task_obj.contains_key("command") {
            return Err(format!("Task {} missing 'command' field", i).into());
        }
    
        if let Some(task_type) = task_obj.get("task_type") {
            let valid_types = ["Lint", "Test", "Build", "Deploy", "Rollback"];
            let type_str = task_type.as_str().ok_or("task_type must be a string")?;
            if !valid_types.contains(&type_str) {
                return Err(format!("Invalid task_type '{}' in task {}", type_str, i).into());
            }
        }
    }
    
    Ok(tasks)
}


pub async fn write_prompt_to_json_file(path: &str, plan: &str) -> Result<(), Box<dyn std::error::Error>> {
    let validated_plan = llm_prompt_validation(plan).await.map_err(|e| {
        eprintln!("Failed to create file {}: {}", path, e);
        e
    })?;

    let json_plan = serde_json::to_string_pretty(&validated_plan)?;

    let mut file = File::create(path).await.map_err(|e| {
        eprintln!("Failed to create file {}: {}", path, e);
        e
    })?;

    file.write_all(json_plan.to_string().as_bytes()).await.map_err(|e| {
        eprintln!("Failed to create file {}: {}", path, e);
        e
    })?;

    Ok(())
}

fn sanitize_llm_response(resp: &str) -> &str {
    resp.trim()
        .strip_prefix("```json")
        .or_else(|| resp.strip_prefix("```")) 
        .and_then(|s| s.strip_suffix("```"))
        .unwrap_or(resp)
        .trim()
}
