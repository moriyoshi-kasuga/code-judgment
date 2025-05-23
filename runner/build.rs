#![allow(clippy::unwrap_used)]

use more_convert::VariantName;

pub const RUNNER_PATH: &str = "/runner";
pub const RUNNING_PATH: &str = "/running";
pub const NIX_STORE_PATH: &str = "/nix/store";
pub const NIX_BIN: &str = "/global/bin";
pub const PERMISSION_ID_STR: &str = "99999";

fn main() {
    println!("cargo:rustc-env=RUNNER_PATH={}", RUNNER_PATH);
    println!("cargo:rustc-env=RUNNING_PATH={}", RUNNING_PATH);
    println!("cargo:rustc-env=NIX_STORE_PATH={}", NIX_STORE_PATH);
    println!("cargo:rustc-env=NIX_BIN={}", NIX_BIN);
    println!("cargo:rustc-env=PERMISSION_ID_STR={}", PERMISSION_ID_STR);

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

    let build = builds.join(" && \\ \n  ");

    let mkdirs = format!(
        "mkdir -p {RUNNING_PATH} && \
  mkdir -p {RUNNER_PATH} && \
  chown -R {PERMISSION_ID_STR}:{PERMISSION_ID_STR} {RUNNING_PATH}"
    );

    let text = format!(
        "# This file is auto-generated. Do not edit it directly.
RUN {mkdirs} && \\\n  {build}
",
    );

    if let Ok(read_text) = std::fs::read("Dockerfile.build") {
        if read_text == text.as_bytes() {
            return;
        }
    }

    std::fs::write("Dockerfile.build", text).unwrap();
}
