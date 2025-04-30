use std::env;

use axum::{Json, Router, extract::State, routing::get};
use rand::Rng;
use serde::Serialize;
use tokio::{fs, net::TcpListener};

#[derive(Clone, Debug, Default)]
struct AppState {
    pub reasons: Vec<String>,
    pub len: usize,
}

#[derive(Clone, Debug, Default, Serialize)]
struct ApiResponse {
    reason: String,
}

async fn load_reasons() -> Result<Vec<String>, std::io::Error> {
    let reasons_file: String = fs::read_to_string("reasons.json").await?;
    let v: Vec<String> = serde_json::from_str(reasons_file.as_str())?;

    Ok(v)
}

async fn get_random_reason(State(state): State<AppState>) -> Json<ApiResponse> {
    let random_index = rand::rng().random_range(..state.len);
    let random_reason = state.reasons[random_index].clone();

    let resp = ApiResponse {
        reason: random_reason,
    };
    Json(resp)
}

#[tokio::main]
async fn main() {
    let reasons = load_reasons()
        .await
        .expect("Failed to load reasons.json file");
    let reasons_amount = reasons.len();
    println!("Loaded {reasons_amount} reasons!");

    let app_state = AppState {
        len: reasons_amount,
        reasons,
    };

    let ip: String = env::var("NOAAS_IP").unwrap_or("0.0.0.0".to_string());
    let port: String = env::var("NOAAS_PORT").unwrap_or("3000".to_string());
    let address: String = format!("{ip}:{port}");

    let app = Router::new()
        .route("/no", get(get_random_reason))
        .with_state(app_state);

    let listener = TcpListener::bind(address.as_str()).await.unwrap();
    println!("No As a Service is running at: {address}");
    axum::serve(listener, app).await.unwrap();
}
