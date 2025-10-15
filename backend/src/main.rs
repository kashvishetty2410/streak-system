use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;

// Fun variable names
const CONFETTI: &str = "🎉✨🎆";

#[derive(Deserialize)]
struct ClaimRequest {
    username: String,
}

#[derive(Serialize)]
struct ClaimResponse {
    reward: i128,
    streak: u32,
    message: String,
}

#[post("/claim")]
async fn claim(req: web::Json<ClaimRequest>) -> impl Responder {
    let username = req.username.clone();
    log::info!("{} Claim endpoint hit by {}", CONFETTI, username);

    // Placeholder: call Soroban contract (we'll simulate a claim)
    let now_ts = chrono::Utc::now().timestamp() as u64;

    // Simulate a successful claim with toy logic similar to contract
    let simulated_reward = 10i128;
    let simulated_streak = 3u32;

    // Placeholder: send Stellar XLM via Stellar SDK for Rust (not mature), so we stub it
    log::info!("Sending {} XLM to {} (simulated)", simulated_reward, username);

    let message = if username.contains("Unicorn") {
        "You brought unicorns! 🦄✨".to_string()
    } else if username.contains("Dragon") {
        "Dragon breathes fire! 🐉 +42 bonus applied".to_string()
    } else {
        "Claim successful!".to_string()
    };

    let res = ClaimResponse { reward: simulated_reward, streak: simulated_streak, message };
    HttpResponse::Ok().json(res)
}

#[get("/streak/{username}")]
async fn get_streak(path: web::Path<String>) -> impl Responder {
    let username = path.into_inner();
    log::info!("Fetching streak for {}", username);

    // Placeholder response — would query Soroban contract for real data
    let resp = serde_json::json!({
        "username": username,
        "streak": 3,
        "last_claim": 0,
        "total_rewards": 100
    });
    HttpResponse::Ok().json(resp)
}

#[get("/leaderboard")]
async fn leaderboard() -> impl Responder {
    log::info!("Leaderboard requested — assembling ASCII art 🏆");
    let board = vec![
        "🏆 1. Alice - 30 days",
        "🎉 2. Bob - 14 days",
        "🥳 3. Carol - 7 days",
    ];
    let ascii = r#"
      ___________
     '._==_==_=_.'
     .-\:      /-.
    | (|:.     |) |
     '-|:.     |-' 
       \::.    /
        '::..'"
    "#;

    let payload = serde_json::json!({ "board": board, "ascii": ascii });
    HttpResponse::Ok().json(payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".into());
    let bind = format!("{}:{}", host, port);

    println!("{} Streakinator3000 backend starting on {}", CONFETTI, bind);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
            
        App::new()
            .wrap(cors)
            .service(claim)
            .service(get_streak)
            .service(leaderboard)
    })
    .bind(bind)?
    .run()
    .await
}
