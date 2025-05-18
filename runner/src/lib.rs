use runner_schema::web::{RunnerRequest, RunnerResponse};

pub mod error;
pub use error::*;

pub fn runner(request: RunnerRequest) -> Result<RunnerResponse> {
    todo!();
}
