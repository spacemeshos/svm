use svm_app::{
    error::StoreError,
    memory::{JsonMemAppStore, JsonMemAppTemplateStore, JsonMemoryEnv},
    testing::AppBuilder,
    traits::Env,
    types::{App, AppTemplate, SpawnApp, WasmValue},
};
use svm_common::Address;

#[test]
fn spawn_app_parse() {
    let app_store = JsonMemAppStore::new();
    let template_store = JsonMemAppTemplateStore::new();
    let env = JsonMemoryEnv::new(app_store, template_store);

    let template = Address::of("@my-template");
    let creator = Address::of("@creator");
    let ctor_buf = vec![0xAA, 0xAA, 0xAA, 0xBB, 0xBB];
    let ctor_args = vec![WasmValue::I32(10), WasmValue::I64(200)];

    let bytes = AppBuilder::new()
        .with_version(0)
        .with_template(&template)
        .with_ctor_buf(&ctor_buf)
        .with_ctor_args(&ctor_args)
        .build();

    let actual = env.parse_app(&bytes, &creator).unwrap();

    let expected = SpawnApp {
        app: App { template, creator },
        ctor_buf,
        ctor_args,
    };

    assert_eq!(expected, actual);
}

#[test]
fn spawn_app_valid_app() {
    let app_store = JsonMemAppStore::new();
    let template_store = JsonMemAppTemplateStore::new();
    let mut env = JsonMemoryEnv::new(app_store, template_store);

    let template = AppTemplate {
        name: "My Template".to_string(),
        author: Address::of("@author"),
        page_count: 10,
        code: vec![0x00, 0x00, 0x00],
    };
    assert!(env.store_template(&template).is_ok());

    let template_addr = env.derive_template_address(&template);
    let creator_addr = Address::from(0x50_60_70_80);

    let bytes = AppBuilder::new()
        .with_version(0)
        .with_template(&template_addr)
        .build();

    let spawn_app = env.parse_app(&bytes, &creator_addr).unwrap();
    let app = &spawn_app.app;

    let expected_addr = env.derive_app_address(app);
    let actual_addr = env.store_app(app).unwrap();
    assert_eq!(expected_addr, actual_addr);

    let expected = App {
        template: template_addr,
        creator: creator_addr,
    };

    let actual = env.load_app(&actual_addr).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn spawn_app_template_does_not_exist() {
    let app_store = JsonMemAppStore::new();
    let template_store = JsonMemAppTemplateStore::new();
    let mut env = JsonMemoryEnv::new(app_store, template_store);

    let template = Address::of("@my-template");
    let creator = Address::of("@creator");

    let bytes = AppBuilder::new()
        .with_version(0)
        .with_template(&template)
        .build();

    let spawn_app = env.parse_app(&bytes, &creator).unwrap();
    let actual = env.store_app(&spawn_app.app);

    let msg = format!(
        "`AppTemplate` not found (address = `Address({:?})`)",
        template.bytes()
    );
    let expected = Err(StoreError::DataCorruption(msg.to_string()));

    assert_eq!(expected, actual);
}
