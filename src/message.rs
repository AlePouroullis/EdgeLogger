use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMessage {
    pub machine_id: String,
    pub timestamp: String, 
    pub metrics: HashMap<String, f64>
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub status: String,
    pub message: String,
    pub timestamp: String
}