use std::process::Command;

use runner_schema::{memory::Memory, time::MsTime};

use crate::env::{NIX_BIN, NIX_STORE_PATH, NSJAIL_CMD};

pub struct NsJailBuilder<'a> {
    time_limit: Option<MsTime>,
    memory_limit: Option<Memory>,
    path: Option<&'a str>,
    cwd: Option<&'a str>,
}

impl<'a> Default for NsJailBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> NsJailBuilder<'a> {
    pub fn new() -> Self {
        NsJailBuilder {
            time_limit: None,
            memory_limit: None,
            path: None,
            cwd: None,
        }
    }

    pub fn time_limit(&mut self, time_limit: MsTime) -> &mut Self {
        self.time_limit = Some(time_limit);
        self
    }

    pub fn memory_limit(&mut self, memory_limit: Memory) -> &mut Self {
        self.memory_limit = Some(memory_limit);
        self
    }

    pub fn path(&mut self, path: &'a str) -> &mut Self {
        self.path = Some(path);
        self
    }

    pub fn cwd(&mut self, cwd: &'a str) -> &mut Self {
        self.cwd = Some(cwd);
        self
    }

    pub fn build(&self) -> Command {
        let mut command = Command::new(NSJAIL_CMD);
        self.write_args(&mut command);
        command
    }

    pub fn write(&self, command: &mut Command) {
        command.arg(NSJAIL_CMD);
        self.write_args(command);
    }

    fn write_args(&self, command: &mut Command) {
        command.arg("-Mo");
        command.arg("--user").arg("99999");
        command.arg("--group").arg("99999");
        command.arg("--max_cpus").arg("1");
        command.arg("--detect_cgroupv2");

        command
            .arg("--disable_proc")
            .arg("--disable_clone_newnet")
            .arg("--disable_clone_newuser")
            .arg("--disable_clone_newipc")
            .arg("--disable_clone_newuts")
            .arg("--disable_clone_newcgroup");

        if let Some(time_limit) = self.time_limit {
            command
                .arg("--time_limit")
                .arg(time_limit.as_seconds_ceil().to_string());
        }

        if let Some(memory_limit) = self.memory_limit {
            command
                .arg("--cgroup_mem_max")
                .arg(memory_limit.as_bytes().to_string());
            command
                .arg("--rlimit_as")
                .arg((memory_limit.as_megabytes() + 5).to_string());
        }

        if let Some(path) = self.path {
            command.arg("--env").arg(format!("PATH={}", path));
            command.arg("-R").arg(path);
        }

        if let Some(cwd) = self.cwd {
            command.arg("--chroot").arg(cwd);
            command.current_dir(cwd);
        } else {
            command.arg("--chroot").arg("/");
        }

        command.arg("-R").arg(NIX_STORE_PATH);
        command.arg("-R").arg(NIX_BIN);

        // command.arg("--log").arg("nsjail.txt");

        command.arg("--");
    }
}
