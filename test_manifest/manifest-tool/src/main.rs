mod proto;
use anyhow::Context;
use prost::Message;
use proto::{FileDiff, PbClientDiff};
use std::collections::BTreeMap;
use std::fs;

fn test() {
    let diff1: FileDiff = FileDiff {
        file_name: "data/test.json".to_string(),
        hash: "1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d".to_string(),
        version: 100,
        additional_path: "1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d".to_string(),
    };

    let diff2: FileDiff = FileDiff {
        file_name: "lua/test.lua".to_string(),
        hash: "a1b2c3d4e5f678901234567890abcdef".to_string(),
        version: 101,
        additional_path: "a1b2c3d4e5f678901234567890abcdef".to_string(),
    };

    let client_diff: PbClientDiff = PbClientDiff {
        file_diff: vec![diff1, diff2],
    };

    let encoded: Vec<u8> = client_diff.encode_to_vec();
    println!("Encoded PbClientDiff: {:?}", encoded);

    let decoded: PbClientDiff = PbClientDiff::decode(&encoded[..]).unwrap();

    for file in decoded.file_diff {
        println!("File: {}, Version: {}", file.file_name, file.version);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    test();
    let bin_data: Vec<u8> = fs::read("win.bin").context("File not found win.bin")?;
    let client_diff: PbClientDiff = PbClientDiff::decode(bin_data.as_slice())?;

    let mut manifest_map: BTreeMap<String, &proto::FileDiff> = BTreeMap::new();
    for diff in &client_diff.file_diff {
        manifest_map.insert(diff.file_name.clone(), diff);
    }

    let json_string = serde_json::to_string_pretty(&manifest_map)?;
    // println!("{}", json_string);
    for file_name in manifest_map.keys() {
        println!("File Key: {}", file_name);
    }

    std::fs::create_dir_all("output/").expect("Failed to create output directory");

    fs::write("output/manifest.json", json_string)?;

    Ok(())
}
