use actix_cors::Cors;
use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder},
    RateLimiter,
};
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::{env, time::Duration};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    content: String,
    role: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChatInput {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatOutput {
    model: String,
    messages: Vec<Message>,
}

#[post("/")]
async fn chatgpt_clone(chat_input: web::Json<ChatInput>) -> impl Responder {
    // Check if the key is valid
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Remove the key from the request
    let chat_output = ChatOutput {
        model: chat_input.model.clone(),
        messages: chat_input.messages.clone(),
    };

    let json = serde_json::to_string(&chat_output).unwrap();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .headers(headers)
        .body(json)
        .send()
        .await;

    match response {
        Ok(res) => {
            // Convert to JSON and return
            let body = res.text().await.unwrap();
            let body: serde_json::Value = serde_json::from_str(&body).unwrap();
            HttpResponse::Ok().json(body)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // A database for storing requests for rate limiting
    let backend = InMemoryBackend::builder().build();

    HttpServer::new(move || {
        // Assign a limit of 5 requests per minute per client ip address
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(5), 1)
            .real_ip_key()
            .build();
        let rate_limit = RateLimiter::builder(backend.clone(), input)
            .add_headers()
            .build();

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .service(chatgpt_clone)
            .wrap(rate_limit)
            .wrap(cors)
    })
    .bind("0.0.0.0:8456")?
    .run()
    .await
}
