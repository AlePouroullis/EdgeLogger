mod message;
mod db;
use std::str;
use serde_json;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use chrono::Utc;
use dotenvy::dotenv;
use message::{LogMessage, Response};
use db::pool::create_pool;
use db::models::{MachineLog, Metric};
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = match create_pool().await {
        Ok(pool) => Arc::new(pool),
        Err(e) => {
            eprintln!("Failed to create pool: {:?}", e);
            return Ok(());
        }
    };

    match pool.acquire().await {
        Ok(conn) => {
            println!("Connected to database");
        }
        Err(e) => {
            eprintln!("Failed to acquire connection: {:?}", e);
        }
    };

    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    println!("Server listening on port 8000...");

    loop {
        let (mut stream, addr) = listener.accept().await?;
        println!("New connection from {}", addr);
        let pool = Arc::clone(&pool);

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            loop {
                match stream.read(&mut buffer).await {
                    Ok(n) if n == 0 => {
                        println!("Client disconnected");
                        break;
                    }
                    Ok(n) => {
                        // Convert buffer to string 
                        if let Ok(message_str) = str::from_utf8(&buffer[..n]) {
                            let response = match serde_json::from_str::<LogMessage>(message_str) {
                                Ok(log_message) => {
                                    println!("Processing message from machine: {}", log_message.machine_id);
                
                                    // Save log to database with original raw JSON
                                    let raw_json: serde_json::Value = serde_json::from_str(message_str)
                                        .expect("Valid JSON already checked");
                                    
                                    let metrics: Vec<(String, f64)> = log_message.metrics
                                    .into_iter()
                                    .map(|(name, value)| (name, value))
                                    .collect();

                                    match MachineLog::create_with_metrics(
                                        &pool,
                                        log_message.machine_id.clone(),
                                        raw_json,
                                        metrics
                                    ).await {
                                        Ok((log, metrics)) => {
                                            println!("Stored log and {} metrics", metrics.len());
                                            Response {
                                                status: "success".to_string(),
                                                message: "Log received and stored".to_string(),
                                                timestamp: Utc::now().to_rfc3339()
                                            }
                                        },
                                        Err(e) => {
                                            eprintln!("Failed to store log: {}", e);
                                            Response {
                                                status: "error".to_string(),
                                                message: "Failed to store log".to_string(),
                                                timestamp: Utc::now().to_rfc3339()
                                            }
                                        }
                                    }
                                    
                               
                                }
                                Err(e) => {
                                    println!("Failed to parse JSON: {}", e);

                                    Response {
                                        status: "error".to_string(),
                                        message: format!("Invalid JSON: {}", e),
                                        timestamp: Utc::now().to_rfc3339()
                                    }
                                }
                            };

                            if let Ok(response_json) = serde_json::to_string(&response){
                                if let Err(e) = stream.write_all(response_json.as_bytes()).await {
                                    eprintln!("Failed to send response: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from connection: {}", e);
                        break;
                    }
                }
            }
        });
    }
}
 