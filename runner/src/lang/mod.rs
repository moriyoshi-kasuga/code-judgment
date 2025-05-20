use more_convert::VariantName;
use runner::LangRunner;
use runner_schema::Language;

use crate::env::RUNNER_PATH;

pub mod runner;

pub trait LangExt {
    fn runner_path(&self) -> String;
    fn bin_path(&self) -> String {
        format!("{}/bin", self.runner_path())
    }

    fn into_runner(self) -> LangRunner;
}

impl LangExt for Language {
    fn runner_path(&self) -> String {
        format!("{}/{}", RUNNER_PATH, self.variant_name())
    }

    fn into_runner(self) -> LangRunner {
        runner::lang_into_runner(self)
    }
}
