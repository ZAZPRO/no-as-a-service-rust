use axum::{Json, Router, extract::State, routing::get};
use rand::Rng;
use serde::Serialize;
use std::env;
use tokio::net::TcpListener;

#[derive(Clone, Debug, Default)]
struct AppState {
    pub reasons: Vec<String>,
    pub len: usize,
}

#[derive(Clone, Debug, Default, Serialize)]
struct ApiResponse {
    reason: String,
}

impl ApiResponse {
    pub fn new(reason: String) -> Self {
        Self { reason }
    }
}

async fn get_random_reason(State(state): State<AppState>) -> Json<ApiResponse> {
    let random_index = rand::rng().random_range(..state.len);
    let random_reason = state.reasons[random_index].clone();

    Json(ApiResponse::new(random_reason))
}

#[tokio::main]
async fn main() {
    let reasons_string: &str = include_str!("../reasons.json");
    let reasons: Vec<String> =
        serde_json::from_str(reasons_string).expect("Can not parse reasons file");
    let reasons_amount = reasons.len();
    println!("Loaded {reasons_amount} reasons!");

    let app_state = AppState {
        len: reasons_amount,
        reasons,
    };
    let app = Router::new()
        .route("/no", get(get_random_reason))
        .with_state(app_state);

    let ip: String = env::var("NAAS_IP").unwrap_or("0.0.0.0".to_string());
    let port: String = env::var("NAAS_PORT").unwrap_or("3000".to_string());
    let address: String = format!("{ip}:{port}");
    println!("No As a Service is running at: {address}");
    let listener = TcpListener::bind(address.as_str())
        .await
        .expect("Failed to create TCP listener");
    axum::serve(listener, app)
        .await
        .expect("Failed to start axum app");
}
