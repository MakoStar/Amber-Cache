use std::fs;
use std::path::Path;
use std::io::Result;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=proto/Pb.proto");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir: &str = "src/proto";
    
    std::fs::create_dir_all(out_dir)
        .expect("Failed to create output directory");

    let mut config: prost_build::Config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.type_attribute(".", "#[serde(rename_all = \"PascalCase\")]"); 
    
    config.out_dir(out_dir)
        .compile_protos(&["proto/Pb.proto"], &["proto/"])
        .expect("Failed to compile proto files");

    fs::write(
        Path::new(out_dir).join("mod.rs"),
        "pub mod pb;\npub use pb::*;\n",
    ).expect("Failed to write mod.rs");
    
    Ok(())
}