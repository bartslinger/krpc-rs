use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["proto/krpc.proto"], &["proto/"])?;
    let out_path = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("space_center.rs");
    krpc_gen::generate_for(
        std::path::Path::new("/home/bart/.local/share/Steam/steamapps/common/Kerbal Space Program/GameData/kRPC/KRPC.SpaceCenter.json"),
        &out_path
    );
    Ok(())
}
