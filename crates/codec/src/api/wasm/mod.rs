//! WASM API

mod call;
mod inputdata;
mod deploy;
mod error;
mod receipt;
mod spawn;

pub use call::{decode_call, encode_call};
pub use inputdata::{decode_inputdata, encode_inputdata};
pub use deploy::encode_deploy;
pub use error::{error_as_string, into_error_buffer};
pub use receipt::decode_receipt;
pub use spawn::{decode_spawn, encode_spawn};

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
    unsafe {
        let offset = offset as *const u8;
        let slice = std::slice::from_raw_parts(offset.add(off), 4);

        let bytes: [u8; 4] = [slice[0], slice[1], slice[2], slice[3]];

        u32::from_be_bytes(bytes)
    }
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

#[cfg(test)]
mod test {
    use super::*;

    fn wasm_buf_data_copy(ptr: usize, offset: usize, data: &[u8]) {
        let buf: &mut [u8] = wasm_buffer_mut(ptr);
        let len = wasm_buf_len(ptr);

        // asserting there is no overflow
        assert!(offset + data.len() - 1 < len as usize);

        unsafe {
            let src = data.as_ptr();

            let dst = buf.as_mut_ptr();
            let dst = dst.add(HEADER_SIZE).add(offset);

            std::ptr::copy(src, dst, data.len());
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
}
