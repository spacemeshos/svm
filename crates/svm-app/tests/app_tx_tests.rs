use svm_app::{
    memory::{DefaultMemAppStore, DefaultMemAppTemplateStore, DefaultMemoryEnv},
    testing::AppTxBuilder,
    traits::Env,
    types::{App, AppTemplate, AppTransaction, HostCtx, SpawnApp, WasmValue},
};
use svm_common::Address;

#[test]
fn parse_exec_app() {
    let app_store = DefaultMemAppStore::new();
    let template_store = DefaultMemAppTemplateStore::new();
    let mut env = DefaultMemoryEnv::new(app_store, template_store);

    let author = Address::of("@author");

    let template = AppTemplate {
        version: 0,
        name: "My Template".to_string(),
        page_count: 5,
        code: vec![0x0C, 0x00, 0x0D, 0x0E],
    };

    let host_ctx = HostCtx::new();
    let res = env.store_template(&template, &author, &host_ctx);
    assert!(res.is_ok());

    let template = env.derive_template_address(&template, &host_ctx);

    let spawn = SpawnApp {
        app: App {
            version: 0,
            template,
        },
        ctor_idx: 2,
        ctor_buf: vec![],
        ctor_args: vec![],
    };

    let host_ctx = HostCtx::new();

    assert!(env.store_app(&spawn, &author, &host_ctx).is_ok());

    let app = env.derive_app_address(&spawn, &host_ctx);

    let func_buf = vec![0xAA, 0xAA, 0xAA, 0xBB, 0xBB];
    let func_args = vec![WasmValue::I32(10), WasmValue::I64(20)];
    let func_idx = 5;

    let bytes = AppTxBuilder::new()
        .with_version(0)
        .with_app(&app)
        .with_func_index(func_idx)
        .with_func_buf(&func_buf)
        .with_func_args(&func_args)
        .build();

    let actual = env.parse_exec_app(&bytes).unwrap();

    let expected = AppTransaction {
        app,
        func_idx,
        func_args,
        func_buf,
    };

    assert_eq!(expected, actual);
}
