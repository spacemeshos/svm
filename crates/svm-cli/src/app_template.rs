use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use svm_app::{raw::parse_template, testing::AppTemplateBuilder, types::AppTemplate};
use svm_common::Address;

pub fn encode(
    version: u32,
    name: &str,
    page_count: u16,
    code_path: &str,
    output_path: &str,
) -> Result<usize, io::Error> {
    let mut file = File::open(code_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let bytes = AppTemplateBuilder::new()
        .with_version(version)
        .with_name(name)
        .with_page_count(page_count)
        .with_code(&buffer)
        .build();

    let mut file = File::create(output_path)?;
    file.write_all(&bytes)?;

    Ok(bytes.len())
}

pub fn decode(data_path: &str) -> Result<AppTemplate, Box<dyn Error>> {
    let mut file = File::open(data_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    parse_template(&buffer, &Address::of("")).map_err(|e| e.to_string().into())
}
