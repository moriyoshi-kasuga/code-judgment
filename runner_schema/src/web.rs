use crate::{Language, memory::Memory, state::RunnerState, time::MsTime};

tonic::include_proto!("runner");

#[derive(Debug, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RunnerRequest {
    pub lang: Language,
    pub code: String,
    pub ms_time_limit: MsTime,
    pub memory_limit: Memory,
    pub stdin: Option<String>,
}

#[derive(Debug, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RunnerResponse {
    pub state: RunnerState,
}
