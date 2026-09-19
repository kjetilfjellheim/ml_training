use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthResponse {
    pub status: &'static str,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GreetingResponse {
    pub message: String,
}
