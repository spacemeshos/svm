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
        name: "Template #1".to_string(),
        author: Address::from(0x00_11_22_33),
        pages_count: 5,
        code: vec![0x00, 0x00, 0x00],
    };

    let res = env.store_template(&template);
    assert!(res.is_ok());

    let template_addr = env.derive_template_address(&template);
    let creator_addr = Address::from(0x50_60_70_80);

    let app = App {
        template: template_addr,
        creator: creator_addr,
    };

    assert!(env.store_app(&app).is_ok());

    let sender_addr = Address::from(0x00_AA_BB_CC);
    let app_addr = env.derive_app_address(&app);

    let bytes = AppTxBuilder::new()
        .with_version(0)
        .with_app(&app_addr)
        .with_func_name("run")
        .with_func_args(&vec![WasmValue::I32(10), WasmValue::I64(20)])
        .build();

    let actual = env.parse_app_tx(&bytes, &sender_addr).unwrap();

    let expected = AppTransaction {
        app: app_addr,
        sender: sender_addr,
        func_name: "run".to_string(),
        func_args: vec![WasmValue::I32(10), WasmValue::I64(20)],
        func_args_buf: vec![],
    };

    assert_eq!(expected, actual);
}
