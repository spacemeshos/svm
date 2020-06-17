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
/// explicitly in the `Header` the `capacity` returned by `Vec#into_raw_parts`
///
/// ### Buffer Header Layout
///
/// +------------------+--------------------+
/// | Length (4 bytes) | Capacity (4 bytes) |
/// +------------------+--------------------+
///
/// Both `Length` and `Capacity` are laid out in Big-Endian order
///
#[no_mangle]
pub unsafe extern "C" fn alloc(length: i32) -> i32 {
    let buf_len = HEADER_SIZE + length as usize;
    let mut buf = vec![0; buf_len];

    let (ptr, len, cap) = buf.into_raw_parts();

    write_header_u32(ptr, len as u32, 0);
    write_header_u32(ptr, cap as u32, 4);

    ptr as *const u8 as i32
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: i32) {
    let ptr = ptr as *mut u8;

    let len = read_header_u32(ptr, 0);
    let cap = read_header_u32(ptr, 4);

    let _vec = Vec::from_raw_parts(ptr, len as usize, cap as usize);
}

fn write_header_u32(buf: *mut u8, n: u32, off: usize) {
    let ptr = unsafe { buf.offset(off as isize) };
    let slice = unsafe { std::slice::from_raw_parts_mut(ptr, 4) };

    BigEndian::write_u32(slice, n)
}

fn read_header_u32(buf: *mut u8, off: usize) -> u32 {
    let slice = header_slice(buf, off);

    BigEndian::read_u32(slice)
}

#[inline]
fn header_slice<'a>(buf: *mut u8, off: usize) -> &'a mut [u8] {
    unsafe {
        let ptr = buf.offset(off as isize);
        std::slice::from_raw_parts_mut(ptr, 4)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn buffer_alloc_and_free() {
        let buf: &'static [u8] = b"Hello World";
        // let len = buf.len() as i32;

        // let ptr = unsafe { alloc(len) };
    }
}
