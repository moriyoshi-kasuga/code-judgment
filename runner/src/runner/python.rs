use super::LangRunner;

pub fn python() -> LangRunner {
    LangRunner::WithoutCompile {
        file_name: "main.py",
        run_cmd: "python main.py",
        option: Default::default(),
    }
}
