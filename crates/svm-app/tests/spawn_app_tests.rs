use svm_app::{
    error::{ParseError, StoreError},
    memory::{DefaultMemAppStore, DefaultMemAppTemplateStore, DefaultMemoryEnv},
    testing::SpawnAppBuilder,
    traits::Env,
    types::{App, AppTemplate, HostCtx, SpawnApp, WasmValue},
};

use svm_common::Address;

fn inject_extra(bytes: &mut Vec<u8>) {
    bytes.extend_from_slice(&[0xFF]);
}

#[test]
fn spawn_app_fails_when_excessive_palyoad() {
    let app_store = DefaultMemAppStore::new();
    let template_store = DefaultMemAppTemplateStore::new();
    let env = DefaultMemoryEnv::new(app_store, template_store);

    let template = Address::of("@my-template").into();
    let ctor_idx = 2;
    let ctor_buf = vec![0xAA, 0xAA, 0xAA, 0xBB, 0xBB];
    let ctor_args = vec![WasmValue::I32(10), WasmValue::I64(200)];

    let mut bytes = SpawnAppBuilder::new()
        .with_version(0)
        .with_template(&template)
        .with_ctor_index(ctor_idx)
        .with_ctor_buf(&ctor_buf)
        .with_ctor_args(&ctor_args)
        .build();

    inject_extra(&mut bytes);

    let res = env.parse_spawn_app(&bytes);
    assert_eq!(Err(ParseError::ExpectedEOF), res);
}

#[test]
fn spawn_app_parse() {
    let app_store = DefaultMemAppStore::new();
    let template_store = DefaultMemAppTemplateStore::new();
    let env = DefaultMemoryEnv::new(app_store, template_store);

    let template = Address::of("@my-template").into();
    let ctor_idx = 2;
    let ctor_buf = vec![0xAA, 0xAA, 0xAA, 0xBB, 0xBB];
    let ctor_args = vec![WasmValue::I32(10), WasmValue::I64(200)];

    let bytes = SpawnAppBuilder::new()
        .with_version(0)
        .with_template(&template)
        .with_ctor_index(ctor_idx)
        .with_ctor_buf(&ctor_buf)
        .with_ctor_args(&ctor_args)
        .build();

    let actual = env.parse_spawn_app(&bytes).unwrap();

    let expected = SpawnApp {
        app: App {
            version: 0,
            template,
        },
        ctor_idx,
        ctor_buf,
        ctor_args,
    };

    assert_eq!(expected, actual);
}

#[test]
fn spawn_app_valid_app() {
    let app_store = DefaultMemAppStore::new();
    let template_store = DefaultMemAppTemplateStore::new();
    let mut env = DefaultMemoryEnv::new(app_store, template_store);

    let author = Address::of("@author").into();
    let creator = Address::of("@creator").into();
    let host_ctx = HostCtx::new();

    let template = AppTemplate {
        version: 0,
        name: "My Template".to_string(),
        page_count: 10,
        code: vec![0x0C, 0x00, 0x0D, 0x0E],
    };
    assert!(env.store_template(&template, &author, &host_ctx).is_ok());

    let template = env.derive_template_address(&template, &host_ctx);

    let ctor_idx = 2;
    let bytes = SpawnAppBuilder::new()
        .with_version(0)
        .with_ctor_index(ctor_idx)
        .with_template(&template)
        .build();

    let spawn = env.parse_spawn_app(&bytes).unwrap();
    let expected_addr = env.derive_app_address(&spawn, &host_ctx);
    let actual_addr = env.store_app(&spawn, &creator, &host_ctx).unwrap();

    assert_eq!(expected_addr, actual_addr);

    let expected_app = App {
        version: 0,
        template,
    };

    let expected = (expected_app, creator.into());

    let actual = env.load_app(&actual_addr).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn spawn_app_template_does_not_exist() {
    let app_store = DefaultMemAppStore::new();
    let template_store = DefaultMemAppTemplateStore::new();
    let mut env = DefaultMemoryEnv::new(app_store, template_store);

    let template = Address::of("@my-template").into();
    let creator = Address::of("@creator").into();
    let ctor_idx = 2;

    let bytes = SpawnAppBuilder::new()
        .with_version(0)
        .with_ctor_index(ctor_idx)
        .with_template(&template)
        .build();

    let spawn = env.parse_spawn_app(&bytes).unwrap();

    let host_ctx = HostCtx::new();
    let actual = env.store_app(&spawn, &creator, &host_ctx);

    let msg = format!(
        "`AppTemplate` not found (address = `Address({:?})`)",
        template.inner().bytes()
    );

    let expected = Err(StoreError::DataCorruption(msg.to_string()));

    assert_eq!(expected, actual);
}
