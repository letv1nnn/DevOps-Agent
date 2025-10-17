use std::env::var;
use std::error::Error;
use async_trait::async_trait;
use tracing::{
    info, span, Level
};
use serde::{
    Serialize, Deserialize
};

mod openai;
mod ollama;

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    pub role: String,
    pub content: String,
}

// creating two structs to implement the RequestLlm trait for them, so it would be easier to extend the crate
struct Ollama {
    model: String,
}

struct Openai {
    model: String,
    api_key: String,
}

#[async_trait]
trait RequestLlm {
    async fn request_llm(&self, prompt: &str, system_prompt: &str) -> Result<String, Box<dyn Error>>;
}

const ENV_ISSUE: &str = "Missing required environment variables: either OPENAI_API_KEY for OpenAI or MODEL for Ollama. Please set one of them in your .env file or system environment.";

pub async fn request_llm(prompt: &str, system_prompt: &str) -> Result<String, Box<dyn Error>> {
    let span = span!(Level::INFO, "");

    let _enter = span.enter();

    let response: String;

    if let Some(model) = var("MODEL").ok() {
        if let Some(api_key) = var("OPENAI_API_KEY").ok() {
            info!("All environment variables for OpenAI has been provided");

            let openai = Openai {
                model,
                api_key,
            };
            
            info!("Making a request to OpenAI");
            response = openai.request_llm(prompt, system_prompt).await?;
            info!("Got the response from OpenAI");
        } else {
            info!("All environment variables for Ollama has been provided");

            let ollama = Ollama {
                model,
            };

            info!("Making a request to OpenAI");
            response = ollama.request_llm(prompt, system_prompt).await?;
            info!("Got the response from Ollama");
        }
    } else {
        return Err(ENV_ISSUE.into());
    }

    Ok(response)
}