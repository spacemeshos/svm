//! Implements common functionality to be consumed by tests.

use tokio::runtime::Runtime as TokioRuntime;

use std::sync::Arc;

use svm_codec::{template, Codec};
use svm_genesis_config::GenesisConfig;
use svm_layout::{FixedLayout, Layout};
use svm_runtime::{PriceResolverRegistry, Runtime, TemplatePriceCache};
use svm_state::GlobalState;
use svm_types::{
    Address, CodeSection, CtorsSection, DataSection, HeaderSection, SpawnAccount, Template,
    TemplateAddr, Transaction,
};

/// Hold a Wasm file in textual or binary form
pub enum WasmFile<'a> {
    /// Textual Wasm
    Text(&'a str),

    /// Binary Wasm
    Binary(&'a [u8]),
}

impl<'a> WasmFile<'a> {
    /// Returns the underlying bytes of the Wasm file binary.  
    pub fn to_binary(self) -> Vec<u8> {
        match self {
            Self::Text(text) => wat::parse_str(text).unwrap(),
            Self::Binary(wasm) => wasm.to_vec(),
        }
    }
}

/// Creates an [`Runtime`] backed by an in-memory [`GlobalState`].
pub fn create_memory_runtime() -> Runtime {
    let genesis = GenesisConfig::mainnet();
    let registry = PriceResolverRegistry::default();
    let tokio_rt = Arc::new(TokioRuntime::new().unwrap());
    let gs = tokio_rt.block_on(GlobalState::in_memory(genesis));
    Runtime::new(tokio_rt, gs, TemplatePriceCache::new(registry))
}

/// Creates an [`Runtime`] backed by the [`GlobalState`].
pub fn create_db_runtime(path: &str) -> Runtime {
    let genesis = GenesisConfig::mainnet();
    let registry = PriceResolverRegistry::default();
    let tokio_rt = Arc::new(TokioRuntime::new().unwrap());
    let gs = tokio_rt.block_on(GlobalState::new(path, genesis));
    Runtime::new(tokio_rt, gs, TemplatePriceCache::new(registry))
}

/// Builds a binary `Deploy Template` transaction.
pub fn build_deploy(
    code_version: u32,
    name: &str,
    layout: FixedLayout,
    ctors: &[String],
    wasm: WasmFile,
) -> Vec<u8> {
    let code = CodeSection::new_fixed(wasm.to_binary(), 0);
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
