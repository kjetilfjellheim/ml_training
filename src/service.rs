use crate::api_contract::{GreetingResponse, HealthResponse};
use std::sync::Arc;

pub trait ApiService: Send + Sync {
    fn health(&self) -> HealthResponse;
    fn greeting(&self, name: &str) -> GreetingResponse;
}

pub type SharedApiService = Arc<dyn ApiService>;

#[derive(Debug, Default)]
pub struct DefaultApiService;

impl ApiService for DefaultApiService {
    fn health(&self) -> HealthResponse {
        HealthResponse { status: "ok" }
    }

    fn greeting(&self, name: &str) -> GreetingResponse {
        GreetingResponse {
            message: format!("Hello, {}!", name),
        }
    }
}

pub fn default_service() -> SharedApiService {
    Arc::new(DefaultApiService)
}
