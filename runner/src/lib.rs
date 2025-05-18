use std::sync::LazyLock;

use envman::EnvMan;
use runner_schema::web::{RunnerRequest, RunnerResponse};

pub mod lang;

mod env;
pub use env::RunnerEnv;

pub mod error;
pub use error::{Error, Result};

static RUNNER_ENV: LazyLock<RunnerEnv> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    let env = RunnerEnv::load().unwrap();
    log::info!("Loaded env: {:#?}", env);
    env
});

pub fn runner(request: RunnerRequest) -> Result<RunnerResponse> {
    todo!();
}
