use std::io::Result;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src/admin/v1/req.proto");
    println!("cargo:rerun-if-changed=build.rs");
    prost_build::Config::new()
        .out_dir("src/admin/v1/")
        .compile_protos(&["src/admin/v1/req.proto"], &["src"])?;
    Ok(())
}
