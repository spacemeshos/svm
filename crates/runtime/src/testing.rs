//! Implements common functionality to be consumed by tests.

use futures::executor::block_on;
use svm_codec::{template, Codec};
use svm_layout::{FixedLayout, Layout};
use svm_state::GlobalState;
use svm_types::{
    Address, CodeSection, CtorsSection, DataSection, HeaderSection, SpawnAccount, Template,
    TemplateAddr, Transaction,
};

use crate::env::{DefaultMemAccountStore, DefaultMemEnvTypes, DefaultMemTemplateStore};
use crate::{Config, DefaultRuntime, Env};

/// Hold a Wasm file in textual or binary form
pub enum WasmFile<'a> {
    /// Textual Wasm
    Text(&'a str),

    /// Binary Wasm
    Binary(&'a [u8]),
}

impl<'a> WasmFile<'a> {
    /// Returns the underlying bytes of the Wasm file binary.  
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::Text(text) => wat::parse_str(text).unwrap(),
            Self::Binary(wasm) => wasm.to_vec(),
        }
    }
}

impl<'a> From<&'a str> for WasmFile<'a> {
    fn from(text: &'a str) -> Self {
        Self::Text(text)
    }
}

impl<'a> From<&'a [u8]> for WasmFile<'a> {
    fn from(wasm: &'a [u8]) -> Self {
        Self::Binary(wasm)
    }
}

/// Creates an in-memory `Runtime` backed by a `state_kv`.
pub fn create_memory_runtime() -> DefaultRuntime<DefaultMemEnvTypes> {
    let template_store = DefaultMemTemplateStore::new();
    let account_store = DefaultMemAccountStore::new();
    let env = Env::<DefaultMemEnvTypes>::new(account_store, template_store);

    let config = Config::default();
    let imports = ("sm".to_string(), wasmer::Exports::new());

    let global_state = block_on(GlobalState::in_memory());
    DefaultRuntime::new(env, imports, global_state, config, None)
}

/// Builds a binary `Deploy Template` transaction.
pub fn build_deploy(
    code_version: u32,
    name: &str,
    layout: FixedLayout,
    ctors: &[String],
    wasm: WasmFile,
) -> Vec<u8> {
    let code = CodeSection::new_fixed(wasm.into_bytes(), 0);
    let ctors = CtorsSection::new(ctors.to_vec());
    let data = DataSection::with_layout(Layout::Fixed(layout));
    let header = HeaderSection::new(code_version, name.to_string(), "".to_string());

    let template = Template::new(code, data, ctors).with_header(Some(header));

    template::encode(&template)
}

/// Builds a binary `Spawn Account` transaction.
pub fn build_spawn(template: &TemplateAddr, name: &str, ctor: &str, calldata: &[u8]) -> Vec<u8> {
    let spawn = SpawnAccount::new(0, template, name, ctor, calldata);

    spawn.encode_to_vec()
}

/// Builds a binary `Call Account` transaction. (a.k.a a `Transaction`).
pub fn build_call(target: &Address, func: &str, calldata: &[u8]) -> Vec<u8> {
    let tx = Transaction {
        version: 0,
        target: target.clone(),
        func_name: func.to_string(),
        calldata: calldata.to_vec(),
        verifydata: vec![],
    };

    tx.encode_to_vec()
}
