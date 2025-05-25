use std::{ffi::OsStr, path::Path, process::Command};

use runner_schema::{memory::Memory, time::MsTime};

use crate::env::{NIX_BIN, NIX_STORE_PATH, NSJAIL_CMD, PERMISSION_ID_STR};

pub struct NsJailBuilder {
    command: Command,
    proc_writable: Option<bool>,
}

impl Default for NsJailBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl NsJailBuilder {
    fn inner(command: Command) -> Self {
        NsJailBuilder {
            command,
            proc_writable: None,
        }
    }

    pub fn new() -> Self {
        let mut command = Command::new(NSJAIL_CMD);
        Self::write_args(&mut command);
        NsJailBuilder::inner(command)
    }

    pub fn new_with(mut command: Command) -> Self {
        command.arg(NSJAIL_CMD);
        Self::write_args(&mut command);
        NsJailBuilder::inner(command)
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

    pub fn tmpfsmount(&mut self, tmpfsmount: &str, memory: Memory) -> &mut Self {
        self.command.arg("-m").arg(format!(
            "none:{tmpfsmount}:tmpfs:size={}",
            memory.as_bytes()
        ));

        self.command
            .arg("--env")
            .arg(format!("TMPDIR={}", tmpfsmount));

        self
    }

    /// mount read only
    pub fn mount_ro(&mut self, path: &str) -> &mut Self {
        self.mount_ro_dest(path, path)
    }

    /// mount read only
    pub fn mount_ro_dest(&mut self, src: &str, dest: &str) -> &mut Self {
        self.command.arg("-R").arg(format!("{src}:{dest}"));

        self
    }

    // mount read write
    pub fn mount_rw(&mut self, path: &str) -> &mut Self {
        self.mount_rw_dest(path, path)
    }

    // mount read write
    pub fn mount_rw_dest(&mut self, src: &str, dest: &str) -> &mut Self {
        self.command.arg("-B").arg(format!("{src}:{dest}"));

        self
    }

    pub fn writable(&mut self) -> &mut Self {
        self.command.arg("--rw");

        self
    }

    pub fn arg(&mut self, arg: impl AsRef<OsStr>) -> &mut Self {
        self.command.arg(arg);

        self
    }

    pub fn proc_writable(&mut self, is_writable: bool) -> &mut Self {
        self.proc_writable = Some(is_writable);

        self
    }

    pub fn log(&mut self, log_path: &str) -> &mut Self {
        self.command.arg("--log").arg(log_path);

        self
    }

    pub fn build(self) -> Command {
        let mut command = self.command;

        match self.proc_writable {
            Some(true) => {
                command.arg("--proc_rw");
            }
            Some(false) => {}
            None => {
                command.arg("--disable_proc");
            }
        };

        command.arg("--");

        command
    }

    fn write_args(command: &mut Command) {
        command.arg("-Mo");
        command.arg("--user").arg(PERMISSION_ID_STR);
        command.arg("--group").arg(PERMISSION_ID_STR);
        command.arg("--detect_cgroupv2");
        command.arg("--bindmount_ro").arg("/dev/null");

        command
            .arg("--disable_clone_newnet")
            .arg("--disable_clone_newuser")
            .arg("--disable_clone_newipc")
            .arg("--disable_clone_newuts")
            .arg("--disable_clone_newcgroup");

        // virtual memory by MB
        command.arg("--rlimit_as").arg("9192");

        command.arg("-R").arg(NIX_STORE_PATH);
        command.arg("-R").arg(NIX_BIN);
    }
}
