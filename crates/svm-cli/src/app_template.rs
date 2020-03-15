use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use svm_app::{
    raw::decode_deploy_template, raw::NibbleIter, testing::DeployAppTemplateBuilder,
    types::AppTemplate,
};

pub fn encode(
    version: u32,
    name: &str,
    page_count: u16,
    code_path: &str,
    output_path: &str,
) -> Result<usize, Box<dyn Error>> {
    let file = File::open(code_path);
    let mut file = match file {
        Ok(v) => v,
        Err(e) => {
            let e = format!("failed to open file at {}: {}", code_path, e.to_string());
            return Err(e.into());
        }
    };

    let mut buf = Vec::new();
    if let Err(e) = file.read_to_end(&mut buf) {
        let e = format!("failed to read file at {}: {}", code_path, e.to_string());
        return Err(e.into());
    }

    let bytes = DeployAppTemplateBuilder::new()
        .with_version(version)
        .with_name(name)
        .with_page_count(page_count)
        .with_code(&buf)
        .build();

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

pub fn decode(data_path: &str) -> Result<AppTemplate, Box<dyn Error>> {
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
    decode_deploy_template(&mut iter).map_err(|e| e.to_string().into())
}
