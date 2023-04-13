use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
// use chrono::Utc;
use actix_web::middleware::Logger;
use dotenv::dotenv;
use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::Path};

use argon2::Argon2;

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{query, Row};

const MODELS: [&str; 2] = ["gpt-4", "gpt-3.5-turbo"];

fn generate_token() -> String {
    let mut rng = rand::thread_rng();
    let token = format!(
        "{}-{}-{}-{}",
        rng.gen_range(1000..9999),
        rng.gen_range(1000..9999),
        rng.gen_range(1000..9999),
        rng.gen_range(1000..9999)
    );
    token
}

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

#[post("/generate")]
async fn generate(chat_input: web::Json<ChatInput>) -> impl Responder {
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
    let user = query("SELECT token FROM users WHERE token = ?")
        .bind(&token)
        .fetch_all(&pool)
        .await;

    // Check if the token is valid
    if let Ok(user) = user {
        if user.len() > 1 {
            return HttpResponse::InternalServerError().body("There are multiple users with the same token. Please contact the developer. This is insanely rare!");
        } else if user.len() == 0 {
            return HttpResponse::Unauthorized().body("Invalid token");
        }
    } else {
        return HttpResponse::Unauthorized().finish();
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
            // Add this to the log table
            // First, calculate the token amount (it's in the response)
            let body: serde_json::Value = serde_json::from_str(&body).unwrap();
            let total_tokens = body["usage"]["total_tokens"].as_i64().unwrap();
            // also get the model
            let model = body["model"].to_string();

            // Add the tokens to the log table
            let _ = query("INSERT INTO log (token, total_tokens, model) VALUES (?, ?, ?)")
                .bind(&token)
                .bind(total_tokens)
                .bind(model)
                .execute(&pool)
                .await;

            HttpResponse::Ok().json(body)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RegisterInput {
    register_token: String,
    login: String,
    password: String,
}

#[post("/register")]
async fn register(body: web::Json<RegisterInput>) -> impl Responder {
    // Check if the token is valid
    let token = body.register_token.clone();
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
    if let Ok(user) = user {
        if user.len() > 1 {
            return HttpResponse::InternalServerError().finish();
        } else if user.len() == 0 {
            return HttpResponse::Unauthorized().finish();
        }
    } else {
        return HttpResponse::Unauthorized().finish();
    }

    // Check if the username is taken
    let username = body.login.clone();
    let user = query("SELECT * FROM users WHERE username = ?")
        .bind(&username)
        .fetch_all(&pool)
        .await;

    if user.is_ok() && user.unwrap().len() > 0 {
        return HttpResponse::BadRequest().body("Username is taken");
    }

    // Generate a token using the argon2 crate and the salt is in the .env
    let password = body.password.clone();
    let password = password.as_bytes();
    let salt = env::var("SALT").expect("SALT must be set");
    let salt = salt.as_bytes();
    let mut hashed_password = [0u8; 32]; // Can be any desired size
    Argon2::default()
        .hash_password_into(password, salt, &mut hashed_password)
        .unwrap();
    // The hashed password is now in the output_key_material variable

    // For security reasons, we should regenerate the token
    let new_token = generate_token();

    // Update the users credentials
    let _ = query("UPDATE users SET username = ?, password = ?, token = ? WHERE token = ?")
        .bind(&username)
        .bind(&hashed_password.to_vec())
        .bind(&new_token)
        .bind(&token)
        .execute(&pool)
        .await;

    // Return the new token
    HttpResponse::Ok().body(new_token)
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginInput {
    username: String,
    password: String,
}

#[post("/login")]
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

    // Get the hash from the database
    let password_row = query("SELECT password FROM users WHERE username = ?")
        .bind(&username)
        .fetch_one(&pool)
        .await
        .unwrap();

    // To verify the password, we need to hash the password and compare it to the hash in the database
    let password = password.as_bytes();
    let salt = env::var("SALT").expect("SALT must be set");
    let salt = salt.as_bytes();
    let mut hashed_password = [0u8; 32]; // Can be any desired size
    Argon2::default()
        .hash_password_into(password, salt, &mut hashed_password)
        .unwrap();

    // Check if the password is correct
    if password_row.get::<Vec<u8>, _>(0) != hashed_password.to_vec() {
        println!(
            "Incorrect password: {:?} != {:?}",
            password_row.get::<Vec<u8>, _>(0),
            hashed_password.to_vec()
        );
        return HttpResponse::Unauthorized().finish();
    }

    // Return the token
    let token = query("SELECT token FROM users WHERE username = ?")
        .bind(&username)
        .fetch_one(&pool)
        .await
        .unwrap()
        .get::<String, _>(0);

    HttpResponse::Ok().body(token)
}

#[derive(Serialize, Deserialize, Debug)]
struct AdminInput {
    admin_token: String,
}

#[post("/newuser")]
async fn newuser(credentials: web::Json<AdminInput>) -> impl Responder {
    // The admin token is in the environment variables
    let admin_token = env::var("ADMIN_TOKEN").expect("ADMIN_TOKEN must be set");

    // Check if the admin token is valid
    if credentials.admin_token != admin_token {
        return HttpResponse::Unauthorized().finish();
    }

    // Start a pool
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("data.db")
        .await
        .unwrap();

    // Generate a token
    let token = generate_token();

    // Create the user
    query("INSERT INTO users (token) VALUES (?)")
        .bind(&token)
        .execute(&pool)
        .await
        .unwrap();

    // Return the token
    HttpResponse::Ok().body(token)
}

#[derive(Serialize, Deserialize, Debug)]
struct MainDatabase {
    id: i32,
    username: String,
    password: Vec<u8>,
    token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LogDatabase {
    id: i32,
    token: String,
    model: String,
    message: String,
    total_tokens: i32,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DatabaseResponse {
    time: String,
    database: String,
    log: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Set logging to info
    env::set_var("RUST_LOG", "info");
    // env::set_var("RUST_BACKTRACE", "1");

    // Start the logger
    env_logger::init();

    // Check if the database exists
    if !Path::new("data.db").exists() {
        // Create the database file
        File::create("data.db").unwrap();

        // Start a pool for sqlite
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("data.db")
            .await
            .unwrap();

        // Create the table
        query("CREATE TABLE users (id INTEGER PRIMARY KEY, username VARCHAR(255), password BLOB, token VARCHAR(20))")
            .execute(&pool)
            .await
            .unwrap();

        query("CREATE TABLE log (id INTEGER PRIMARY KEY, token VARCHAR(20), model VARCHAR(20), total_tokens INTEGER DEFAULT 0, timestamp DATETIME DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool)
            .await
            .unwrap();
    }

    HttpServer::new(move || {
        // This allows the server to be accessed from https://chat.ducky.pics and only allows GET and POST requests from that domain.
        let cors = Cors::default()
            .allowed_origin("https://chat.ducky.pics")
            // .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allow_any_header()
            .supports_credentials() // This allows cookies to be sent
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(generate)
            .service(login)
            .service(register)
            .service(newuser)
    })
    .bind("0.0.0.0:8456")?
    .run()
    .await
}
