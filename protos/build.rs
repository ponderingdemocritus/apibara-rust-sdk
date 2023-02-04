use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("node_descriptor.bin"))
        .compile(&["proto/v1alpha2/starknet.proto", "proto/v1alpha2/stream.proto"], &["proto/"])?;
    Ok(())
}
