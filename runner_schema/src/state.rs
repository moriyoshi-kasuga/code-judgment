use crate::{memory::Memory, time::MsTime};

#[derive(Debug, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub enum RunnerState {
    Success {
        stdout: String,

        max_memory_usage: Memory,
        time_elapsed: MsTime,
    },
    RuntimeError {
        stderr: String,
        exit_code: i32,

        max_memory_usage: Memory,
        time_elapsed: MsTime,
    },
    Timeout {
        duration: MsTime,
    },
    MemoryLimit {
        memory: Memory,
    },
    CompileError {
        stderr: String,
    },
    InternalError,
}
