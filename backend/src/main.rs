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
 use rand::Rng;

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::query;

const MODELS: [&str; 2] = ["gpt-4", "gpt-3.5-turbo"];


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    content: String,
    role: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChatInput {
    model: String,
    messages: Vec<Message>,
    token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatOutput {
    model: String,
    messages: Vec<Message>,
}


#[post("/api/generate")]
async fn chatgpt_clone(chat_input: web::Json<ChatInput>) -> impl Responder {
    // Check if the key is valid
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    // Check if the token is valid
    let token = chat_input.token.clone();
    // Start a pool
    let pool = SqlitePoolOptions::new()
    .max_connections(1)
    .connect("data.db")
    .await
    .unwrap();
// See if the token exists
let user = query("SELECT * FROM users WHERE token = ?")
.bind(&token)
.fetch_all(&pool)
.await;

if user.is_err() {
        return HttpResponse::Unauthorized().finish();
    } else if user.unwrap().len() > 1 {
        return HttpResponse::InternalServerError().finish();
    }
    
    // Check if the model is valid
    let model = chat_input.model.clone();
    
    if !MODELS.contains(&model.as_str()) {
        return HttpResponse::BadRequest().finish();
    }

    // Create a client
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

#[derive(Serialize, Deserialize, Debug)]
struct RegisterInput { 
    registerToken: String,
}

#[post("/api/register")]
async fn register(body: web::Json<RegisterInput>) -> impl Responder {
    // Check if the token is valid
    let token = body.registerToken.clone();
    // Start a pool
    let pool = SqlitePoolOptions::new()
    .max_connections(1)
    .connect("data.db")
    .await
    .unwrap();

// See if the token exists
let user = query("SELECT * FROM users WHERE token = ?")
.bind(&token)
.fetch_all(&pool)
.await;

// Check if the token is valid
if user.is_err() {
        return HttpResponse::Unauthorized().finish();
    } else if user.unwrap().len() > 1 {
        return HttpResponse::InternalServerError().finish();
    }


// Return that the register token is successful
HttpResponse::Ok().finish()
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginInput {
    username: String,
    password: String,
}

#[post("/api/login")]
async fn login(credentials: web::Json<LoginInput>) -> impl Responder {
    // Check the database
    let username = credentials.username.clone();
    let password = credentials.password.clone();
    
    // Start a pool
    let pool = SqlitePoolOptions::new()
    .max_connections(1)
    .connect("data.db")
    .await
    .unwrap();

// Check if the user exists
let user = query("SELECT * FROM users WHERE username = ? AND password = ?")
.bind(&username)
.bind(&password)
.fetch_one(&pool)
.await;

match user {
    Ok(_) => {
        // Authorized
        
        // Generate a alphanumeric character like this xxxx-xxxx-xxxx-xxxx using the rand crate
        let mut rng = rand::thread_rng();
        let token = format!(
            "{}-{}-{}-{}",
            rng.gen_range(1000..9999),
            rng.gen_range(1000..9999),
            rng.gen_range(1000..9999),
            rng.gen_range(1000..9999)
        );
        
        // Update the token in the database
        query("UPDATE users SET token = ? WHERE username = ? AND password = ?")
        .bind(&token)
        .bind(username)
                .bind(password)
                .execute(&pool)
                .await
                .unwrap();

            // Return the token
            HttpResponse::Ok().body(token)
            
        }
        // Unauthorized
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NewUserInput {
    new_username: String,
    new_password: String, 
}


#[post("/api/newuser")]
async fn newuser(credentials: web::Json<NewUserInput>) -> impl Responder {
    // Check the database
    let username = credentials.new_username.clone();
    let password = credentials.new_password.clone();

    // Start a pool
    let pool = SqlitePoolOptions::new()
    .max_connections(1)
    .connect("data.db")
    .await
    .unwrap();

// Check if the user exists
let user = query("SELECT * FROM users WHERE username = ?")
.bind(&username)
.fetch_one(&pool)
.await;

    if !user.is_err() {
        // User exists
        return HttpResponse::BadRequest().finish();
    }

    // Generate a token
    let mut rng = rand::thread_rng();
    let token = format!(
        "{}-{}-{}-{}",
        rng.gen_range(1000..9999),
        rng.gen_range(1000..9999),
        rng.gen_range(1000..9999),
        rng.gen_range(1000..9999)
    );

    // Create the user
    query("INSERT INTO users (username, password, token) VALUES (?, ?, ?)")
    .bind(&username)
    .bind(&password)
    .bind(&token)
    .execute(&pool)
    .await
    .unwrap();

    // Return the token
    HttpResponse::Ok().body(token)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // A database for storing requests for rate limiting
    let backend = InMemoryBackend::builder().build();
    
    // Start a pool for sqlite
    let pool = SqlitePoolOptions::new()
    .max_connections(1)
        .connect("data.db")
        .await
        .unwrap();
    // This will work everywhere

     // Check if the database exists
    let tables = query("SELECT name FROM sqlite_master WHERE type='table'")
        .fetch_all(&pool)
        .await
        .unwrap();

    if tables.len() < 1 {
        // Create the table
        query("CREATE TABLE users (id INTEGER PRIMARY KEY, username VARCH, password VARCHAR(255), token TEXT, firstTime BOOLEAN DEFAULT 1)")
            .execute(&pool)
            .await
            .unwrap();

        // Create the admin user for testing
        query("INSERT INTO users (username, password) VALUES (?, ?)")
            .bind("admin")
            .bind("admin123")
            .execute(&pool)
            .await
            .unwrap();
    }
                                    

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
            .service(login)
            .service(register)
            .service(newuser)
            
            .wrap(rate_limit)
            .wrap(cors)
    })
    .bind("0.0.0.0:8456")?
    .run()
    .await
}
