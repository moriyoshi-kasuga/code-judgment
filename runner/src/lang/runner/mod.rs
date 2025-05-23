use runner_schema::Language;

use crate::nsjail::NsJailBuilder;

pub enum LangRunner {
    WithCompile {
        file_name: &'static str,
        compile_cmd: &'static str,
        run_cmd: &'static str,
        option: LangRunnerOption,
    },
    WithoutCompile {
        file_name: &'static str,
        run_cmd: &'static str,
        option: LangRunnerOption,
    },
    Inline {
        run_cmd: fn(&str) -> String,
        option: LangRunnerOption,
    },
}

#[derive(Default)]
pub struct LangRunnerOption {
    pub more_compile: Option<fn(&mut NsJailBuilder)>,
    pub more_run: Option<fn(&mut NsJailBuilder)>,
}

pub enum RunCommand {
    WithCode { run_cmd: fn(&str) -> String },
    Static { run_cmd: &'static str },
}

impl LangRunner {
    pub fn file_name(&self) -> Option<&'static str> {
        match self {
            LangRunner::WithCompile { file_name, .. } => Some(file_name),
            LangRunner::WithoutCompile { file_name, .. } => Some(file_name),
            LangRunner::Inline { .. } => None,
        }
    }

    pub fn compile_cmd(&self) -> Option<&'static str> {
        match self {
            LangRunner::WithCompile { compile_cmd, .. } => Some(compile_cmd),
            LangRunner::WithoutCompile { .. } => None,
            LangRunner::Inline { .. } => None,
        }
    }

    pub fn run_cmd(&self) -> RunCommand {
        match self {
            LangRunner::WithCompile { run_cmd, .. } => RunCommand::Static { run_cmd },
            LangRunner::WithoutCompile { run_cmd, .. } => RunCommand::Static { run_cmd },
            LangRunner::Inline { run_cmd, .. } => RunCommand::WithCode { run_cmd: *run_cmd },
        }
    }

    pub fn option(&self) -> &LangRunnerOption {
        match self {
            LangRunner::WithCompile { option, .. } => option,
            LangRunner::WithoutCompile { option, .. } => option,
            LangRunner::Inline { option, .. } => option,
        }
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
pub(super) fn lang_into_runner(lang: &Language) -> LangRunner {
    match lang {
        Language::Rust1_82 => LangRunner::WithCompile {
            file_name: "main.rs",
            compile_cmd: "rustc -O main.rs -o main",
            run_cmd: "./main",
            option: Default::default(),
        },
        Language::Go1_23 => LangRunner::WithCompile {
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
        },
        Language::Python3_13 => LangRunner::WithoutCompile {
            file_name: "main.py",
            run_cmd: "python main.py",
            option: Default::default(),
        },
    }
}
