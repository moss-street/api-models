use anyhow::Result;
use std::{env, path::PathBuf};

fn main() -> Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("moss-street.bin"))
        .compile_protos(&["protos/common.proto", "protos/users.proto"], &["protos"])?;

    tonic_build::compile_protos("protos/common.proto")?;
    tonic_build::compile_protos("protos/users.proto")?;
    Ok(())
}
