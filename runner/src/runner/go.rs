use std::{path::PathBuf, process::Command};

use runner_schema::Language;

use crate::{env::PERMISSION_ID, lang::LangExt};

use super::{LangRunner, LangRunnerOption};

pub fn go() -> LangRunner {
    const GOCACHE: &str = "/go-cache";
    const GO: &str = r#"package main
import "fmt"
func main() {
    fmt.Println("Hello, World!")
}"#;

    std::fs::create_dir_all(GOCACHE).expect("Failed to create GOCACHE directory");
    std::os::unix::fs::chown(GOCACHE, Some(PERMISSION_ID), Some(PERMISSION_ID))
        .expect("Failed to set ownership of GOCACHE directory");

    let go_bin = PathBuf::from(Language::Go1_23.bin_path()).join("go");

    let temp_dir = std::env::temp_dir();
    let go_main_result = temp_dir.join("go-cache-main");
    let go_main = temp_dir.join("go-cache-main.go");
    std::fs::write(&go_main, GO).expect("Failed to write go cache file");

    let status = Command::new(go_bin)
        .arg("build")
        .arg("-o")
        .arg(&go_main_result)
        .arg(&go_main)
        .env("GOCACHE", GOCACHE)
        .current_dir(temp_dir)
        .status()
        .expect("Failed to compile go cache file");

    if !status.success() {
        panic!("Failed to compile go cache file");
    }

    log::debug!(
        "Go cache file compiled successfully: {}",
        go_main_result.display()
    );

    LangRunner::WithCompile {
        file_name: "main.go",
        compile_cmd: "go build -o main main.go",
        run_cmd: "./main",
        option: LangRunnerOption {
            more_compile: Some(|builder| {
                builder.env("GOCACHE", GOCACHE).mount_rw(GOCACHE);
            }),
            ..Default::default()
        },
    }
}
