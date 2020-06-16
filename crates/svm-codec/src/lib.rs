#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![feature(vec_into_raw_parts)]

mod app;
mod error;
mod field;
mod func_args;
mod func_buf;
mod gas;
mod host_ctx;
mod template;
mod transaction;
mod varuint14;
mod version;

#[macro_use]
mod nibble;

pub mod api;
pub mod helpers;
pub mod traits;
pub mod wasm;

pub use app::{decode_spawn_app, encode_spawn_app, DefaultAppDeserializer, DefaultAppSerializer};
pub use error::ParseError;
pub use field::Field;
pub use func_args::{decode_func_args, decode_func_rets, encode_func_args, encode_func_rets};
pub use func_buf::{decode_func_buf, encode_func_buf};
pub use gas::{decode_gas_used, encode_gas_used};
pub use nibble::{concat_nibbles, Nibble, NibbleIter, NibbleWriter};
pub use template::{decode_deploy_template, encode_deploy_template};
pub use template::{DefaultAppTemplateDeserializer, DefaultAppTemplateSerializer};
pub use transaction::{decode_exec_app, encode_exec_app};
pub use varuint14::{decode_varuint14, encode_varuint14};
pub use version::{decode_version, encode_version};
