use std::{path::Path, process::Command};

use runner_schema::{memory::Memory, time::MsTime};

use crate::env::TIME_CMD;

pub struct GTime;

impl GTime {
    pub const TIME_TXT: &'static str = "time.txt";

    pub fn new_cmd() -> Command {
        let mut command = Command::new(TIME_CMD);
        Self::write_args(&mut command);
        command
    }

    pub fn write(command: &mut Command) {
        command.arg(TIME_CMD);
        Self::write_args(command);
    }

    fn write_args(command: &mut Command) {
        command
            .arg("--quiet")
            .arg("--format")
            .arg("%M\n%E")
            .arg("--output")
            .arg(Self::TIME_TXT);
    }

    pub fn read(parent_dir: impl AsRef<Path>) -> std::io::Result<(Memory, MsTime)> {
        let path = parent_dir.as_ref().join(Self::TIME_TXT);
        let time_txt = std::fs::read_to_string(path)?;
        let mut lines = time_txt.lines();
        fn invalid_data(message: &str) -> std::io::Error {
            std::io::Error::new(std::io::ErrorKind::InvalidData, message)
        }
        let memory = lines
            .next()
            .ok_or_else(|| invalid_data("Missing memory line"))?;
        let memory = memory
            .parse::<u64>()
            .map_err(|_| invalid_data("Invalid memory line"))?;

        let memory = Memory::new_kilobytes(memory);
        let time = lines
            .next()
            .ok_or_else(|| invalid_data("Missing time line"))?;
        let time =
            MsTime::from_str_mm_ss_ms(time).ok_or_else(|| invalid_data("Invalid time line"))?;

        Ok((memory, time))
    }
}
