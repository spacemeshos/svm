//!
//! # WASM API
//!
//! The following API methods are available to Wasm code via FFI.
//! In order to output a `.wasm` file run (or run `./build.sh` under the crate root directory).
//!
//! ```bash
//! cargo +nightly build --release --target wasm32-unknown-unknown
//! ```
//!
//! The emitted `svm_codec.wasm` is being tested in the `examples/test.js`
//! In order to build and test run `./run.sh` under the `examples` directory.
//!
//! The CI of the `SVM` also runs the js tests and outputs `svm_codec.wasm` under the artifacts.
//!
//!
//! ## WASM API Usage
//!
//! Before calling `wasm_deploy / wasm_spawn / wasm_call` we need first to allocate
//! a WASM buffer using the `wasm_alloc` method. After the buffer isn't needed anymore, make sure to
//! call the `wasm_free` method. (otherwise it'll be a memory-leak).
//!
//! The data returned by `wasm_deploy / wasm_spawn / wasm_call` is a pointer to a new allocated
//! WASM buffer. This WASM buffer is allocated internally by the method and have to be freed later too using `wasm_free`.
//!
//!
//! WASM Buffer `Data` for Success result:
//!
//! ```text
//! +------------------------------------------------+
//! | OK_MAKER = 1 (1 byte) | SVM binary transaction |  
//! +------------------------------------------------+
//! ```
//!
//! WASM Buffer `Data` for Error result:
//!
//! ```text
//! +------------------------------------------------+
//! | ERR_MAKER = 0 (1 byte) | UTF-8 String (error)  |  
//! +------------------------------------------------+
//! ```
//!

use crate::api;
use crate::api::json::JsonError;

/// ## WASM `Deploy Template`
///
/// Reads the WASM buffer given at parameter `offset` containing a JSON value.
/// Encodes a `Deploy Template` binary-transaction using that JSON value.
///
/// Returns a pointer to a new WASM buffer holding the encoded transaction.
/// If the encoding failed, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub extern "C" fn wasm_encode_deploy(offset: i32) -> i32 {
    wasm_func_call(api::wasm::encode_deploy, offset)
}

/// ## WASM `Spawn Account`
///
/// Reads the WASM buffer given at parameter `offset` containing a JSON value.
/// Encodes a `Spawn Account` binary-transaction using that JSON value.
///
/// Returns a pointer to a new WASM buffer holding the encoded transaction.
/// If the encoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub extern "C" fn wasm_encode_spawn(offset: i32) -> i32 {
    wasm_func_call(api::wasm::encode_spawn, offset)
}

/// Decodes the encoded `Spawn Account` given as a WASM buffer (parameter `offset`).
///
/// Returns a pointer to a new WASM buffer holding the decoded transaction.
/// If the decoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub extern "C" fn wasm_decode_spawn(offset: i32) -> i32 {
    wasm_func_call(api::wasm::decode_spawn, offset)
}

/// ## WASM `Call Account`
///
/// Reads the WASM buffer given at parameter `offset` containing a JSON value.
/// Encodes a `Call Account` binary-transaction using that JSON value.
///
/// Returns a pointer to a new WASM buffer holding the encoded transaction.
/// If the encoding failed, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub extern "C" fn wasm_encode_call(offset: i32) -> i32 {
    wasm_func_call(api::wasm::encode_call, offset)
}

/// Decodes the encoded `Call Account` given as a WASM buffer (parameter `offset`).
///
/// Returns a pointer to a new WASM buffer holding the decoded transaction.
/// If the decoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub extern "C" fn wasm_decode_call(offset: i32) -> i32 {
    wasm_func_call(api::wasm::decode_call, offset)
}

/// ## WASM Buffer Allocation
///
/// Allocates a new WASM Buffer holding data of `length` bytes.
///
/// For more info read: `api::wasm::alloc`
#[no_mangle]
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
pub extern "C" fn wasm_free(offset: i32) {
    api::wasm::free(offset as usize);
}

/// ## WASM Buffer Length
///
/// Returns the buffer `Data` byte-length
#[no_mangle]
pub extern "C" fn wasm_buffer_length(offset: i32) -> i32 {
    let buf_len = api::wasm::wasm_buf_len(offset as usize);

    buf_len as _
}

/// ## WASM Buffer Data
///
/// Returns a pointer to the buffer `Data`
#[no_mangle]
pub extern "C" fn wasm_buffer_data(offset: i32) -> i32 {
    let (data_offset, _len) = api::wasm::wasm_buf_data_offset(offset as usize);

    data_offset as _
}

/// ## Input Data (i.e `CallData/VerifyData`)
///
/// Reads the WASM buffer given at parameter `offset` containing a JSON value.
/// Encodes the `Input Data`, and returns a pointer to a new WASM buffer holding the encoded `Input Data`.
/// If the encoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub extern "C" fn wasm_encode_inputdata(offset: i32) -> i32 {
    wasm_func_call(api::wasm::encode_inputdata, offset)
}

/// Decodes the encoded `Input Data` given as a WASM buffer (parameter `offset`).
///
/// Returns a pointer to a new WASM buffer holding the decoded `Input Data`.
/// If the decoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub extern "C" fn wasm_decode_inputdata(offset: i32) -> i32 {
    wasm_func_call(api::wasm::decode_inputdata, offset)
}

/// Decodes the encoded `Receipt` given as a WASM buffer (parameter `offset`).
///
/// Returns a pointer to a new WASM buffer holding the decoded `Receipt`.
/// If the decoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub extern "C" fn wasm_decode_receipt(offset: i32) -> i32 {
    wasm_func_call(api::wasm::decode_receipt, offset)
}

fn wasm_func_call<F>(f: F, offset: i32) -> i32
where
    F: Fn(usize) -> Result<usize, JsonError>,
{
    match f(offset as usize) {
        Ok(tx_offset) => tx_offset as i32,
        Err(err) => {
            let err_offset = api::wasm::into_error_buffer(err);

            err_offset as _
        }
    }
}
