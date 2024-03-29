use std::io::Result;

fn main() -> Result<()> {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=api.proto");

    let mut conf = prost_build::Config::new();
    conf.service_generator(Box::new(dcl_rpc::codegen::RPCServiceGenerator::new()));
    conf.compile_protos(&["api.proto"], &["../"])?;
    Ok(())
}
