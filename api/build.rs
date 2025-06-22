fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 告诉cargo在proto文件修改时重新构建
    println!("cargo:rerun-if-changed=proto");
    println!("cargo:rerun-if-changed=build.rs");
    let files = &["idl/admin/v1/api.proto"];
    let includes = &["idl", "."];


    let mut prost_config = prost_build::Config::new();
    prost_config
        .out_dir("src/admin")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"PascalCase\")]");

    let mut config = prost_validate_build::Builder::new();
    
    config.compile_protos_with_config(prost_config, files, includes)?;


    Ok(())
}
