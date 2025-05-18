#[derive(Debug, envman::EnvMan)]
pub struct RunnerEnv {
    #[envman(default = true)]
    pub use_cgroup_v2: bool,
}

pub const RUNNER_PATH: &str = "/runner";
pub const RUNNING_PATH: &str = "/running";
