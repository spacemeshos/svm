use std::error::Error;

use crate::common;
use svm_app::{
    raw::{decode_exec_app, NibbleIter},
    testing::AppTxBuilder,
    types::{AppTransaction, WasmValue},
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

    let mut func_buf = None;
    if let Some(func_buf_hex) = func_buf_hex {
        func_buf = Some(common::decode_hex(func_buf_hex)?);
    }

    let mut func_args_vals: Option<Vec<WasmValue>> = None;
    if let Some(func_args) = func_args {
        func_args_vals = Some(common::decode_args(func_args)?);
    }

    let mut builder = AppTxBuilder::new()
        .with_version(version)
        .with_app(&app_addr.into())
        .with_func_index(func_idx);
    if let Some(func_buf) = func_buf {
        builder = builder.with_func_buf(&func_buf)
    }
    if let Some(func_args_vals) = func_args_vals {
        builder = builder.with_func_args(&func_args_vals);
    }
    let bytes = builder.build();

    common::write_to_file(output_path, &bytes)?;

    Ok(bytes.len())
}

pub fn decode(data_path: &str) -> Result<AppTransaction, Box<dyn Error>> {
    let buf = common::read_file(data_path)?;

    let mut iter = NibbleIter::new(&buf);
    decode_exec_app(&mut iter).map_err(|e| e.to_string().into())
}
