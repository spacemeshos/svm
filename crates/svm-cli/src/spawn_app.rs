use std::error::Error;

use crate::common;

use svm_codec::api::builder::SpawnAppBuilder;
use svm_codec::api::raw::decode_spawn_app;
use svm_codec::nibble::NibbleIter;

use svm_types::SpawnApp;

pub fn encode(
    version: u32,
    template_addr_hex: &str,
    ctor_idx: u16,
    ctor_buf_hex: Option<&str>,
    ctor_args: Option<Vec<&str>>,
    output_path: &str,
) -> Result<usize, Box<dyn Error>> {
    let template_addr = common::decode_addr(template_addr_hex)?;

    let ctor_buf = match ctor_buf_hex {
        Some(v) => Some(common::decode_hex(v)?),
        None => None,
    };

    let ctor_args = match ctor_args {
        Some(v) => Some(common::decode_args(v)?),
        None => None,
    };

    let bytes = SpawnAppBuilder::new()
        .with_version(version)
        .with_template(&template_addr.into())
        .with_ctor_index(ctor_idx)
        .with_ctor_buf(&ctor_buf.unwrap_or(vec![]))
        .with_ctor_args(&ctor_args.unwrap_or(vec![]))
        .build();

    common::write_to_file(output_path, &bytes)?;

    Ok(bytes.len())
}

pub fn decode(data_path: &str) -> Result<SpawnApp, Box<dyn Error>> {
    let buf = common::read_file(data_path)?;

    let mut iter = NibbleIter::new(&buf);
    decode_spawn_app(&mut iter).map_err(|e| e.to_string().into())
}
