use runner_schema::{memory::Memory, time::MsTime};

#[derive(Debug, envman::EnvMan)]
pub struct RunnerOption {
    #[envman(parser = compile_time_limit_seconds)]
    pub compile_time_limit_seconds: MsTime,
    #[envman(parser = compile_memory_limit_megabytes)]
    pub compile_memory_limit_megabytes: Memory,
}

fn compile_time_limit_seconds(value: &str) -> Result<MsTime, <u64 as std::str::FromStr>::Err> {
    value.parse::<u64>().map(MsTime::new_seconds)
}

fn compile_memory_limit_megabytes(value: &str) -> Result<Memory, <u64 as std::str::FromStr>::Err> {
    value.parse::<u64>().map(Memory::new_megabytes)
}

pub const RUNNER_PATH: &str = env!("RUNNER_PATH");
pub const RUNNING_PATH: &str = env!("RUNNING_PATH");
pub const NIX_STORE_PATH: &str = env!("NIX_STORE_PATH");
pub const NIX_BIN: &str = env!("NIX_BIN");
pub const SH_CMD: &str = concat!(env!("NIX_BIN"), "/sh");
pub const NSJAIL_CMD: &str = concat!(env!("NIX_BIN"), "/nsjail");
pub const TIME_CMD: &str = concat!(env!("NIX_BIN"), "/time");

pub const PERMISSION_ID_STR: &str = env!("PERMISSION_ID_STR");
pub const PERMISSION_ID: u32 = match u32::from_str_radix(PERMISSION_ID_STR, 10) {
    Ok(id) => id,
    Err(_) => panic!("Invalid permission ID"),
};
