use more_convert::VariantName;
use runner_schema::Language;

use crate::env::RUNNER_PATH;

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
