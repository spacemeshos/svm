use std::error::Error;

use crate::common;
use svm_app::{
    raw::{decode_spawn_app, NibbleIter},
    testing::SpawnAppBuilder,
    types::{SpawnApp, WasmValue},
};

pub fn encode(
    version: u32,
    template_addr_hex: &str,
    ctor_idx: u16,
    ctor_buf_hex: Option<&str>,
    ctor_args: Option<Vec<&str>>,
    output_path: &str,
) -> Result<usize, Box<dyn Error>> {
    let template_addr = common::decode_addr(template_addr_hex)?;

    let mut ctor_buf = None;
    if let Some(ctor_buf_hex) = ctor_buf_hex {
        ctor_buf = Some(common::decode_hex(ctor_buf_hex)?);
    }

    let mut ctor_args_vals: Option<Vec<WasmValue>> = None;
    if let Some(ctor_args) = ctor_args {
        ctor_args_vals = Some(common::decode_args(ctor_args)?);
    }

    let mut builder = SpawnAppBuilder::new()
        .with_version(version)
        .with_template(&template_addr.into())
        .with_ctor_index(ctor_idx);
    if let Some(ctor_buf) = ctor_buf {
        builder = builder.with_ctor_buf(&ctor_buf)
    }
    if let Some(ctor_args_vals) = ctor_args_vals {
        builder = builder.with_ctor_args(&ctor_args_vals);
    }
    let bytes = builder.build();

    common::write_to_file(output_path, &bytes)?;

    Ok(bytes.len())
}

pub fn decode(data_path: &str) -> Result<SpawnApp, Box<dyn Error>> {
    let buf = common::read_file(data_path)?;

    let mut iter = NibbleIter::new(&buf);
    decode_spawn_app(&mut iter).map_err(|e| e.to_string().into())
}
