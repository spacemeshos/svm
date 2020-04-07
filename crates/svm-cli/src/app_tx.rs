use std::error::Error;

use crate::common;
use svm_app::{
    raw::{decode_exec_app, NibbleIter},
    testing::AppTxBuilder,
    types::AppTransaction,
};

pub fn encode(
    version: u32,
    app_addr_hex: &str,
    func_idx: u16,
    func_buf_hex: Option<&str>,
    func_args: Option<Vec<&str>>,
    output_path: &str,
) -> Result<usize, Box<dyn Error>> {
    let app_addr = common::decode_addr(app_addr_hex)?;

    let func_buf = match func_buf_hex {
        Some(v) => Some(common::decode_hex(v)?),
        None => None,
    };

    let func_args = match func_args {
        Some(v) => Some(common::decode_args(v)?),
        None => None,
    };

    let bytes = AppTxBuilder::new()
        .with_version(version)
        .with_app(&app_addr.into())
        .with_func_index(func_idx)
        .with_func_buf(&func_buf.unwrap_or(vec![]))
        .with_func_args(&func_args.unwrap_or(vec![]))
        .build();

    common::write_to_file(output_path, &bytes)?;

    Ok(bytes.len())
}

pub fn decode(data_path: &str) -> Result<AppTransaction, Box<dyn Error>> {
    let buf = common::read_file(data_path)?;

    let mut iter = NibbleIter::new(&buf);
    decode_exec_app(&mut iter).map_err(|e| e.to_string().into())
}
