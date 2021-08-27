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

mod buffer;

use serde_json::Value as Json;

use std::convert::TryInto;

use crate::api;
use crate::api::json::JsonError;
use buffer::Buffer;

// BUFFERS OPERATIONS
// ------------------

/// ## WASM Buffer Allocation
///
/// Allocates a new WASM Buffer holding data of `length` bytes.
///
/// For more info read: `api::wasm::alloc`
#[no_mangle]
pub extern "C" fn wasm_alloc(length: i32) -> *mut u8 {
    Buffer::alloc(length.try_into().unwrap()).ptr()
}

/// ## WASM Buffer Freeing
///
/// Frees the WASM buffer allocated starting from ptr `ptr`.
///
/// For more info read: `api::wasm::free`
#[no_mangle]
pub unsafe extern "C" fn wasm_free(ptr: *mut u8) {
    Buffer::from_ptr(ptr).free();
}

/// ## WASM Buffer Length
///
/// Returns the buffer `Data` byte-length
#[no_mangle]
pub unsafe extern "C" fn wasm_buffer_length(ptr: *mut u8) -> i32 {
    Buffer::from_ptr(ptr).len().try_into().unwrap()
}

/// ## WASM Buffer Data
///
/// Returns a pointer to the buffer `Data`
#[no_mangle]
pub unsafe extern "C" fn wasm_buffer_data(ptr: *mut u8) -> *mut u8 {
    Buffer::from_ptr(ptr).ptr().add(8)
}

// ENCODERS & DECODERS
// -------------------

/// ## Input Data (i.e `CallData/VerifyData`)
///
/// Reads the WASM buffer given at parameter `ptr` containing a JSON value.
/// Encodes the `Input Data`, and returns a pointer to a new WASM buffer holding the encoded `Input Data`.
/// If the encoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_encode_inputdata(ptr: *mut u8) -> *mut u8 {
    wasm_call_json(api::json::encode_inputdata, ptr)
}

/// Decodes the encoded `Input Data` given as a WASM buffer (parameter `ptr`).
///
/// Returns a pointer to a new WASM buffer holding the decoded `Input Data`.
/// If the decoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_decode_inputdata(ptr: *mut u8) -> *mut u8 {
    wasm_call_json(api::json::decode_inputdata, ptr)
}

/// Decodes the encoded `Receipt` given as a WASM buffer (parameter `ptr`).
///
/// Returns a pointer to a new WASM buffer holding the decoded `Receipt`.
/// If the decoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_decode_receipt(ptr: *mut u8) -> *mut u8 {
    wasm_call_json(api::json::decode_receipt, ptr)
}

/// ## WASM `Deploy Template`
///
/// Reads the WASM buffer given at parameter `ptr` containing a JSON value.
/// Encodes a `Deploy Template` binary-transaction using that JSON value.
///
/// Returns a pointer to a new WASM buffer holding the encoded transaction.
/// If the encoding failed, the returned WASM buffer will contain a String
/// containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_encode_deploy(ptr: *mut u8) -> *mut u8 {
    wasm_call_raw(api::json::deploy_template, ptr)
}

/// ## WASM `Spawn Account`
///
/// Reads the WASM buffer given at parameter `ptr` containing a JSON value.
/// Encodes a `Spawn Account` binary-transaction using that JSON value.
///
/// Returns a pointer to a new WASM buffer holding the encoded transaction.
/// If the encoding fails, the returned WASM buffer will contain a String
/// containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_encode_spawn(ptr: *mut u8) -> *mut u8 {
    wasm_call_raw(api::json::encode_spawn, ptr)
}

/// Decodes the encoded `Spawn Account` given as a WASM buffer (parameter `ptr`).
///
/// Returns a pointer to a new WASM buffer holding the decoded transaction.
/// If the decoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_decode_spawn(ptr: *mut u8) -> *mut u8 {
    wasm_call_json(api::json::decode_spawn, ptr)
}

/// ## WASM `Call Account`
///
/// Reads the WASM buffer given at parameter `ptr` containing a JSON value.
/// Encodes a `Call Account` binary-transaction using that JSON value.
///
/// Returns a pointer to a new WASM buffer holding the encoded transaction.
/// If the encoding failed, the returned WASM buffer will contain a String
/// containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_encode_call(ptr: *mut u8) -> *mut u8 {
    wasm_call_json(api::json::encode_call, ptr)
}

/// Decodes the encoded `Call Account` given as a WASM buffer (parameter
/// `ptr`).
///
/// Returns a pointer to a new WASM buffer holding the decoded transaction.
/// If the decoding fails, the returned WASM buffer will contain a String
/// containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_decode_call(ptr: *mut u8) -> *mut u8 {
    wasm_call_json(api::json::decode_call, ptr)
}

// UTILITIES
// ---------

unsafe fn wasm_call_raw<F>(decode: F, ptr: *mut u8) -> *mut u8
where
    F: Fn(&str) -> Result<Vec<u8>, JsonError>,
{
    let buf = Buffer::from_ptr(ptr);
    let json_s = std::str::from_utf8(buf.as_ref()).expect("Invalid UTF-8");
    let result = decode(json_s);

    result
        .map(|bytes| Buffer::alloc_ok(&bytes))
        .unwrap_or_else(|e| Buffer::alloc_err(e))
        .ptr() as *mut u8
}

unsafe fn wasm_call_json<F>(decode: F, ptr: *mut u8) -> *mut u8
where
    F: Fn(&str) -> Result<Json, JsonError>,
{
    wasm_call_raw(
        |s| {
            decode(s).map(|json| {
                let json_s = serde_json::to_string(&json).expect("JSON serialization error");
                json_s.into_bytes()
            })
        },
        ptr,
    )
}
