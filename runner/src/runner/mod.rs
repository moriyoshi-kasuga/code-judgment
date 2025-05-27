#![deny(clippy::panic)]

use runner_schema::Language;

use crate::nsjail::NsJailBuilder;

mod go;
mod python;
mod rust;

pub struct Runners {
    map: enum_table::EnumTable<Language, LangRunner, { Language::COUNT }>,
}

impl Runners {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Result<Self, (&'static Language, Box<dyn std::error::Error>)> {
        let map = enum_table::EnumTable::try_new_with_fn(lang_into_runner)?;

        Ok(Self { map })
    }

    pub fn get(&self, lang: &Language) -> &LangRunner {
        self.map.get(lang)
    }
}

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

pub(super) fn lang_into_runner(lang: &Language) -> Result<LangRunner, Box<dyn std::error::Error>> {
    Ok(match lang {
        Language::Rust1_82 => rust::rust(),
        Language::Go1_23 => go::go()?,
        Language::Python3_13 => python::python(),
    })
}
