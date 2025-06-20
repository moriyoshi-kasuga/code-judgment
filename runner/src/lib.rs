use env::{PERMISSION_ID, RUNNING_PATH, RunnerOption, SH_CMD};
use lang::LangExt;
use nsjail::NsJailBuilder;
use runner::{RunCommand, Runners};
use std::{
    io::Write,
    path::{Path, PathBuf},
    process::Stdio,
};
use time::GTime;

use runner_schema::{
    memory::Memory,
    state::RunnerState,
    web::{RunnerRequest, RunnerResponse},
};

pub mod lang;
pub mod nsjail;
pub mod runner;
pub mod time;

pub mod env;

pub mod error;
pub use error::{Error, Result};

pub fn run(
    runners: &Runners,
    request: RunnerRequest,
    option: &RunnerOption,
) -> Result<RunnerResponse> {
    let uid = ulid::Ulid::new();
    log::debug!("Started runner {}: {:#?}", uid, request);

    let lang_runner = runners.get(&request.lang);
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
            .proc_writable(true)
            .arg("--rlimit_fsize")
            .arg("100")
            .arg("--rlimit_nofile")
            .arg("128")
            .cwd(&current_dir)
            .env("PATH", &bin_path)
            .mount_ro(&lang_runner_path)
            .tmpfsmount("/tmp", Memory::new_megabytes(512))
            .writable();

        if let Some(f) = lang_runner.option().more_compile {
            f(&mut builder)
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
        // TODO: compile time limit and memory limit check
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
        .mount_ro(&lang_runner_path)
        .time_limit(request.ms_time_limit.add_seconds(1))
        .memory_limit(request.memory_limit.add_megabytes(1))
        .log("nsjail.log")
        .cwd(&current_dir);

    if let Some(f) = lang_runner.option().more_run {
        f(&mut builder)
    }

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
    std::os::unix::fs::chown(&current_dir, Some(PERMISSION_ID), Some(PERMISSION_ID))?;
    Ok(current_dir)
}
