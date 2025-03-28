use anyhow::Result;
use std::{env, path::PathBuf};

fn main() -> Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let protos = &["protos/auth/v1/auth.proto", "protos/trade/v1/trading.proto"];

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("moss-street.bin"))
        .compile_protos(protos, &["protos"])?;

    protos.iter().for_each(|p| {
        let _ = tonic_build::compile_protos(*p)
            .inspect_err(|e| eprintln!("Error compiling proto {e:#?}"));
    });
    Ok(())
}
