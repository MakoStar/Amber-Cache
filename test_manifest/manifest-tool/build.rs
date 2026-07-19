use std::fs;
use std::env;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;
use protox::prost::Message;
use protox::prost_reflect::prost_types::FileDescriptorSet;


fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=proto/Pb.proto");
    println!("cargo:rerun-if-changed=build.rs");

    let file_descriptors: FileDescriptorSet = protox::compile(&["proto/Pb.proto"], &["proto/"]).unwrap();
    
    let file_descriptor_path: PathBuf = PathBuf::from(env::var_os("OUT_DIR")
        .expect("OUT_DIR not set"))
        .join("file_descriptor_set.bin");

    fs::write(&file_descriptor_path, file_descriptors.encode_to_vec()).unwrap();

    let out_dir: &str = "src/proto";
    
    std::fs::create_dir_all(out_dir)
        .expect("Failed to create output directory");

    let mut config: prost_build::Config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.type_attribute(".", "#[serde(rename_all = \"PascalCase\")]"); 
    
    config
        .file_descriptor_set_path(&file_descriptor_path)
        .skip_protoc_run()
        .out_dir(out_dir)
        .compile_protos(&["proto/Pb.proto"], &["proto/"])
        .expect("Failed to compile proto files");

    fs::write(
        Path::new(out_dir).join("mod.rs"),
        "pub mod pb;\npub use pb::*;\n",
    ).expect("Failed to write mod.rs");
    
    Ok(())
}