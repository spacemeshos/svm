#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![feature(const_type_id)]
#![feature(const_type_name)]
#![feature(vec_into_raw_parts)]

#[macro_use]
mod macros;

mod address;
mod address_of;
mod app;
mod app_tx;
mod spawn_app;
mod state;
mod template;
mod wasm_type;
mod wasm_value;

pub mod gas;
pub mod receipt;
pub use address::{Address, AppAddr, AuthorAddr, CreatorAddr, TemplateAddr};
pub use address_of::AddressOf;
pub use app::App;
pub use app_tx::AppTransaction;
pub use spawn_app::SpawnApp;
pub use state::State;
pub use template::AppTemplate;
pub use wasm_type::{WasmType, WasmTypeError};
pub use wasm_value::WasmValue;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Type {
    TypeId(std::any::TypeId, &'static str),

    Str(&'static str),
}

impl Type {
    pub const fn of<T: 'static>() -> Self {
        let ty = std::any::TypeId::of::<T>();
        let name = std::any::type_name::<T>();

        Type::TypeId(ty, name)
    }
}

impl From<&'static str> for Type {
    fn from(s: &'static str) -> Self {
        Type::Str(s)
    }
}
