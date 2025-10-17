use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use async_trait::async_trait;

use crate::{
    ChatMessage, Openai, RequestLlm
};

#[derive(Serialize)]
struct OpenaiRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
}

#[derive(Deserialize)]
struct OpenaiResponse {
    pub choices: Vec<ChatMessage>
}

// openai is not tested, because I do not have api key, if you encounter any issues, leave a comment pls
#[async_trait]
impl RequestLlm for Openai {
    async fn request_llm(&self, prompt: &str, system_prompt: &str) -> Result<String, Box<dyn Error>> {
        let client = Client::new();

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: self.model.clone(),
                content: prompt.to_string(),
            },
        ];

        let request_body = OpenaiRequest {
            model: self.model.clone(),
            messages,
            temperature: Some(0.7),
        };

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(self.api_key.clone())
            .json(&request_body)
            .send()
            .await?
            .json::<OpenaiResponse>()
            .await?;

        let reply = response
            .choices
            .first()
            .map(|m| m.content.clone())
            .unwrap_or_else(|| "<no reply>".to_string());

        Ok(reply)
    }

}