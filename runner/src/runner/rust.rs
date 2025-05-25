use super::LangRunner;

pub fn rust() -> LangRunner {
    LangRunner::WithCompile {
        file_name: "main.rs",
        compile_cmd: "rustc -O main.rs -o main",
        run_cmd: "./main",
        option: Default::default(),
    }
}
