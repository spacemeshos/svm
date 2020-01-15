use svm_app::{
    error::StoreError,
    memory::{JsonMemAppStore, JsonMemAppTemplateStore, JsonMemoryEnv},
    testing::AppBuilder,
    traits::Env,
    types::{App, AppTemplate},
};
use svm_common::Address;

#[test]
fn parse_app() {
    let app_store = JsonMemAppStore::new();
    let template_store = JsonMemAppTemplateStore::new();
    let env = JsonMemoryEnv::new(app_store, template_store);

    let template_addr = Address::from(0x10_20_30_40);
    let creator_addr = Address::from(0x50_60_70_80);

    let bytes = AppBuilder::new()
        .with_version(0)
        .with_template(&template_addr)
        .build();

    let app = env.parse_app(&bytes, &creator_addr).unwrap();

    assert_eq!(template_addr, app.template);
    assert_eq!(creator_addr, app.creator);
}

#[test]
fn valid_app_creation() {
    let app_store = JsonMemAppStore::new();
    let template_store = JsonMemAppTemplateStore::new();
    let mut env = JsonMemoryEnv::new(app_store, template_store);

    let template = AppTemplate {
        name: "Template #1".to_string(),
        author: Address::from(0x00_11_22_33),
        pages_count: 10,
        code: vec![0x00, 0x00, 0x00],
    };
    assert!(env.store_template(&template).is_ok());

    let template_addr = env.derive_template_address(&template);
    let creator_addr = Address::from(0x50_60_70_80);

    let bytes = AppBuilder::new()
        .with_version(0)
        .with_template(&template_addr)
        .build();

    let app = env.parse_app(&bytes, &creator_addr).unwrap();
    let expected_addr = env.derive_app_address(&app);

    let actual_addr = env.store_app(&app).unwrap();
    assert_eq!(expected_addr, actual_addr);

    let expected = App {
        template: template_addr,
        creator: creator_addr,
    };

    let actual = env.load_app(&actual_addr).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn app_template_does_not_exist() {
    let app_store = JsonMemAppStore::new();
    let template_store = JsonMemAppTemplateStore::new();
    let mut env = JsonMemoryEnv::new(app_store, template_store);

    let template_addr = Address::from(0x10_20_30_40);
    let creator_addr = Address::from(0x50_60_70_80);

    let bytes = AppBuilder::new()
        .with_version(0)
        .with_template(&template_addr)
        .build();

    let app = env.parse_app(&bytes, &creator_addr).unwrap();
    let actual = env.store_app(&app);

    let msg = "`AppTemplate` not found (address = `Address([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 32, 48, 64])`)";
    let expected = Err(StoreError::DataCorruption(msg.to_string()));

    assert_eq!(expected, actual);
}
