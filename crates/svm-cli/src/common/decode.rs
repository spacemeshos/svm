use std::error::Error;

use crate::wasm_value;

use svm_types::{Address, WasmValue};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    match hex::decode(hex_str) {
        Ok(v) => Ok(v),
        Err(e) => {
            let e = format!(
                "failed to decode hex string '{}': {}",
                hex_str,
                e.to_string()
            );
            return Err(e.into());
        }
    }
}

pub fn decode_addr(addr_hex: &str) -> Result<Address, Box<dyn Error>> {
    let addr = decode_hex(addr_hex)?;

    if Address::len() != addr.len() {
        let e = format!(
            "invalid address length: found {}, expected: {}",
            addr.len(),
            Address::len()
        );
        return Err(e.into());
    }

    Ok(Address::from(addr.as_slice()))
}

pub fn decode_args(args: Vec<&str>) -> Result<Vec<WasmValue>, Box<dyn Error>> {
    let mut vals = Vec::new();
    let mut errs = Vec::new();

    let results = args.iter().map(|&v| wasm_value::parse_str(v));
    for (i, result) in results.enumerate() {
        match result {
            Ok(v) => vals.push(v),
            Err(e) => errs.push(format!("{}: {:?}", args[i], e)),
        }
    }
    if errs.len() > 0 {
        return Err(format!("invalid ctor_args: {:?}", errs).into());
    }

    Ok(vals)
}
