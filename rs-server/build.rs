extern crate prost_build;
use std::io::Result;

fn main() -> Result<()> {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=api.proto");

    prost_build::compile_protos(&["api.proto"], &["../"])?;
    Ok(())
}
