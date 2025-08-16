use pilota_build::rir::Item;
use pilota_build::{Context, DefId};
use std::path::PathBuf;
use std::sync::Arc;
use std::vec::Vec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 告诉cargo在proto文件修改时重新构建
    println!("cargo:rerun-if-changed=proto");
    println!("cargo:rerun-if-changed=build.rs");
    // let files = &["idl/admin/v1/api.proto"];
    // let includes = &["idl", "."];
    //
    //
    // let mut prost_config = prost_build::Config::new();
    // prost_config
    //     .out_dir("src/admin")
    //     .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    //     .type_attribute(".", "#[serde(rename_all = \"PascalCase\")]");
    //
    // let mut config = prost_validate_build::Builder::new();
    //
    // config.compile_protos_with_config(prost_config, files, includes)?;

    let include_dirs: Vec<PathBuf> = vec![
        ".".into(), // 添加当前目录作为根路径
        "idl".into(),
        "idl/base".into(),
        "idl/third_party".into(),
        "idl/admin/v1".into(),
    ];

    let proto_configs = get_proto_config();

    proto_configs.iter().for_each(|proto_config| {
        let config = pilota_build::Builder::protobuf()
            .plugin(SerdePlugin)
            .include_dirs(include_dirs.clone())
            .ignore_unused(true);

        config.compile(
            proto_config.proto_files.as_slice(),
            pilota_build::Output::File(proto_config.out_dir.clone().into()),
        );
    });
    Ok(())
}

fn get_proto_config() -> Vec<ProtoConfig> {
    let mut ans: Vec<ProtoConfig> = Vec::new();
    // admin服务
    ans.push(get_admin_service_config());

    ans
}

fn get_admin_service_config() -> ProtoConfig {
    ProtoConfig {
        out_dir: "src/admin_gen.rs".to_string(),
        proto_files: vec!["idl/admin/v1/api.proto".to_string()],
    }
}

struct ProtoConfig {
    out_dir: String,

    proto_files: Vec<String>,
}

#[derive(Clone, Copy)]
struct SerdePlugin;

impl pilota_build::Plugin for SerdePlugin {
    fn on_item(&mut self, cx: &Context, def_id: DefId, item: Arc<Item>) {
        match &*item {
            pilota_build::rir::Item::Message(_)
            | pilota_build::rir::Item::Enum(_)
            | pilota_build::rir::Item::NewType(_) => cx.with_adjust_mut(def_id, |adj| {
                adj.add_attrs(&[
                    "#[serde(rename_all = \"PascalCase\")]".into(),
                    "#[derive(::serde::Serialize, ::serde::Deserialize)]".into(),
                ])
            }),
            _ => {}
        }
    }
}
