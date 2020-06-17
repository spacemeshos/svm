mod spawn_app;

use byteorder::{BigEndian, ByteOrder};

const HEADER_SIZE: usize = 8;

///
/// Each Buffer is prefixed with Header consisting of 8 bytes.
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
/// ### Buffer Header Layout
///
/// +------------------+--------------------+
/// | length (4 bytes) | capacity (4 bytes) |
/// +------------------+--------------------+
///
/// Both `length` and `capacity` are laid out in Big-Endian order
///
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

    write_header_u32(ptr, len as u32, 0);
    write_header_u32(ptr, cap as u32, 4);

    ptr as usize
}

pub fn free(ptr: usize) {
    let ptr = ptr as *mut u8;

    let len = read_header_u32(ptr, 0);
    let cap = read_header_u32(ptr, 4);

    let len = len as usize + HEADER_SIZE;
    let cap = cap as usize + HEADER_SIZE;

    let _vec = unsafe { Vec::from_raw_parts(ptr, len, cap) };
}

fn write_header_u32(buf: *mut u8, n: u32, off: usize) {
    unsafe {
        let ptr = buf.add(off);
        let slice = std::slice::from_raw_parts_mut(ptr, 4);

        BigEndian::write_u32(slice, n)
    }
}

fn read_header_u32(buf: *mut u8, off: usize) -> u32 {
    unsafe {
        let ptr = buf.add(off);
        let slice = std::slice::from_raw_parts(ptr, 4);

        BigEndian::read_u32(slice)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn buffer_alloc_and_free() {
        let data: &'static [u8] = b"Hello World";
        let len = data.len();

        let buf_ptr = alloc(len) as *mut u8;
        let buf_len = HEADER_SIZE + len;

        let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(buf_ptr, buf_len) };

        let src = data.as_ptr();
        let dst = buf.as_mut_ptr();

        unsafe {
            std::ptr::copy(src, dst.add(HEADER_SIZE), len as usize);
        }

        // assert buffer Header `length` and `capacity` fields
        assert_eq!(BigEndian::read_u32(&buf[0..4]), len as u32);
        assert_eq!(BigEndian::read_u32(&buf[4..8]), len as u32);

        // assert the buffer data
        assert_eq!(&buf[HEADER_SIZE..(HEADER_SIZE + len)], b"Hello World");

        // freeing the buffer
        free(buf_ptr as usize);
    }
}
