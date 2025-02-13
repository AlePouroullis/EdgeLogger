mod message;
use message::{LogMessage, Response};
use serde_json;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{sleep, Duration};
use std::str;
use chrono::Utc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    println!("Server listening on port 8000...");

    loop {
        let (mut stream, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

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
                
                                    // Simulate some processing time - different for each machine
                                    let delay = Duration::from_millis(
                                        100 * (log_message.machine_id.split('-').nth(1).unwrap_or("1").parse::<u64>().unwrap_or(1))
                                    );
                                    sleep(delay).await;  // Note the .await here!
                                    
                                    println!("Finished processing message from machine: {}", log_message.machine_id);
                                    
                                    Response {
                                        status: "success".to_string(),
                                        message: "Log received".to_string(),
                                        timestamp: Utc::now().to_rfc3339()
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
 