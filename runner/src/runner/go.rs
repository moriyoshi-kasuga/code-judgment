use super::{LangRunner, LangRunnerOption};

pub fn go() -> LangRunner {
    LangRunner::WithCompile {
        file_name: "main.go",
        compile_cmd: "go build -o main main.go",
        run_cmd: "./main",
        option: LangRunnerOption {
            more_compile: Some(|builder| {
                builder
                    .env("HOME", "/")
                    .mount_rw_dest("/go-cache", "/.cache");
            }),
            ..Default::default()
        },
    }
}
