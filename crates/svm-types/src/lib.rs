#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![feature(vec_into_raw_parts)]

#[macro_use]
mod macros;

mod address;
mod address_of;
mod app;
mod app_tx;
mod host_ctx;
mod spawn_app;
mod state;
mod template;
mod wasm_type;
mod wasm_value;

pub use address::{Address, AppAddr, AuthorAddr, CreatorAddr, TemplateAddr};
pub use address_of::AddressOf;
pub use app::App;
pub use app_tx::AppTransaction;
pub use host_ctx::HostCtx;
pub use spawn_app::SpawnApp;
pub use state::State;
pub use template::AppTemplate;
pub use wasm_type::{WasmType, WasmTypeError};
pub use wasm_value::WasmValue;
