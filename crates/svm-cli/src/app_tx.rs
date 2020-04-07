use std::error::Error;
use std::fs::File;
use std::io::prelude::{Read, Write};

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

    let file = File::create(output_path);
    let mut file = match file {
        Ok(v) => v,
        Err(e) => {
            let e = format!(
                "failed to create file at {}: {}",
                output_path,
                e.to_string()
            );
            return Err(e.into());
        }
    };
    file.write_all(&bytes)?;

    Ok(bytes.len())
}

pub fn decode(data_path: &str) -> Result<AppTransaction, Box<dyn Error>> {
    let file = File::open(data_path);
    let mut file = match file {
        Ok(v) => v,
        Err(e) => {
            let e = format!("failed to open file at {}: {}", data_path, e.to_string());
            return Err(e.into());
        }
    };

    let mut buf = Vec::new();
    if let Err(e) = file.read_to_end(&mut buf) {
        let e = format!("failed to read file at {}: {}", data_path, e.to_string());
        return Err(e.into());
    }

    let mut iter = NibbleIter::new(&buf);
    decode_exec_app(&mut iter).map_err(|e| e.to_string().into())
}
