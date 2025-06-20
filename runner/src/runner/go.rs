use runner_schema::{Language, memory::Memory};

use crate::{
    env::{PERMISSION_ID, SH_CMD},
    lang::LangExt,
    nsjail::NsJailBuilder,
};

use super::{LangRunner, LangRunnerOption};

pub fn go() -> Result<LangRunner, Box<dyn std::error::Error>> {
    const GOCACHE: &str = "/go-cache";
    const GO: &str = r#"
package main
import "fmt"
func main() {
    fmt.Println("Hello, Go!")
}
"#;

    std::fs::create_dir_all(GOCACHE)?;
    std::os::unix::fs::chown(GOCACHE, Some(PERMISSION_ID), Some(PERMISSION_ID))?;

    let temp_dir = std::env::temp_dir();
    let go_main = temp_dir.join("go-cache-main.go");
    std::fs::write(&go_main, GO)?;

    let lang_runner_path = Language::Go1_23.runner_path();
    let bin_path = Language::Go1_23.bin_path();

    let mut builder = NsJailBuilder::new();
    builder
        .mount_rw(GOCACHE)
        .proc_writable(true)
        .arg("--rlimit_fsize")
        .arg("100")
        .arg("--rlimit_nofile")
        .arg("128")
        .cwd(&temp_dir)
        .env("PATH", &bin_path)
        .mount_ro(&lang_runner_path)
        .mount_rw(GOCACHE)
        .env("GOCACHE", GOCACHE)
        .tmpfsmount("/tmp", Memory::new_megabytes(512))
        .writable();

    let mut command = builder.build();

    let output = command
        .arg(SH_CMD)
        .arg("-c")
        .arg("go build -o go-cache-main go-cache-main.go")
        .output()?;

    if !output.status.success() {
        return Err(Box::new(std::io::Error::other(format!(
            "Failed to compile Go cache file: {}",
            String::from_utf8_lossy(&output.stderr)
        ))));
    }

    Ok(LangRunner::WithCompile {
        file_name: "main.go",
        compile_cmd: "go build -o main main.go",
        run_cmd: "./main",
        option: LangRunnerOption {
            more_compile: Some(|builder| {
                builder.env("GOCACHE", GOCACHE).mount_rw(GOCACHE);
            }),
            ..Default::default()
        },
    })
}
