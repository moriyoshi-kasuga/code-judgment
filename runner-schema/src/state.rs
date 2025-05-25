use crate::{memory::Memory, time::MsTime};

#[derive(Debug, Clone, Hash, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum RunnerState {
    Success {
        stdout: String,

        max_memory_usage: Memory,
        ms_time_elapsed: MsTime,
    },
    RuntimeError {
        stderr: String,
        exit_code: i32,

        max_memory_usage: Memory,
        ms_time_elapsed: MsTime,
    },
    Timeout {
        ms_time_elapsed: MsTime,
    },
    MemoryLimit {
        max_memory_usage: Memory,
    },
    CompileError {
        stderr: String,
    },
    InternalError,
}
