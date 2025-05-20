use lang::{LangExt, runner::RunCommand};
use nsjail::NsJailBuilder;
use std::{
    io::Write,
    path::{Path, PathBuf},
    process::Stdio,
};
use time::GTime;

use env::{RUNNING_PATH, SH_CMD};
use runner_schema::{
    state::RunnerState,
    web::{RunnerRequest, RunnerResponse},
};

pub mod lang;
pub mod nsjail;
pub mod time;

mod env;
pub use env::RunnerEnv;

pub mod error;
pub use error::{Error, Result};

pub fn run(request: RunnerRequest, option: &RunnerEnv) -> Result<RunnerResponse> {
    let uid = ulid::Ulid::new();
    log::debug!("Started runner {}: {:#?}", uid, request);

    let lang_runner = request.lang.into_runner();
    let uid = ulid::Ulid::new();

    let current_dir = create_dir_by_uid(uid)?;

    log::debug!("Starting runner in directory: {}", current_dir.display());

    if let Some(file_name) = lang_runner.file_name() {
        let path = current_dir.join(file_name);
        log::debug!("Writing to File: {}", path.display());
        std::fs::write(path, request.code.clone())?;
    }

    let lang_runner_path = request.lang.runner_path();
    let bin_path = request.lang.bin_path();

    if let Some(compile_cmd) = lang_runner.compile_cmd() {
        log::debug!("Compile command: {}", compile_cmd);

        let mut builder = NsJailBuilder::new_with(GTime::new_cmd());
        builder
            .time_limit(option.compile_time_limit_seconds)
            .memory_limit(option.compile_memory_limit_megabytes)
            .cwd(&current_dir)
            .env("PATH", &bin_path)
            .mount_read_only(&lang_runner_path)
            .tmpfsmount("/tmp")
            .writable();

        if let Some(option) = lang_runner.option() {
            for (key, value) in option.compile_env.iter() {
                builder.env(key, value);
            }
        }

        let mut command = builder.build();

        command
            .arg(SH_CMD)
            .arg("-c")
            .arg(compile_cmd)
            .stderr(Stdio::piped());
        log::debug!("Compile Command: {:?}", command);
        let child = command.spawn()?;

        let output = child.wait_with_output()?;
        let (memory, time) = GTime::read(&current_dir)?;
        log::debug!("Compile Memory: {:?}, Time: {:?}", memory, time);
        if !output.status.success() {
            return Ok(RunnerResponse {
                state: RunnerState::CompileError {
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                },
            });
        }
    }

    let run_cmd = match lang_runner.run_cmd() {
        RunCommand::WithCode { run_cmd } => run_cmd(&request.code),
        RunCommand::Static { run_cmd } => run_cmd.to_string(),
    };

    let mut builder = NsJailBuilder::new_with(GTime::new_cmd());
    builder
        .env("PATH", &bin_path)
        .mount_read_only(&lang_runner_path)
        .time_limit(request.ms_time_limit.add_seconds(1))
        .memory_limit(request.memory_limit.add_megabytes(1))
        .cwd(&current_dir);
    let mut command = builder.build();
    command.arg(SH_CMD).arg("-c").arg(run_cmd);
    log::debug!("Run command: {:?}", command);
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
    log::debug!("Run Memory: {:?}, Time: {:?}", memory, time);

    if time > request.ms_time_limit {
        return Ok(RunnerResponse {
            state: RunnerState::Timeout {
                ms_time_elapsed: time,
            },
        });
    }

    if memory > request.memory_limit {
        return Ok(RunnerResponse {
            state: RunnerState::MemoryLimit {
                max_memory_usage: memory,
            },
        });
    }

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

fn create_dir_by_uid(uid: ulid::Ulid) -> Result<PathBuf> {
    let current_dir = Path::new(RUNNING_PATH).join(uid.to_string());
    std::fs::create_dir(&current_dir)?;
    std::os::unix::fs::chown(&current_dir, Some(99999), Some(99999))?;
    Ok(current_dir)
}
