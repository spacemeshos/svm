use std::error::Error;

use crate::common;

use svm_app::{
    raw::decode_deploy_template, raw::NibbleIter, testing::DeployAppTemplateBuilder,
    types::AppTemplate,
};
use svm_layout::DataLayout;

pub fn encode(
    version: u32,
    name: &str,
    code_path: &str,
    output_path: &str,
) -> Result<usize, Box<dyn Error>> {
    let buf = common::read_file(code_path)?;

    let bytes = DeployAppTemplateBuilder::new()
        .with_version(version)
        .with_name(name)
        .with_code(&buf)
        .with_data(&DataLayout::empty())
        .build();

    common::write_to_file(output_path, &bytes)?;

    Ok(bytes.len())
}

pub fn decode(data_path: &str) -> Result<AppTemplate, Box<dyn Error>> {
    let buf = common::read_file(data_path)?;

    let mut iter = NibbleIter::new(&buf);
    decode_deploy_template(&mut iter).map_err(|e| e.to_string().into())
}
