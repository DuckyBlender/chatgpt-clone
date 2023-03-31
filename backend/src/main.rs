use actix_cors::Cors;
use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder},
    RateLimiter,
};
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::Path, time::Duration};

use argon2::Argon2;

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{query, Row};

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
            // Add this to the log table
            // First, calculate the token amount (it's in the response)

            let body: serde_json::Value = serde_json::from_str(&body).unwrap();
            let total_tokens = body["usage"]["total_tokens"].as_i64().unwrap();

            // Add the tokens to the log table
            let _ = query("INSERT INTO log (token, tokens) VALUES (?, ?)")
                .bind(&token)
                .bind(total_tokens)
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

#[post("/api/register")]
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
    if user.is_err() {
        return HttpResponse::Unauthorized().finish();
    } else if user.unwrap().len() > 1 {
        // if this happens something is seriously wrong
        println!("More than one user with the same token: {}", token);
        return HttpResponse::InternalServerError().finish();
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

    // Update the users credentials
    let _ = query("UPDATE users SET username = ?, password = ? WHERE token = ?")
        .bind(&username)
        .bind(&hashed_password.to_vec())
        .bind(&token)
        .execute(&pool)
        .await;

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
struct NewUserInput {
    admin_token: String,
}

#[post("/api/newuser")]
async fn newuser(credentials: web::Json<NewUserInput>) -> impl Responder {
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
    let mut rng = rand::thread_rng();
    let token = format!(
        "{}-{}-{}-{}",
        rng.gen_range(1000..9999),
        rng.gen_range(1000..9999),
        rng.gen_range(1000..9999),
        rng.gen_range(1000..9999)
    );

    // Create the user
    query("INSERT INTO users (token) VALUES (?)")
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

    // Set logging to info
    env::set_var("RUST_LOG", "debug");
    // env::set_var("RUST_BACKTRACE", "1");

    // Start the logger
    env_logger::init();

    // A database for storing requests for rate limiting
    let backend = InMemoryBackend::builder().build();

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
        // This will work everywhere

        // Check if the database exists
        let tables = query("SELECT name FROM sqlite_master WHERE type='table'")
            .fetch_all(&pool)
            .await
            .unwrap();

        if tables.len() < 1 {
            // Create the table
            query("CREATE TABLE users (id INTEGER PRIMARY KEY, username VARCHAR(255), password BLOB, token VARCHAR(20))")
            .execute(&pool)
            .await
            .unwrap();

            query("CREATE TABLE logs (id INTEGER PRIMARY KEY, username VARCHAR(255), token VARCHAR(20), message VARCHAR(255), total_tokens INTEGER DEFAULT 0, timestamp DATETIME DEFAULT CURRENT_TIMESTAMP)")
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
            .service(generate)
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
