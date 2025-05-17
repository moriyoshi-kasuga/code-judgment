#![allow(clippy::unwrap_used)]

fn main() {
    tonic_build::compile_protos("proto/runner.proto").unwrap();
}
