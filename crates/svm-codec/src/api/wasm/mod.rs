mod deploy_template;
mod error;
mod exec_app;
mod func_buf;
mod spawn_app;

pub use deploy_template::encode_deploy_template;
pub use error::{error_as_string, into_error_buffer};
pub use exec_app::encode_exec_app;
pub use func_buf::encode_func_buf;
pub use spawn_app::encode_spawn_app;

use byteorder::{BigEndian, ByteOrder};

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

/// Allocates a new WASM buffer having `Data` of `length` bytes.
///
/// The total allocation size of the buffer will always be bigger due to the `Header` section.
/// If for the `capacity` of the `Data` will be bigger - it will also increase the amount of allocated data.
pub fn alloc(length: usize) -> usize {
    let buf_len = HEADER_SIZE + length;
    let mut buf = vec![0; buf_len];

    let (ptr, len, cap) = buf.into_raw_parts();
    debug_assert_eq!(len, buf_len);

    // We subtract the `HEADER_SIZE` from `len` and `cap`.
    // Now they will refer to the actual buffer data size.
    // The method `free` should take that into account.
    let len = len - HEADER_SIZE;
    let cap = cap - HEADER_SIZE;

    write_header_u32(ptr, len as u32, HEADER_LEN_OFF);
    write_header_u32(ptr, cap as u32, HEADER_CAP_OFF);

    ptr as usize
}

/// Frees the WASM buffer allocated starting from offset `ptr`.
///
/// The range of WASM Memory cells that need to be released are
/// determined by the WASM buffer `Header`.
pub fn free(ptr: usize) {
    let len = wasm_buf_len(ptr) + HEADER_SIZE;
    let cap = wasm_buf_cap(ptr) + HEADER_SIZE;

    let _vec = unsafe { Vec::from_raw_parts(ptr as *mut u8, len, cap) };
}

#[inline]
pub fn wasm_buf_len(ptr: usize) -> usize {
    read_header_u32(ptr, HEADER_LEN_OFF) as usize
}

#[inline]
fn wasm_buf_cap(ptr: usize) -> usize {
    read_header_u32(ptr, HEADER_CAP_OFF) as usize
}

#[inline]
fn write_header_u32(buf: *mut u8, n: u32, off: usize) {
    unsafe {
        let ptr = buf.add(off);
        let slice = std::slice::from_raw_parts_mut(ptr, 4);

        BigEndian::write_u32(slice, n)
    }
}

#[inline]
fn read_header_u32(ptr: usize, off: usize) -> u32 {
    unsafe {
        let ptr = ptr as *const u8;
        let slice = std::slice::from_raw_parts(ptr.add(off), 4);

        BigEndian::read_u32(slice)
    }
}

/// Given a WASM buffer memory offset in `ptr` parameter,
/// returns a '&[u8]' to its `Header` section.
pub fn wasm_buffer<'a>(ptr: usize) -> &'a [u8] {
    let len = wasm_buf_len(ptr);
    let len = len as usize + HEADER_SIZE;

    unsafe { std::slice::from_raw_parts(ptr as *const u8, len) }
}

/// Given a WASM buffer memory offset in `ptr` parameter,
/// returns a '&[u8]' to its `Data` section.
pub fn wasm_buffer_data<'a>(ptr: usize) -> &'a [u8] {
    let (ptr, len) = wasm_buf_data_ptr(ptr);

    unsafe { std::slice::from_raw_parts(ptr as *const u8, len) }
}

/// Given a WASM buffer memory offset in `ptr` parameter,
/// Returns a 2-item tuple. The left element will be the pointer to the buffer `Data`.
/// The right element will have the buffer `Data` length
pub fn wasm_buf_data_ptr<'a>(ptr: usize) -> (usize, usize) {
    let len = wasm_buf_len(ptr);

    let ptr = ptr as *const u8;
    let data_ptr = unsafe { ptr.add(HEADER_SIZE) as usize };

    (data_ptr, len)
}

/// Given a WASM buffer memory offset in `ptr` parameter,
/// returns a '&mut [u8]' to its `Header` section.
pub fn wasm_buffer_mut<'a>(ptr: usize) -> &'a mut [u8] {
    let len = wasm_buf_len(ptr);
    let total_len = len + HEADER_SIZE;

    unsafe { std::slice::from_raw_parts_mut(ptr as *mut u8, total_len) }
}

/// Consumes a `Vec<u8>`, and copies its data into a new allocated WASM buffer.
///
/// Returns the WASM memory offset of that allocated buffer.
///
/// The WASM buffer should be destroyed later by calling `free` on its address.
/// (Otherwise, it'll be a memory-leak).
pub fn to_wasm_buffer(bytes: &[u8]) -> usize {
    let buf_ptr = alloc(bytes.len());

    let buf: &mut [u8] = wasm_buffer_mut(buf_ptr);

    let src = bytes.as_ptr();
    let dst = buf.as_mut_ptr();

    unsafe {
        std::ptr::copy(src, dst.add(HEADER_SIZE), bytes.len());
    }

    buf_ptr
}

pub fn wasm_buf_data_copy(ptr: usize, offset: usize, data: &[u8]) {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wasm_buffer_alloc_and_free() {
        let data: &'static [u8] = b"Hello World";
        let len = data.len();

        let buf_ptr = alloc(len);

        wasm_buf_data_copy(buf_ptr, 0, data);

        // assert buffer Header `length` and `capacity` fields
        assert_eq!(wasm_buf_len(buf_ptr), len);
        assert_eq!(wasm_buf_cap(buf_ptr), len);

        // assert the buffer data
        assert_eq!(wasm_buffer_data(buf_ptr), b"Hello World");

        // freeing the buffer
        free(buf_ptr);
    }
}
