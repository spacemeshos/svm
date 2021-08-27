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
pub extern "C" fn wasm_alloc(length: u32) -> usize {
    Buffer::alloc(length.try_into().unwrap()).offset()
}

/// ## WASM Buffer Freeing
///
/// Frees the WASM buffer allocated starting from offset `offset`.
///
/// For more info read: `api::wasm::free`
#[no_mangle]
pub unsafe extern "C" fn wasm_free(offset: usize) {
    Buffer::from_offset(offset).free();
}

/// ## WASM Buffer Length
///
/// Returns the buffer `Data` byte-length
#[no_mangle]
pub unsafe extern "C" fn wasm_buffer_length(offset: usize) -> usize {
    Buffer::from_offset(offset).as_ref().len()
}

/// ## WASM Buffer Data
///
/// Returns a pointer to the buffer `Data`
#[no_mangle]
pub unsafe extern "C" fn wasm_buffer_data(offset: usize) -> usize {
    Buffer::from_offset(offset).offset() + 8
}

// ENCODERS & DECODERS
// -------------------

/// ## Input Data (i.e `CallData/VerifyData`)
///
/// Reads the WASM buffer given at parameter `offset` containing a JSON value.
/// Encodes the `Input Data`, and returns a pointer to a new WASM buffer holding the encoded `Input Data`.
/// If the encoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_encode_inputdata(offset: usize) -> usize {
    wasm_call_json(api::json::encode_inputdata, offset)
}

/// Decodes the encoded `Input Data` given as a WASM buffer (parameter `offset`).
///
/// Returns a pointer to a new WASM buffer holding the decoded `Input Data`.
/// If the decoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_decode_inputdata(offset: usize) -> usize {
    wasm_call_json(api::json::decode_inputdata, offset)
}

/// Decodes the encoded `Receipt` given as a WASM buffer (parameter `offset`).
///
/// Returns a pointer to a new WASM buffer holding the decoded `Receipt`.
/// If the decoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_decode_receipt(offset: usize) -> usize {
    wasm_call_json(api::json::decode_receipt, offset)
}

/// ## WASM `Deploy Template`
///
/// Reads the WASM buffer given at parameter `offset` containing a JSON value.
/// Encodes a `Deploy Template` binary-transaction using that JSON value.
///
/// Returns a pointer to a new WASM buffer holding the encoded transaction.
/// If the encoding failed, the returned WASM buffer will contain a String
/// containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_encode_deploy(offset: usize) -> usize {
    wasm_call_raw(api::json::deploy_template, offset)
}

/// ## WASM `Spawn Account`
///
/// Reads the WASM buffer given at parameter `offset` containing a JSON value.
/// Encodes a `Spawn Account` binary-transaction using that JSON value.
///
/// Returns a pointer to a new WASM buffer holding the encoded transaction.
/// If the encoding fails, the returned WASM buffer will contain a String
/// containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_encode_spawn(offset: usize) -> usize {
    wasm_call_raw(api::json::encode_spawn, offset)
}

/// Decodes the encoded `Spawn Account` given as a WASM buffer (parameter `offset`).
///
/// Returns a pointer to a new WASM buffer holding the decoded transaction.
/// If the decoding fails, the returned WASM buffer will contain a String containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_decode_spawn(offset: usize) -> usize {
    wasm_call_json(api::json::decode_spawn, offset)
}

/// ## WASM `Call Account`
///
/// Reads the WASM buffer given at parameter `offset` containing a JSON value.
/// Encodes a `Call Account` binary-transaction using that JSON value.
///
/// Returns a pointer to a new WASM buffer holding the encoded transaction.
/// If the encoding failed, the returned WASM buffer will contain a String
/// containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_endode_call(offset: usize) -> usize {
    wasm_call_json(api::json::encode_call, offset)
}

/// Decodes the encoded `Call Account` given as a WASM buffer (parameter
/// `offset`).
///
/// Returns a pointer to a new WASM buffer holding the decoded transaction.
/// If the decoding fails, the returned WASM buffer will contain a String
/// containing the error message.
#[no_mangle]
pub unsafe extern "C" fn wasm_decode_call(offset: usize) -> usize {
    wasm_call_json(api::json::decode_call, offset)
}

// UTILITIES
// ---------

unsafe fn wasm_call_raw<F>(decode: F, offset: usize) -> usize
where
    F: Fn(&str) -> Result<Vec<u8>, JsonError>,
{
    let buf = Buffer::from_offset(offset);
    let json_s = std::str::from_utf8(buf.as_ref()).expect("Invalid UTF-8");
    let result = decode(json_s);

    result
        .map(|bytes| Buffer::alloc_ok(&bytes))
        .unwrap_or_else(|e| Buffer::alloc_err(e))
        .offset()
}

unsafe fn wasm_call_json<F>(decode: F, offset: usize) -> usize
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
        offset,
    )
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn encode_then_decode_call() {
        let call = json!({
          "version": 0,
          "target": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
          "func_name": "do_work",
          "verifydata": {"abi": [], "data": []},
          "calldata": {"abi": [], "data": []},
        })
        .to_string();

        let mut buf = Buffer::alloc(call.as_bytes().len() as u32);
        buf.as_mut().clone_from_slice(call.as_bytes());

        let res_buf_offset = unsafe { wasm_endode_call(buf.offset() as usize) };
        let res_buf = unsafe { Buffer::from_offset(res_buf_offset as usize) };

        let encoded = res_buf.as_result().unwrap().unwrap();

        assert!(std::str::from_utf8(encoded).is_ok());
    }
}
