use std::process::Command;

use runner_schema::{memory::Memory, time::MsTime};

use crate::env::{NSJAIL_CMD, SH_CMD};

pub struct NsJailBuilder<'a> {
    target_command: &'a str,
    time_limit: Option<MsTime>,
    memory_limit: Option<Memory>,
    path: Option<&'a str>,
    cwd: Option<&'a str>,
}

impl<'a> NsJailBuilder<'a> {
    pub fn new(target_command: &'a str) -> Self {
        NsJailBuilder {
            target_command,
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
        command.arg("-Mo");
        command.arg("--user").arg("99999");
        command.arg("--group").arg("99999");
        command.arg("--chroot").arg("/");
        command.arg("--max_cpus").arg("1");
        command.arg("--detect_cgroupv2");

        command
            .arg("--disable_proc")
            .arg("--disable_clone_newnet")
            .arg("--disable_clone_newuser")
            .arg("--disable_clone_newns")
            .arg("--disable_clone_newpid")
            .arg("--disable_clone_newipc")
            .arg("--disable_clone_newuts")
            .arg("--disable_clone_newcgroup");

        if let Some(time_limit) = self.time_limit {
            command
                .arg("--time_limit")
                .arg(time_limit.as_seconds().ceil().to_string());
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
        }

        if let Some(cwd) = self.cwd {
            command.arg("--cwd").arg(cwd);
        }

        command
            .arg("--")
            .arg(SH_CMD)
            .arg("-c")
            .arg(self.target_command);

        log::debug!("NsJail command: {:?}", command);

        command
    }
}
