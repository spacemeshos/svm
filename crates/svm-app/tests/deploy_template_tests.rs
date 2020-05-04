use svm_app::{
    error::ParseError,
    memory::{DefaultMemAppStore, DefaultMemAppTemplateStore, DefaultMemoryEnv},
    testing::DeployAppTemplateBuilder,
    traits::Env,
    types::{AppTemplate, HostCtx},
};
use svm_common::Address;
use svm_storage2::layout::DataLayout;

fn inject_extra(bytes: &mut Vec<u8>) {
    bytes.extend_from_slice(&[0xFF]);
}

#[test]
fn deploy_template_fails_when_excessive_palyoad() {
    let app_store = DefaultMemAppStore::new();
    let template_store = DefaultMemAppTemplateStore::new();
    let env = DefaultMemoryEnv::new(app_store, template_store);

    let code = vec![0x0C, 0x00, 0x0D, 0x0E];
    let name = "Template #1";
    let page_count = 10;

    let mut bytes = DeployAppTemplateBuilder::new()
        .with_version(0)
        .with_name(name)
        .with_page_count(page_count)
        .with_code(&code)
        .build();

    inject_extra(&mut bytes);

    let res = env.parse_deploy_template(&bytes);
    assert_eq!(Err(ParseError::ExpectedEOF), res);
}

#[test]
fn deploy_template_store() {
    let app_store = DefaultMemAppStore::new();
    let template_store = DefaultMemAppTemplateStore::new();
    let mut env = DefaultMemoryEnv::new(app_store, template_store);

    let code = vec![0x0C, 0x00, 0x0D, 0x0E];
    let data = DataLayout::new();
    let name = "Template #1";
    let page_count = 10;
    let author = Address::of("@author").into();

    let bytes = DeployAppTemplateBuilder::new()
        .with_version(0)
        .with_name(name)
        .with_page_count(page_count)
        .with_code(&code)
        .with_data(&data)
        .build();

    let host_ctx = HostCtx::new();
    let template = env.parse_deploy_template(&bytes).unwrap();

    let expected_addr = env.derive_template_address(&template, &host_ctx);
    let actual_addr = env.store_template(&template, &author, &host_ctx).unwrap();
    assert_eq!(expected_addr, actual_addr);

    let expected_template = AppTemplate {
        version: 0,
        name: name.to_string(),
        page_count,
        code,
        data,
    };

    let expected = (expected_template, author);

    let host_ctx = HostCtx::new();

    let addr = env.derive_template_address(&template, &host_ctx);
    let actual = env.load_template(&addr).unwrap();

    assert_eq!(expected, actual);
}
