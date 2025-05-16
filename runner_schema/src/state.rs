use std::time::Duration;

use crate::memory::Memory;

#[derive(Debug, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub enum RunnerState {
    Success {
        stdout: String,

        max_memory_usage: Memory,
        time_elapsed: Duration,
    },
    RuntimeError {
        stderr: String,
        exit_code: i32,

        max_memory_usage: Memory,
        time_elapsed: Duration,
    },
    Timeout {
        duration: Duration,
    },
    MemoryLimit {
        memory: Memory,
    },
    CompileError {
        stderr: String,
    },
    InternalError,
}
