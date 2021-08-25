//! WASM API

mod error;

pub use error::{error_as_string, into_error_buffer};

use crate::api;
use crate::api::json::JsonError;

const HEADER_LEN_OFF: usize = 0;
const HEADER_CAP_OFF: usize = 4;
const HEADER_SIZE: usize = 8;

const BUF_OK_MARKER: u8 = 1;
const BUF_ERROR_MARKER: u8 = 0;

/// ## WASM Buffer Layout
///
/// Each WASM Buffer contains 2 section: `Header` and `Data`
///
/// +--------------------------------+
/// | Header Section |  Data Section |
/// +--------------------------------+
///
///
/// ### WASM Buffer Header Section
///
/// Each Buffer is prefixed with `Header` consisting of 8 bytes.
///
/// The first 4 bytes are the byte-length of the buffer.
/// The remaining 4 bytes are the capacity byte-length of the buffer.
///
/// The reason we need both `length` and `capacity` and due to the implementation
/// of Rust `Vec`. Even though we use `Vec::with_capacity` we still prefer to store
/// explicitly in the `Header` the `capacity` returned by `Vec#into_raw_parts`.
///
/// See also `Vec#reserve_exact` documentation:
///
/// ```md
/// Note that the allocator may give the collection more space than it
/// requests. Therefore, capacity can not be relied upon to be precisely
/// minimal. Prefer `reserve` if future insertions are expected.
/// ```
///
/// #### WASM Buffer Header Layout
///
/// +------------------+--------------------+
/// | length (4 bytes) | capacity (4 bytes) |
/// +------------------+--------------------+
///
/// Both `length` and `capacity` are laid out in Big-Endian order
///
///
/// ## WASM Buffer Data Section
///
/// Contains the raw data of the buffer.
///
/// Allocates a new WASM buffer having `Data` of `length` bytes.
///
/// The total allocation size of the buffer will always be bigger due to the `Header` section.
/// If for the `capacity` of the `Data` will be bigger - it will also increase the amount of allocated data.
pub fn alloc(length: usize) -> usize {
    let buf_len = HEADER_SIZE + length;
    let buf = vec![0; buf_len];

    let (offset, len, cap) = buf.into_raw_parts();
    debug_assert_eq!(len, buf_len);

    // We subtract the `HEADER_SIZE` from `len` and `cap`.
    // Now they will refer to the actual buffer data size.
    // The method `free` should take that into account.
    let len = len - HEADER_SIZE;
    let cap = cap - HEADER_SIZE;

    write_header_u32(offset, len as u32, HEADER_LEN_OFF);
    write_header_u32(offset, cap as u32, HEADER_CAP_OFF);

    offset as usize
}

/// Frees the WASM buffer allocated starting from offset `offset`.
///
/// The range of WASM Memory cells that need to be released are
/// determined by the WASM buffer `Header`.
pub fn free(offset: usize) {
    let len = wasm_buf_len(offset) + HEADER_SIZE;
    let cap = wasm_buf_cap(offset) + HEADER_SIZE;

    let _vec = unsafe { Vec::from_raw_parts(offset as *mut u8, len, cap) };
}

/// Returns the WASM buffer `length` (excluding the `header`)
#[inline]
pub fn wasm_buf_len(offset: usize) -> usize {
    read_header_u32(offset, HEADER_LEN_OFF) as usize
}

#[inline]
fn wasm_buf_cap(offset: usize) -> usize {
    read_header_u32(offset, HEADER_CAP_OFF) as usize
}

#[inline]
fn write_header_u32(buf: *mut u8, n: u32, off: usize) {
    unsafe {
        let offset = buf.add(off);
        let slice = std::slice::from_raw_parts_mut(offset, 4);

        let bytes: [u8; 4] = n.to_be_bytes();

        std::ptr::copy(bytes.as_ptr(), slice.as_mut_ptr(), 4);
    }
}

#[inline]
fn read_header_u32(offset: usize, off: usize) -> u32 {
    let offset = offset as *const u8;
    let slice = unsafe { std::slice::from_raw_parts(offset.add(off), 4) };

    let bytes: [u8; 4] = [slice[0], slice[1], slice[2], slice[3]];

    u32::from_be_bytes(bytes)
}

/// Given a WASM buffer memory offset in `offset` parameter,
/// returns a '&[u8]' to its `Header` section.
pub fn wasm_buffer<'a>(offset: usize) -> &'a [u8] {
    let len = wasm_buf_len(offset);
    let len = len as usize + HEADER_SIZE;

    unsafe { std::slice::from_raw_parts(offset as *const u8, len) }
}

/// Given a WASM buffer memory offset in `offset` parameter,
/// returns a '&[u8]' to its `Data` section.
pub fn wasm_buffer_data<'a>(offset: usize) -> &'a [u8] {
    let (offset, len) = wasm_buf_data_offset(offset);

    unsafe { std::slice::from_raw_parts(offset as *const u8, len) }
}

/// Given a WASM buffer memory offset in `offset` parameter,
/// Returns a 2-item tuple. The left element will be the pointer to the buffer `Data`.
/// The right element will have the buffer `Data` length
pub fn wasm_buf_data_offset<'a>(offset: usize) -> (usize, usize) {
    let len = wasm_buf_len(offset);

    let offset = offset as *const u8;
    let data_offset = unsafe { offset.add(HEADER_SIZE) as usize };

    (data_offset, len)
}

/// Given a WASM buffer memory offset in `offset` parameter,
/// returns a '&mut [u8]' to its `Header` section.
pub fn wasm_buffer_mut<'a>(offset: usize) -> &'a mut [u8] {
    let len = wasm_buf_len(offset);
    let total_len = len + HEADER_SIZE;

    unsafe { std::slice::from_raw_parts_mut(offset as *mut u8, total_len) }
}

/// Consumes a `Vec<u8>`, and copies its data into a new allocated WASM buffer.
///
/// Returns the WASM memory offset of that allocated buffer.
///
/// The WASM buffer should be destroyed later by calling `free` on its address.
/// (Otherwise, it'll be a memory-leak).
pub fn to_wasm_buffer(bytes: &[u8]) -> usize {
    let buf_offset = alloc(bytes.len());

    let buf: &mut [u8] = wasm_buffer_mut(buf_offset);

    let src = bytes.as_ptr();
    let dst = buf.as_mut_ptr();

    unsafe {
        std::ptr::copy(src, dst.add(HEADER_SIZE), bytes.len());
    }

    buf_offset
}

pub(crate) fn wasm_buf_apply<F>(offset: usize, func: F) -> Result<usize, JsonError>
where
    F: Fn(&str) -> Result<Vec<u8>, JsonError>,
{
    let bytes = wasm_buffer_data(offset);
    let json_s = std::str::from_utf8(bytes)?;
    let result = func(json_s);

    let bytes = match result {
        Err(JsonError::Eof | JsonError::InvalidJson { .. }) => {
            let offset = into_error_buffer(result.unwrap_err());
            return Ok(offset);
        }
        Err(e) => return Err(e),
        Ok(bytes) => bytes,
    };

    let mut buf = Vec::with_capacity(1 + bytes.len());
    buf.push(BUF_OK_MARKER);
    buf.extend_from_slice(&bytes);

    let offset = to_wasm_buffer(&buf);
    Ok(offset)
}

pub(crate) fn wasm_decode<F>(offset: usize, decoder: F) -> Result<usize, JsonError>
where
    F: Fn(&str) -> Result<serde_json::Value, JsonError>,
{
    wasm_buf_apply(offset, |json: &str| {
        let json = decoder(json)?;

        Ok(api::json::to_bytes(&json))
    })
}

/// Decodes a binary Receipt given as an offset to a Wasm buffer,
/// and then returns an offset to a new Wasm buffer holding the decoded Receipt
/// in a JSON format.
pub fn decode_receipt(offset: usize) -> Result<usize, JsonError> {
    wasm_decode(offset, api::json::decode_receipt)
}

/// Given an offset to a Wasm buffer holding the data to be encoded,
/// encodes it and returns an offset to the encoded binary `Input Data` (wrapped within a JSON).
pub(crate) fn encode_inputdata(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, |json: &str| {
        let json = api::json::encode_inputdata(json)?;

        Ok(api::json::to_bytes(&json))
    })
}

/// Given an offset to a Wasm buffer holding a binary `Input Data`,
/// decodes it and returns an offset to be decoded `Input Data` (wrapped within a JSON)
pub(crate) fn decode_inputdata(offset: usize) -> Result<usize, JsonError> {
    wasm_decode(offset, api::json::decode_inputdata)
}

/// Encodes a `Spawn Account` JSON input into SVM binary format.
/// The JSON input is passed by giving WASM memory start address (`offset` parameter).
///
/// Returns an offset to a Wasm buffer holding the encoded transaction (wrapped within a JSON)
pub fn encode_spawn(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, api::json::encode_spawn)
}

/// Decodes a binary `Spawn Account` transaction given as a Wasm buffer (the `offset` parameter),
///
/// and returns a new Wasm buffer holding the decoded transaction (wrapped with a JSON).
pub fn decode_spawn(offset: usize) -> Result<usize, JsonError> {
    wasm_decode(offset, api::json::decode_spawn)
}

/// Encodes a `Deploy Template` json input into SVM a binary format.
/// The json input is passed by giving WASM memory start address (`ptr` parameter).
///
/// Returns a pointer to a `transaction buffer`.
///
/// See also: `alloc` and `free`
pub fn encode_deploy(ptr: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(ptr, api::json::deploy_template)
}

/// Encodes an `Call Account` JSON into SVM binary format.
/// The JSON input is passed by giving WASM memory start address (`ptr` parameter).
///
/// Returns a pointer to a `transaction buffer`.
///
/// See also: `alloc` and `free`
///
pub fn encode_call(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, |json| api::json::encode_call_raw(&json.to_string()))
}

/// Decodes a `Call Account` transaction into a JSON,
/// stores that JSON content into a new Wasm Buffer,
/// and finally returns that Wasm buffer offset
pub fn decode_call(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, |json: &str| {
        let json = api::json::decode_call(json)?;

        Ok(api::json::to_bytes(&json))
    })
}

#[cfg(test)]
mod test {
    use serde_json::Value as Json;
    use serde_json::{json, Value};

    use std::io::Cursor;

    use svm_layout::Layout;
    use svm_types::{
        Address, BytesPrimitive, CodeKind, CodeSection, CtorsSection, DataSection, Gas, GasMode,
        HeaderSection, SpawnReceipt, State, Template,
    };

    use super::*;
    use crate::api::json::serde_types::HexBlob;
    use crate::api::wasm::{free, to_wasm_buffer, wasm_buffer_data, BUF_OK_MARKER};
    use crate::{template, Codec};

    fn wasm_buf_data_copy(ptr: usize, offset: usize, data: &[u8]) {
        let buf: &mut [u8] = wasm_buffer_mut(ptr);
        let len = wasm_buf_len(ptr);

        // asserting there is no overflow
        assert!(offset + data.len() - 1 < len as usize);

        {
            let src = data.as_ptr();

            let dst = buf.as_mut_ptr();
            let dst = unsafe { dst.add(HEADER_SIZE).add(offset) };

            unsafe { std::ptr::copy(src, dst, data.len()) };
        }
    }

    #[test]
    fn wasm_buffer_alloc_and_free() {
        let data: &'static [u8] = b"Hello World";
        let len = data.len();

        let buf_offset = alloc(len);

        wasm_buf_data_copy(buf_offset, 0, data);

        // assert buffer Header `length` and `capacity` fields
        assert_eq!(wasm_buf_len(buf_offset), len);
        assert_eq!(wasm_buf_cap(buf_offset), len);

        // assert the buffer data
        assert_eq!(wasm_buffer_data(buf_offset), b"Hello World");

        // freeing the buffer
        free(buf_offset);
    }

    #[test]
    fn wasm_decode_receipt_valid() {
        let account = Address::repeat(0x10);
        let state = State::repeat(0xA0);
        let logs = Vec::new();

        let receipt = SpawnReceipt {
            version: 0,
            success: true,
            error: None,
            account_addr: Some(account.into()),
            init_state: Some(state),
            returndata: Some(vec![0x10, 0x20]),
            gas_used: Gas::with(10),
            logs,
        };

        let bytes = receipt.encode_to_vec();
        let data = HexBlob(&bytes);
        let json = json!({ "data": data });
        let json = serde_json::to_string(&json).unwrap();

        let json_buf = to_wasm_buffer(json.as_bytes());
        let receipt_buf = decode_receipt(json_buf).unwrap();

        let data = wasm_buffer_data(receipt_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let json: Value = serde_json::from_slice(&data[1..]).unwrap();

        assert_eq!(
            json,
            json!({
                "success": true,
                "type": "spawn-account",
                "account": "1010101010101010101010101010101010101010",
                "gas_used": 10,
                "returndata": "1020",
                "state": "A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0",
                "logs": []
            })
        );

        free(json_buf);
        free(receipt_buf);
    }

    fn wasm_buf_as_json(buf_ptr: usize) -> Json {
        let data = wasm_buffer_data(buf_ptr);
        assert_eq!(data[0], BUF_OK_MARKER);

        let s = unsafe { String::from_utf8_unchecked(data[1..].to_vec()) };
        let json: Json = serde_json::from_str(&s).unwrap();

        json
    }

    #[test]
    fn wasm_encode_inputdata_valid() {
        let json = r#"{
          "abi": ["i32", "address"],
          "data": [10, "102030405060708090A011121314151617181920"]
        }"#;

        // encode
        let json_buf = to_wasm_buffer(json.as_bytes());
        let inputdata = encode_inputdata(json_buf).unwrap();
        let data = wasm_buffer_data(inputdata);
        assert_eq!(data[0], BUF_OK_MARKER);

        // decode
        let data_buf = to_wasm_buffer(&data[1..]);
        let res_buf = decode_inputdata(data_buf).unwrap();

        assert_eq!(
            wasm_buf_as_json(res_buf),
            json!({
              "abi": ["i32", "address"],
              "data": [10, "102030405060708090A011121314151617181920"]
            })
        );

        free(json_buf);
        free(inputdata);
        free(data_buf);
        free(res_buf);
    }

    #[test]
    fn wasm_encode_inputdata_invalid_json() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = encode_inputdata(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert_eq!(error, "The given JSON is syntactically invalid due to EOF.");

        free(json_buf);
        free(error_buf);
    }

    #[test]
    fn wasm_decode_inputdata_invalid_json() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = decode_inputdata(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert_eq!(error, "The given JSON is syntactically invalid due to EOF.");

        free(json_buf);
        free(error_buf);
    }

    #[test]
    fn wasm_spawn_valid() {
        let template_addr = "1122334455667788990011223344556677889900";

        let calldata = api::json::encode_inputdata(
            &json!({
                "abi": ["i32", "i64"],
                "data": [10, 20]
            })
            .to_string(),
        )
        .unwrap();

        let json = json!({
          "version": 1,
          "template": template_addr,
          "name": "My Account",
          "ctor_name": "initialize",
          "calldata": calldata["data"],
        });

        let json = serde_json::to_string(&json).unwrap();
        let json_buf = to_wasm_buffer(json.as_bytes());
        let tx_buf = encode_spawn(json_buf).unwrap();
        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let data = HexBlob(&data[1..]);
        let json = json!({ "data": data });
        let json = serde_json::to_string(&json).unwrap();

        free(json_buf);
        let json_buf = to_wasm_buffer(json.as_bytes());

        free(tx_buf);
        let tx_buf = decode_spawn(json_buf).unwrap();

        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let json: Value = serde_json::from_slice(&data[1..]).unwrap();

        assert_eq!(
            json,
            json!({
                "version": 1,
                "template": template_addr,
                "name": "My Account",
                "ctor_name": "initialize",
                "calldata": {
                    "abi": ["i32", "i64"],
                    "data": [10, 20],
                }
            })
        );

        free(json_buf);
        free(tx_buf);
    }

    #[test]
    fn wasm_spawn_invalid() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = encode_spawn(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert_eq!(error, "The given JSON is syntactically invalid due to EOF.");

        free(json_buf);
        free(error_buf);
    }

    #[test]
    fn wasm_deploy_valid() {
        let json = r#"{
          "name": "My Template",
          "desc": "A few words",
          "code": "C0DE",
          "svm_version": 1,
          "code_version": 2,
          "data": "0000000100000003",
          "ctors": ["init", "start"]
        }"#;

        let json_buf = to_wasm_buffer(json.as_bytes());
        let tx_buf = encode_deploy(json_buf).unwrap();

        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let cursor = Cursor::new(&data[1..]);
        let actual = template::decode(cursor, None).unwrap();

        let code = CodeSection::new(
            CodeKind::Wasm,
            vec![0xC0, 0xDE],
            CodeSection::exec_flags(),
            GasMode::Fixed,
            1,
        );
        let data = DataSection::with_layout(Layout::Fixed(vec![1, 3].into()));
        let ctors = CtorsSection::new(vec!["init".into(), "start".into()]);
        let header = HeaderSection::new(2, "My Template".into(), "A few words".into());

        let expected = Template::new(code, data, ctors).with_header(Some(header));

        assert_eq!(actual, expected);

        free(json_buf);
        free(tx_buf);
    }

    #[test]
    fn wasm_deploy_invalid() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = encode_deploy(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert_eq!(error, "The given JSON is syntactically invalid due to EOF.");

        free(json_buf);
        free(error_buf);
    }

    #[test]
    fn wasm_call_valid() {
        let target = "1122334455667788990011223344556677889900";

        let verifydata = api::json::encode_inputdata(
            &json!({
                "abi": ["bool", "i8"],
                "data": [true, 3]
            })
            .to_string(),
        )
        .unwrap();

        let calldata = api::json::encode_inputdata(
            &json!({
                "abi": ["i32", "i64"],
                "data": [10, 20]
            })
            .to_string(),
        )
        .unwrap();

        let json = json!({
          "version": 1,
          "target": target,
          "func_name": "do_something",
          "verifydata": verifydata["data"],
          "calldata": calldata["data"]
        });

        let json = serde_json::to_string(&json).unwrap();
        let json_buf = to_wasm_buffer(json.as_bytes());
        let tx_buf = encode_call(json_buf).unwrap();

        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let data = HexBlob(&data[1..]);
        let json = json!({ "data": data });
        let json = serde_json::to_string(&json).unwrap();

        free(json_buf);
        let json_buf = to_wasm_buffer(json.as_bytes());

        free(tx_buf);
        let tx_buf = decode_call(json_buf).unwrap();
        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let json: Value = serde_json::from_slice(&data[1..]).unwrap();

        assert_eq!(
            json,
            json!({
                "version": 1,
                "target": target,
                "func_name": "do_something",
                "verifydata": {
                    "abi": ["bool", "i8"],
                    "data": [true, 3],
                },
                "calldata": {
                    "abi": ["i32", "i64"],
                    "data": [10, 20],
                }
            })
        );

        free(json_buf);
        free(tx_buf);
    }

    #[test]
    fn wasm_call_invalid() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = encode_call(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert_eq!(error, "The given JSON is syntactically invalid due to EOF.");

        free(json_buf);
        free(error_buf);
    }
}
