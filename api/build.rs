fn main() {
    // 告诉cargo在proto文件修改时重新构建
    // println!("cargo:rerun-if-changed=proto");
    // println!("cargo:rerun-if-changed=build.rs");
    volo_build::ConfigBuilder::default()
        .plugin(SerdePlugin)
        .write()
        .unwrap();
}

#[derive(Clone, Copy)]
struct SerdePlugin;

impl pilota_build::Plugin for SerdePlugin {
    fn on_item(
        &mut self,
        cx: &pilota_build::Context,
        def_id: pilota_build::DefId,
        item: std::sync::Arc<pilota_build::rir::Item>,
    ) {
        match &*item {
            pilota_build::rir::Item::Message(_)
            | pilota_build::rir::Item::Enum(_)
            | pilota_build::rir::Item::NewType(_) => cx.with_adjust_mut(def_id, |adj| {
                adj.add_attrs(&[
                    "#[derive(::serde::Serialize, ::serde::Deserialize, ::derive_builder::Builder)]".into(),
                    "#[serde(rename_all = \"PascalCase\")]".into(),
                ])
            }),
            _ => {}
        };
        pilota_build::plugin::walk_item(self, cx, def_id, item)
    }
}
