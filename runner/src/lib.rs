use more_convert::VariantName;
use nsjail::NsJailBuilder;
use std::{io::Write, process::Stdio, sync::LazyLock};
use time::GTime;

use env::{RUNNER_PATH, RUNNING_PATH, SH_CMD};
use envman::EnvMan;
use runner_schema::{
    memory::Memory,
    state::RunnerState,
    time::MsTime,
    web::{RunnerRequest, RunnerResponse},
};

pub mod lang;
pub mod nsjail;
pub mod time;

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

    log::debug!("Starting runner in directory: {}", current_dir);

    if let Some(file_name) = lang_runner.file_name() {
        std::fs::write(
            format!("{}/{}", current_dir, file_name),
            request.code.clone(),
        )?;
    }

    let bin_path = format!("{}/{}/bin", RUNNER_PATH, request.lang.variant_name());

    log::debug!("Bin path: {}", bin_path);

    if let Some(compile_cmd) = lang_runner.compile_cmd() {
        log::debug!("Compile command: {}", compile_cmd);

        let child = NsJailBuilder::new()
            .path(&bin_path)
            .time_limit(MsTime::new_seconds(5))
            .memory_limit(Memory::new_megabytes(128))
            .cwd(&current_dir)
            .build()
            .arg(SH_CMD)
            .arg("-c")
            .arg(compile_cmd)
            .stderr(Stdio::piped())
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
    let mut command = GTime::new_cmd();
    NsJailBuilder::new()
        .path(&bin_path)
        .time_limit(request.ms_time_limit)
        .memory_limit(request.memory_limit)
        .cwd(&current_dir)
        .write(&mut command);
    command.arg(run_cmd);
    log::debug!("Command: {:?}", command);
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdin = child
        .stdin
        .as_mut()
        .ok_or_else(|| Error::IO(std::io::Error::other("Failed to open stdin")))?;

    stdin.write_all(request.stdin.as_bytes())?;

    let output = child.wait_with_output()?;
    let (memory, time) = GTime::read(&current_dir)?;

    if !output.status.success() {
        return Ok(RunnerResponse {
            state: RunnerState::RuntimeError {
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                exit_code: output.status.code().unwrap_or(137),

                max_memory_usage: memory,
                ms_time_elapsed: time,
            },
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(RunnerResponse {
        state: RunnerState::Success {
            stdout,

            max_memory_usage: memory,
            ms_time_elapsed: time,
        },
    })
}
