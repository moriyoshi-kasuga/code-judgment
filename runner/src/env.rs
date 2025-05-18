#[derive(Debug, envman::EnvMan)]
pub struct RunnerEnv {
    #[envman(default = true)]
    pub use_cgroup_v2: bool,
}
