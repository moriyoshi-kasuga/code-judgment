use crate::{Language, memory::Memory, state::RunnerState, time::MsTime};

#[derive(Debug, Clone, Hash, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct RunnerRequest {
    pub lang: Language,
    pub code: String,
    pub ms_time_limit: MsTime,
    pub memory_limit: Memory,
    pub stdin: String,
}

#[derive(Debug, Clone, Hash, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct RunnerResponse {
    pub state: RunnerState,
}
