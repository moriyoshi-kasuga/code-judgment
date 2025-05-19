#[derive(Debug, envman::EnvMan)]
pub struct RunnerEnv {}

pub const RUNNER_PATH: &str = "/runner";
pub const RUNNING_PATH: &str = "/running";
pub const SH_CMD: &str = "/root/.nix-profile/bin/sh";
pub const NSJAIL_CMD: &str = "/root/.nix-profile/bin/nsjail";
pub const TIME_CMD: &str = "/root/.nix-profile/bin/time";
