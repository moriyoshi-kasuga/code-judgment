#![allow(clippy::unwrap_used)]

use more_convert::VariantName;

pub const RUNNER_PATH: &str = "/runner";

fn main() {
    let builds = runner_schema::Language::VARIANTS
        .iter()
        .map(|lang| {
            let name = lang.variant_name();
            format!(
                "nix-build /default.nix -A {} --out-link {}/{}",
                name, RUNNER_PATH, name
            )
        })
        .collect::<Vec<_>>();

    let build = builds.join(" && \n  ");

    std::fs::write(
        "build.sh",
        format!(
            "#!/usr/bin/env bash
set -e
set -x

{}
",
            build
        ),
    )
    .unwrap();
}
