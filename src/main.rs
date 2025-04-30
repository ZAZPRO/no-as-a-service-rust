#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

use bytes::BytesMut;
use core::str;
use rand::Rng;
use std::{env, error::Error, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

async fn process(
    stream: TcpStream,
    reasons: &Vec<String>,
    len: usize,
) -> Result<(), Box<dyn Error>> {
    let mut stream = BufReader::new(stream);
    let mut buffer = BytesMut::with_capacity(7);
    stream.read_buf(&mut buffer).await?;
    let s = str::from_utf8_mut(&mut buffer)?;
    if s == "GET /no" {
        let random_index = rand::rng().random_range(..len);
        let random_reason = reasons[random_index].as_str();
        let response = format!(
            "HTTP/1.1 200 \ncontent-length: {}\ncontent-type: application/json\n\n{{\"reason\":\"{random_reason}\"}}",
            random_reason.len() + 13
        );

        stream.write_all(response.as_bytes()).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let reasons_string: &str = include_str!("../reasons.json");
    let reasons: Vec<String> =
        serde_json::from_str(reasons_string).expect("Can not parse reasons file");
    let reasons_amount = reasons.len();
    println!("Loaded {reasons_amount} reasons!");

    let ip: String = env::var("NAAS_IP").unwrap_or("0.0.0.0".to_string());
    let port: String = env::var("NAAS_PORT").unwrap_or("3000".to_string());
    let address: String = format!("{ip}:{port}");
    println!("No As a Service is running at: {address}");
    let listener = TcpListener::bind(address.as_str())
        .await
        .expect("Failed to create TCP listener");
    let shared_reasons = Arc::new(reasons);
    loop {
        let (stream, _) = listener.accept().await?;
        let reasons = Arc::clone(&shared_reasons);
        tokio::spawn(async move {
            if let Err(e) = process(stream, &reasons, reasons_amount).await {
                println!("failed to process connection; error = {e}");
            }
        });
    }
}
