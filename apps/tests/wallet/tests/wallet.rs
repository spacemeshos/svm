use svm_app::types::{HostCtx, WasmValue};
use svm_common::Address;
use svm_runtime::{settings::AppSettings, testing, traits::Runtime};

fn deploy_template(author: Address) -> Address {
    let version = 0;
    let kv = testing::memory_kv_store_init();
    let host = std::ptr::null_mut();
    let imports = Vec::new();
    let mut runtime = testing::create_memory_runtime(host, &kv, imports);
    let page_count = 1;

    let bytes = testing::build_template(
        version,
        "MultiSig Wallet",
        page_count,
        include_str!("../../../wasm/wallet.wast"),
    );

    runtime
        .deploy_template(&author, HostCtx::new(), &bytes)
        .unwrap()
}

fn spawn_app(template: Address, creator: Address) -> (State, Address) {
    //
}

#[test]
fn wallet_sanity() {
    let author = Address::of("@author");

    let template = deploy_template(author);

    dbg!(template);
}
