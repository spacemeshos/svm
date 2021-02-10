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
pub use template::Template;
pub use wasm_type::{WasmType, WasmTypeError};
pub use wasm_value::WasmValue;

/// Represents a type in one of two ways:
/// * `(std::any::TypeId, &'static str str)`
///
/// * `&'static str`
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Type {
    /// An integer (`std::any::TypeId`) along it a static string.
    /// This string will usually be the value of `std::any::type_name::<T>()`
    TypeId(std::any::TypeId, &'static str),

    /// A static string.
    /// It enables the API user to attach descriptive names as types.
    ///
    /// One can name instances of the same Rust native `struct/enum`
    /// using different strings. It makes it easier to troubleshoot
    /// allocation/deallocation of resources since we can pinpoint each resource.
    Str(&'static str),
}

use std::fmt;

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Str(s) => write!(f, "{}", s),
            Type::TypeId(_ty, s) => write!(f, "{}", s),
        }
    }
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
