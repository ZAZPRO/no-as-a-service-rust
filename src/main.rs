use std::env;

use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let ip: String = env::var("NOAAS_IP").unwrap_or("0.0.0.0".to_string());
    let port: String = env::var("NOAAS_PORT").unwrap_or("3000".to_string());
    let address: String = format!("{}:{}", ip, port);

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = TcpListener::bind(address.as_str()).await.unwrap();
    println!("No As a Service is running at: {}", address);
    axum::serve(listener, app).await.unwrap();
}
