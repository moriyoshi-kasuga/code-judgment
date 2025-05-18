use runner_schema::Language;

pub enum LangRunner {
    WithCompile {
        file_name: &'static str,
        compile_cmd: &'static str,
        run_cmd: &'static str,
    },
    WithoutCompile {
        file_name: &'static str,
        run_cmd: &'static str,
    },
    Inline {
        run_cmd: &'static str,
    },
}

pub fn lang_into_runner(lang: Language) -> &'static LangRunner {
    match lang {
        Language::Rust1_82 => &LangRunner::WithCompile {
            file_name: "main.rs",
            compile_cmd: "rustc --release main.rs -o main",
            run_cmd: "./main",
        },
        Language::Go1_23 => &LangRunner::WithCompile {
            file_name: "main.go",
            compile_cmd: "go build -o main main.go",
            run_cmd: "./main",
        },
        Language::Python3_13 => &LangRunner::WithoutCompile {
            file_name: "main.py",
            run_cmd: "python3 main.py",
        },
    }
}
