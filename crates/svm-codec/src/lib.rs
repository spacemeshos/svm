#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![feature(vec_into_raw_parts)]

#[macro_use]
mod nibble;

#[macro_use]
mod wasm;

mod api;
mod app;
mod error;
mod field;
mod func_args;
mod func_buf;
mod gas;
mod helpers;
mod host_ctx;
mod template;
mod traits;
mod transaction;
mod varuint14;
mod version;

use app::{decode_spawn_app, encode_spawn_app, DefaultAppDeserializer, DefaultAppSerializer};
use error::ParseError;
use field::Field;
use func_args::{decode_func_args, decode_func_rets, encode_func_args, encode_func_rets};
use func_buf::{decode_func_buf, encode_func_buf};
use gas::{decode_gas_used, encode_gas_used};
use nibble::{concat_nibbles, Nibble, NibbleIter, NibbleWriter};
use template::{decode_deploy_template, encode_deploy_template};
use template::{DefaultAppTemplateDeserializer, DefaultAppTemplateSerializer};
use transaction::{decode_exec_app, encode_exec_app};
use varuint14::{decode_varuint14, encode_varuint14};
use version::{decode_version, encode_version};

/// ## WASM Deploy-Template
///
/// Reads the WASM buffer given at parameter `buf_ptr` containing a JSON value.
/// Encodes a `deploy-template` binary-transaction using that JSON value.
///
/// Returns a pointer to a new WASM buffer holding the encoded transaction.
/// If the encoding failed, the return WASM buffer will contain a JSON with the error.
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_deploy_template(buf_ptr: i32) -> i32 {
    todo!()
}

/// ## WASM Spawn-App
///
/// Reads the WASM buffer given at parameter `buf_ptr` containing a JSON value.
/// Encodes a `spawn-app` binary-transaction using that JSON value.
///
/// Returns a pointer to a new WASM buffer holding the encoded transaction.
/// If the encoding failed, the return WASM buffer will contain a JSON with the error.
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_spawn_app(buf_ptr: i32) -> i32 {
    todo!()
}

/// ## WASM Execute-App
///
/// Reads the WASM buffer given at parameter `buf_ptr` containing a JSON value.
/// Encodes a `exec-app` binary-transaction using that JSON value.
///
/// Returns a pointer to a new WASM buffer holding the encoded transaction.
/// If the encoding failed, the return WASM buffer will contain a JSON with the error.
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_exec_app(buf_ptr: i32) -> i32 {
    todo!()
}

/// ## WASM Buffer Allocate
///
/// Allocates a new WASM Buffer holding data of `length` bytes.
///
/// For more info read: `api::wasm::alloc`
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_alloc(length: i32) -> i32 {
    let ptr = api::wasm::alloc(length as usize);

    ptr as i32
}

/// ## WASM Buffer Free
///
/// Frees the WASM buffer allocated starting from offset `buf_ptr`.
///
/// For more info read: `api::wasm::free`
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_free(buf_ptr: i32) {
    api::wasm::free(buf_ptr as usize)
}
