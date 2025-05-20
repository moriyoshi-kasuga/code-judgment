use std::{path::Path, process::Command};

use runner_schema::{memory::Memory, time::MsTime};

use crate::env::{NIX_BIN, NIX_STORE_PATH, NSJAIL_CMD};

pub struct NsJailBuilder {
    command: Command,
}

impl Default for NsJailBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl NsJailBuilder {
    pub fn new() -> Self {
        let mut command = Command::new(NSJAIL_CMD);
        Self::write_args(&mut command);
        NsJailBuilder { command }
    }

    pub fn new_with(mut command: Command) -> Self {
        command.arg(NSJAIL_CMD);
        Self::write_args(&mut command);
        NsJailBuilder { command }
    }

    pub fn time_limit(&mut self, time_limit: MsTime) -> &mut Self {
        self.command
            .arg("--time_limit")
            .arg(time_limit.as_seconds_ceil().to_string());

        self
    }

    pub fn memory_limit(&mut self, memory_limit: Memory) -> &mut Self {
        self.command
            .arg("--cgroup_mem_max")
            .arg(memory_limit.as_bytes().to_string());

        self
    }

    pub fn env(&mut self, key: &str, value: &str) -> &mut Self {
        self.command.arg("--env").arg(format!("{}={}", key, value));

        self
    }

    pub fn cwd(&mut self, cwd: &Path) -> &mut Self {
        self.command.arg("--chroot").arg(cwd);
        self.command.current_dir(cwd);

        self
    }

    pub fn tmpfsmount(&mut self, tmpfsmount: &str) -> &mut Self {
        self.command.arg("--tmpfsmount").arg(tmpfsmount);
        self.command
            .arg("--env")
            .arg(format!("TMPDIR={}", tmpfsmount));

        self
    }

    pub fn mount_read_only(&mut self, path: &str) -> &mut Self {
        self.command.arg("-R").arg(path);

        self
    }

    pub fn writable(&mut self) -> &mut Self {
        self.command.arg("--rw");

        self
    }

    pub fn build(self) -> Command {
        let mut command = self.command;

        command.arg("--");

        command
    }

    fn write_args(command: &mut Command) {
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

        // virtual memory by 4096MB
        command.arg("--rlimit_as").arg("4096");

        command.arg("-R").arg(NIX_STORE_PATH);
        command.arg("-R").arg(NIX_BIN);

        command.arg("--log").arg("nsjail.txt");
    }
}
