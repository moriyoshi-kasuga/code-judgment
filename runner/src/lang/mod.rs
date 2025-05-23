use more_convert::VariantName;
use runner_schema::Language;

use crate::env::RUNNER_PATH;

pub mod runner;

pub trait LangExt {
    fn runner_path(&self) -> String;
    fn bin_path(&self) -> String {
        format!("{}/bin", self.runner_path())
    }
}

impl LangExt for Language {
    fn runner_path(&self) -> String {
        format!("{}/{}", RUNNER_PATH, self.variant_name())
    }
}

pub struct Runners {
    map: enum_table::EnumTable<Language, runner::LangRunner, { Language::COUNT }>,
}

impl Runners {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let map = enum_table::EnumTable::new_with_fn(runner::lang_into_runner);

        Self { map }
    }

    pub fn get(&self, lang: &Language) -> &runner::LangRunner {
        self.map.get(lang)
    }
}
