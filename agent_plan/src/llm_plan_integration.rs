use std::env;

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
because user is going to provide only task_type and programming language that he uses. You must include 5 json keys in each task: "task_type", "command", "args", "retry_on_failure" and "dir".
Task types can be only Lint, Test, Build, Deploy or Rollback. The command must be a valid command for the task type and args must be a vector of strings.
IMPORTANT: 
    1. Your response must be a raw JSON array only. Do NOT include any explanation or markdown formatting (like ```json or ```)!
    2. Each of the tasks must have "task_type", "command", "args" and "retry_on_failure" keys, you must include them!
    3. If user specified the project, you need to add appropriate arguments, for instance for cargo it's --manifest-path /home/letv1n/Projects/DevOps-Agent
    4. if user has said to do something in the current directory, just put curr in the dirs value, like dir: "curr".
    5. your response MUST have strictly 5 fields: "task_type" : "String", "command" : "String", "args" : [vector], "retry_on_failure" : bool, "dir": String
Example:
User prompt: Lint and test rust /home/letv1n/Projects/DevOps-Agent/Cargo.toml project. Then, build a Docker image tagged my_app using docker build.
LLM output:
[
    {"task_type" : "Lint", "command" : "cargo", "args" : ["clippy"], "retry_on_failure" : false, "dir": "/home/letv1n/Projects/DevOps-Agent/Cargo.toml" },
    {"task_type" : "Test", "command" : "cargo", "args" : ["test"], "retry_on_failure" : true, "dir": "/home/letv1n/Projects/DevOps-Agent/Cargo.toml" },
    {"task_type" : "Build", "command" : "docker", "args" : ["build", "-t", "my_app", "."], "retry_on_failure" : false, "dir": "path/to/the/docker/image" }
]

USER PROMPT:
"#;


pub async fn send_request(promt: &str) -> Result<String, Box<dyn StdError + Sync + Send>> {
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


async fn llm_prompt_validation(plan: &str) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let parsed: Value = serde_json::from_str(plan)
        .map_err(|e| {
            eprintln!("Failed to parse plan as JSON: {}", e);
            e
        })?;

    let mut tasks = parsed
        .as_array()
        .ok_or("Plan must be a JSON array")?
        .to_vec();

    for (i, task) in tasks.iter_mut().enumerate() {
        let task_obj = task.as_object_mut().ok_or(format!("Task {} is not an object", i))?;

        let json_keys = vec!["task_type", "command", "retry_on_failure", "dir", "args"];

        for key in json_keys {
            if !task_obj.contains_key(key) {
                return Err(format!("Task {} missing {} field", i, key).into());
            }
            if key == "dir" {
                if task_obj[key].to_string().len() == 0 {
                    return Err(format!("Task {} has empty dir field", i).into());
                }
            }
        }

        if task_obj["dir"] == "curr" {
            let curr_dir = env::current_dir()?.canonicalize()?;
            let curr_dir_str = curr_dir.to_string_lossy().replace(r"\\?\", r"");
            let curr_dir_str = curr_dir_str.replace(r"\", "\\");
            task_obj["dir"] = serde_json::Value::String(curr_dir_str)
        }

        
        if let Some(task_type) = task_obj.get("task_type") {
            let valid_types = ["Lint", "Test", "Build", "Deploy", "Rollback"];
            let type_str = task_type.as_str().ok_or("task_type must be a string")?;
            if !valid_types.contains(&type_str) {
                return Err(format!("Invalid task_type '{}' in task {}", type_str, i).into());
            }
        }

        if let Some(command) = task_obj.get("command") {
            let forbidden_commands: Vec<&'static str> = vec![
                "rm", "mv", "dd", "mkfs", "shutdown", "reboot", "halt",
                "poweroff", "chmod", "chown", "kill", "pkill", "killall",
                "useradd", "userdel", "passwd", "mount", "umount",
                "iptables", "ufw", "curl", "wget", "scp", "ftp",
                "nc", "netcat", "telnet", "bash", "sh",
            ];
            let command_str = command.as_str().ok_or("command must be a string")?;
            if forbidden_commands.contains(&command_str) {
                return Err(format!("Can proceed Task {}, because LLM generated forbidden or unsafe command: {}", i, command_str).into());
            }
        }
    }
    
    Ok(tasks)
}


pub async fn write_prompt_to_json_file(path: &str, plan: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let validated_plan = match llm_prompt_validation(plan).await {
        Ok(tasks) => tasks,
        Err(e) => {
            eprintln!("Failed to create or load to file {}: {}. Retrying plan generation...", path, e);
            llm_prompt_validation(plan).await.map_err(|e| {
                eprintln!("Failed to create or load to file {}: {}. Try again", path, e);
                e
            })?
        }
    };

    let json_plan = serde_json::to_string_pretty(&validated_plan)?;

    let mut file = File::create(path).await.map_err(|e| {
        eprintln!("Failed to create file or load to {}: {}", path, e);
        e
    })?;

    file.write_all(json_plan.to_string().as_bytes()).await.map_err(|e| {
        eprintln!("Failed to create file or load to {}: {}", path, e);
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
