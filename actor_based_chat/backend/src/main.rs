use actix::prelude::*;
use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running...");
    env_logger::init();

    let base_url = std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434".into());

    let model_addr = ModelActor {
        base_url,
        client: reqwest::Client::new(),
    }
    .start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(model_addr.clone()))
            .route("/api/chat", web::post().to(chat_endpoint))
            // Serve your static frontend (adjust path as needed)
            .service(Files::new("/", "../frontend").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


// Actor definition and message

// message, that carries session_id, model amd prompt
#[derive(Message)]
#[rtype(result = "Result<String, String>")]
struct AskModel {
    session_id: String,
    model: String,
    prompt: String,
}

// Actor that handles that message by trying Ollama. If that's down, you still see a reply (echo).
struct ModelActor {
    base_url: String,
    client: reqwest::Client,
}

impl Actor for ModelActor {
    type Context = Context<Self>;
}

// implement the handler
impl Handler<AskModel> for ModelActor {
    type Result = ResponseFuture<Result<String, String>>;

    fn handle(&mut self, msg: AskModel, _ctx: &mut Self::Context) -> Self::Result {
        let client = self.client.clone();
        let base = self.base_url.clone();

        Box::pin(async move {
            #[derive(Serialize)]
            struct OllamaMessage {
                role: String,
                content: String,
            }
            #[derive(Serialize)]
            struct OllamaRequest {
                model: String,
                messages: Vec<OllamaMessage>,
                stream: bool
            }

            let url = format!("{}/api/chat", base);
            let req = OllamaRequest {
                model: msg.model.clone(),
                messages: vec![OllamaMessage { role: "user".into(), content: msg.prompt.clone() }],
                stream: false,
            };

            let resp = client.post(url).json(&req).send().await;

            match resp {
                Ok(r) if r.status().is_success() => {
                    let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
                    if let Some(s) = v.get("message").and_then(|m| m.get("content")).and_then(|c| c.as_str()) {
                        Ok(s.to_string())
                    } else if let Some(s) = v.get("reply").and_then(|c| c.as_str()) {
                        Ok(s.to_string())
                    } else {
                        Ok(v.to_string())
                    }
                },
                _ => Ok(format!("(dev echo) session={} model={} -> {}", msg.session_id, msg.model, msg.prompt))
            }
        })
    }
}


// http endpoint
// frontend posts json: { "sessionId": "default", "model": "mistral", "message": "..." }
// need to parse that and send an AskModel to the actor
#[derive(Deserialize)]
struct ChatIn {
    #[serde(rename = "sessionId")]
    session_id: String,
    model: String,
    message: String,
}

#[derive(Serialize)]
struct ChatOut {
    reply: String,
}

async fn chat_endpoint(payload: web::Json<ChatIn>, model_addr: web::Data<Addr<ModelActor>>) -> impl Responder {
    let msg = AskModel {
        session_id: payload.session_id.clone(),
        model: payload.model.clone(),
        prompt: payload.message.clone(),
    };

    match model_addr.send(msg).await {
        Ok(Ok(reply)) => HttpResponse::Ok().json(ChatOut { reply }),
        Ok(Err(err))  => HttpResponse::InternalServerError().json(ChatOut { reply: format!("error: {}", err) }),
        Err(mailbox)  => HttpResponse::InternalServerError().json(ChatOut { reply: format!("mailbox error: {}", mailbox) }),
    }
}
