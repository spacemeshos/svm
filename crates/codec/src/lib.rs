//! This crate is responsible for doing the binary encoding for SVM transactions.
//! It code is compiled as a single WASM file and it should be integrated by Wallet Apps (`smapp`).
//!
//! By doing that, a Wallet Apps can locally encode a binary transaction without having to re-implement all the logic
//! of the `svm-codec`.
//
//! The CI of the SVM outputs the WASM package of `svm-codec` as one of its artifacts.

#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![feature(vec_into_raw_parts)]

mod calldata;
mod common;
mod ext;
mod field;
mod section;
mod r#type;

/// Encoding for apps
pub mod app;

/// Encoding for templates
pub mod template;

/// Encoding for apps-transactions
pub mod transaction;

pub use ext::{ReadExt, WriteExt};
pub use field::Field;

/// Wraps the exposed APIs under a single place.
/// This crate exposes the following APIs:
///
/// * Builder
/// * Raw  
/// * JSON   
/// * WASM
pub mod api;

/// Encoding of receipts.
pub mod receipt;

mod error;
pub use error::ParseError;

/// # WASM API
///
/// The following API methods are annotated with `#[cfg(target_arch = "wasm32")]`.
/// In order to output a `.wasm` file run (or run `./build.sh` under the crate root directory).
/// ```
//// cargo +nightly build --release --target wasm32-unknown-unknown
/// ```
///
/// The emitted `svm_codec.wasm` is being tested in the `examples/test.js`
/// In order to build and test run `./run.sh` under the `examples` directory.
///
/// The CI of the `SVM` also runs the js tests and outputs `svm_codec.wasm` under the artifacts.
///

/// ## WASM API Usage
///
/// Before calling `wasm_deploy_template/wasm_spawn_app/wasm_exec_app` we need first to allocate
/// a WASM buffer using the `wasm_alloc` method. After the buffer isn't needed anymore, make sure to
/// call the `wasm_free` method. (otherwise it'll be a memory-leak).
///
/// The data returned by `wasm_deploy_template/wasm_spawn_app/wasm_exec_app` is a pointer to a new allocated
/// WASM buffer. This WASM buffer is allocated internally by the method and have to be freed later too using `wasm_free`.
///
///
/// WASM Buffer `Data` for Success result:
/// ```
/// +------------------------------------------------+
/// | OK_MAKER = 1 (1 byte) | SVM binary transaction |  
/// +------------------------------------------------+
/// ```
///
///
/// WASM Buffer `Data` for Error result:
/// ```
/// +------------------------------------------------+
/// | ERR_MAKER = 0 (1 byte) | UTF-8 String (error)  |  
/// +------------------------------------------------+
/// ```
///

#[cfg(target_arch = "wasm32")]
macro_rules! wasm_func_call {
    ($func:ident, $buf_offset:expr) => {{
        match api::wasm::$func($buf_offset as usize) {
            Ok(tx_offset) => tx_offset as _,
            Err(err) => {
                let err_offset = api::wasm::into_error_buffer(err);

                err_offset as _
            }
        }
    }};
}

/// ## WASM Deploy-Template
///
/// Reads the WASM buffer given at parameter `offset` containing a JSON value.
/// Encodes a `deploy-template` binary-transaction using that JSON value.
///
/// Returns a pointer to a new WASM buffer holding the encoded transaction.
/// If the encoding failed, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_encode_deploy_template(offset: i32) -> i32 {
    wasm_func_call!(encode_deploy_template, offset)
}

/// ## WASM Spawn-App
///
/// Reads the WASM buffer given at parameter `offset` containing a JSON value.
/// Encodes a `spawn-app` binary-transaction using that JSON value.
///
/// Returns a pointer to a new WASM buffer holding the encoded transaction.
/// If the encoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_encode_spawn_app(offset: i32) -> i32 {
    wasm_func_call!(encode_spawn_app, offset)
}

/// Decodes the encoded `spawn-app` given as a WASM buffer (parameter `offset`).
///
/// Returns a pointer to a new WASM buffer holding the decoded transaction.
/// If the decoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_decode_spawn_app(offset: i32) -> i32 {
    wasm_func_call!(decode_spawn_app, offset)
}

/// ## WASM Execute-App
///
/// Reads the WASM buffer given at parameter `offset` containing a JSON value.
/// Encodes a `exec-app` binary-transaction using that JSON value.
///
/// Returns a pointer to a new WASM buffer holding the encoded transaction.
/// If the encoding failed, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_encode_exec_app(offset: i32) -> i32 {
    wasm_func_call!(encode_exec_app, offset)
}

/// Decodes the encoded `exec-app` given as a WASM buffer (parameter `offset`).
///
/// Returns a pointer to a new WASM buffer holding the decoded transaction.
/// If the decoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_decode_exec_app(offset: i32) -> i32 {
    wasm_func_call!(decode_exec_app, offset)
}

/// ## WASM Buffer Allocation
///
/// Allocates a new WASM Buffer holding data of `length` bytes.
///
/// For more info read: `api::wasm::alloc`
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_alloc(length: i32) -> i32 {
    let offset = api::wasm::alloc(length as usize);

    offset as _
}

/// ## WASM Buffer Freeing
///
/// Frees the WASM buffer allocated starting from offset `offset`.
///
/// For more info read: `api::wasm::free`
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_free(offset: i32) {
    api::wasm::free(offset as usize);
}

/// ## WASM Buffer Length
///
/// Returns the buffer `Data` byte-length
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_buffer_length(offset: i32) -> i32 {
    let buf_len = api::wasm::wasm_buf_len(offset as usize);

    buf_len as _
}

/// ## WASM Buffer Data
///
/// Returns a pointer to the buffer `Data`
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_buffer_data(offset: i32) -> i32 {
    let (data_offset, _len) = api::wasm::wasm_buf_data_offset(offset as usize);

    data_offset as _
}

/// ## Calldata
///
/// Reads the WASM buffer given at parameter `offset` containing a JSON value.
/// Encodes the `Calldata, and returns a pointer to a new WASM buffer holding the encoded `Calldata`.
/// If the encoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_encode_calldata(offset: i32) -> i32 {
    wasm_func_call!(encode_calldata, offset)
}

/// Decodes the encoded `Calldata` given as a WASM buffer (parameter `offset`).
///
/// Returns a pointer to a new WASM buffer holding the decoded `Calldata`.
/// If the decoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_decode_calldata(offset: i32) -> i32 {
    wasm_func_call!(decode_calldata, offset)
}

/// Decodes the encoded `Receipt` given as a WASM buffer (parameter `offset`).
///
/// Returns a pointer to a new WASM buffer holding the decoded `Receipt`.
/// If the decoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn wasm_decode_receipt(offset: i32) -> i32 {
    wasm_func_call!(decode_receipt, offset)
}
