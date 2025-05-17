use std::time::Duration;

use crate::{Language, memory::Memory, state::RunnerState};
tonic::include_proto!("runner");

#[derive(Debug, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RunnerRequest {
    pub lang: Language,
    pub code: String,
    pub timeout: Duration,
    pub memory_limit: Memory,
    pub stdin: Option<String>,
}

#[derive(Debug, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RunnerResponse {
    pub state: RunnerState,
}
