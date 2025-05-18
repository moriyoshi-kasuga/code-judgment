use more_convert::VariantName;
use std::sync::LazyLock;

use env::{RUNNER_PATH, RUNNING_PATH};
use envman::EnvMan;
use runner_schema::{
    memory::Memory,
    state::RunnerState,
    time::MsTime,
    web::{RunnerRequest, RunnerResponse},
};

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

pub fn run(request: RunnerRequest) -> Result<RunnerResponse> {
    log::debug!("Started runner with request: {:?}", request);

    let lang_runner = lang::lang_into_runner(request.lang);
    let uid = ulid::Ulid::new();

    let current_dir = format!("{}/{}", RUNNING_PATH, uid);
    std::fs::create_dir(&current_dir)?;

    if let Some(file_name) = lang_runner.file_name() {
        std::fs::write(
            format!("{}/{}", current_dir, file_name),
            request.code.clone(),
        )?;
    }

    let bin_path = format!("{}/{}/bin", RUNNER_PATH, request.lang.variant_name());

    if let Some(compile_cmd) = lang_runner.compile_cmd() {
        log::debug!("Compile command: {}", compile_cmd);

        let child = std::process::Command::new("sh")
            .arg("-c")
            .arg(compile_cmd)
            .env("PATH", &bin_path)
            .current_dir(&current_dir)
            .spawn()?;

        let output = child.wait_with_output()?;
        if !output.status.success() {
            return Ok(RunnerResponse {
                state: RunnerState::CompileError {
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                },
            });
        }
    }

    let run_cmd = match lang_runner.run_cmd() {
        lang::RunCommand::WithCode { run_cmd } => run_cmd(&request.code),
        lang::RunCommand::Static { run_cmd } => run_cmd.to_string(),
    };

    log::debug!("Run command: {}", run_cmd);
    let child = std::process::Command::new("sh")
        .arg("-c")
        .arg(run_cmd)
        .env("PATH", &bin_path)
        .current_dir(&current_dir)
        .spawn()?;

    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Ok(RunnerResponse {
            state: RunnerState::RuntimeError {
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                exit_code: output.status.code().unwrap_or(137),

                // TODO: get memory usage and time elapsed
                max_memory_usage: Memory::new_bytes(0),
                ms_time_elapsed: MsTime::new_ms(0),
            },
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(RunnerResponse {
        state: RunnerState::Success {
            stdout,

            // TODO: get memory usage and time elapsed
            max_memory_usage: Memory::new_bytes(0),
            ms_time_elapsed: MsTime::new_ms(0),
        },
    })
}
