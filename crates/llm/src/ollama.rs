use reqwest::Client;
use std::error::Error;
use async_trait::async_trait;
use serde::{
    Deserialize, Serialize
};

use crate::{
    ChatMessage, Ollama, RequestLlm
};

#[derive(Serialize)]
struct OllamaRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    pub message: Option<ChatMessage>,
    #[allow(unused)]
    pub done: bool,
}

#[async_trait]
impl RequestLlm for Ollama {
    async fn request_llm(&self, prompt: &str, system_prompt: &str) -> Result<String, Box<dyn Error>> {
        let client = Client::new();

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ];

        let request = OllamaRequest {
            model: self.model.clone(),
            messages,
            stream: false,
        };

        let response = client
            .post("http://localhost:11434/api/chat")
            .json(&request)
            .send()
            .await?
            .json::<OllamaResponse>()
            .await?;

        if let Some(message) = response.message {
            Ok(message.content)
        } else {
            Ok("<no reply>".to_string())
        }
    }
}