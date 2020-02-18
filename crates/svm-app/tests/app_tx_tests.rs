use svm_app::{
    memory::{JsonMemAppStore, JsonMemAppTemplateStore, JsonMemoryEnv},
    testing::AppTxBuilder,
    traits::Env,
    types::{App, AppTemplate, AppTransaction, WasmValue},
};
use svm_common::Address;

#[test]
fn parse_app_tx() {
    let app_store = JsonMemAppStore::new();
    let template_store = JsonMemAppTemplateStore::new();
    let mut env = JsonMemoryEnv::new(app_store, template_store);

    let template = AppTemplate {
        name: "My Template".to_string(),
        author: Address::of("@author"),
        page_count: 5,
        code: vec![0x00, 0x00, 0x00],
    };

    let res = env.store_template(&template);
    assert!(res.is_ok());

    let template_addr = env.derive_template_address(&template);
    let creator_addr = Address::of("@creator");

    let app = App {
        template: template_addr,
        creator: creator_addr,
    };

    assert!(env.store_app(&app).is_ok());

    let sender = Address::of("@sender");
    let app = env.derive_app_address(&app);
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

    let actual = env.parse_app_tx(&bytes, &sender).unwrap();

    let expected = AppTransaction {
        app,
        sender,
        func_idx,
        func_args,
        func_buf,
    };

    assert_eq!(expected, actual);
}
