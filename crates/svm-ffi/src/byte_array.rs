use std::convert::TryFrom;
use std::string::FromUtf8Error;

use byteorder::{BigEndian, ByteOrder};

use svm_types::{Type, WasmType, WasmValue};

use crate::tracking;

/// FFI representation for a byte-array
///
/// # Example
///
/// ```rust
/// use std::convert::TryFrom;
/// use std::string::FromUtf8Error;
///
/// use svm_types::Type;
/// use svm_ffi::svm_byte_array;
///
/// let ty = Type::Str("test string");
///
/// let s1 = "Hello World!".to_string();
/// let bytes: svm_byte_array = (ty, s1).into();
///
/// let s2 = String::try_from(bytes).unwrap();
/// assert_eq!(s2, "Hello World!".to_string());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Clone)]
#[repr(C)]
pub struct svm_byte_array {
    /// Raw pointer to the beginning of array.
    pub bytes: *const u8,

    /// Number of bytes of the data view.
    pub length: u32,

    /// Total number of allocated bytes.
    /// It may be unequal and bigger than `length` if the `svm_byte_array` instance is an alias to
    /// an instance of a data structure such as `Vec` (which in order to properly get deallocated
    /// needs first to be re-constructed using the proper allocated capacity).
    pub capacity: u32,

    /// The `svm_types::Type` associated with the data represented by `bytes`.
    /// It's the interned value of the type. (For more info see `tracking::interning.rs`)
    pub type_id: usize,
}

impl svm_byte_array {
    /// Creates a new `svm_byte_array` backed by a buffer of zeros sized `size`.
    pub fn new(size: usize, ty: Type) -> Self {
        let vec = vec![0u8; size];

        (ty, vec).into()
    }

    /// Releases the memory region starting at `ptr` (of length `length` bytes).
    pub unsafe fn destroy(self) {
        let ptr = self.bytes as *mut u8;
        let length = self.length as usize;
        let capacity = self.capacity as usize;

        let _ = Vec::from_raw_parts(ptr, length, capacity);

        tracking::decrement_live_1(self.type_id)
    }

    /// Copies the WASM values given by `values` into the raw format of `self` (i.e `svm_byte_array`).
    /// The function receives an allocated buffer filled with zeros it should fill-in.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::convert::TryFrom;
    ///
    /// use svm_types::{WasmValue, Type};
    /// use svm_ffi::svm_byte_array;
    ///
    /// let src = vec![WasmValue::I64(10), WasmValue::I32(20), WasmValue::I64(30)];
    ///
    /// // We allocate `dst` with zeros.
    /// let size = 1 + 9 * src.len();
    ///
    /// let ty = Type::of::<Vec<WasmValue>>();
    /// let vec = vec![0u8; size];
    ///
    /// let mut dst: svm_byte_array = (ty, vec).into();
    ///
    /// // We fill-in `dst` with the WASM values given by `src`
    /// unsafe { dst.copy_wasm_values(&src) };
    ///
    /// let copied = Vec::<WasmValue>::try_from(&dst).unwrap();
    /// assert_eq!(copied, src);
    ///
    /// // de-allocate `dst`
    /// unsafe { dst.destroy() };
    /// ```
    pub unsafe fn copy_wasm_values(&mut self, values: &[WasmValue]) {
        assert!(values.len() <= 255);

        let nvalues = values.len() as u8;

        let mut ptr = self.bytes as *mut u8;

        // The first byte signifies the `#values`.
        std::ptr::write::<u8>(ptr, nvalues);
        ptr = ptr.add(1);

        macro_rules! copy_wasm_val {
            ($ty:expr, $val:expr, $size:expr, $bits:expr) => {{
                paste::item! {
                    // First we copy the `type` of the WASM value
                    let type_id: u8 = $ty.into();
                    std::ptr::write::<u8>(ptr, type_id);
                    ptr = ptr.add(1);

                    // We copy the `value` with the data given by `$val`
                    let buf = std::slice::from_raw_parts_mut(ptr as *mut u8, $size);
                    BigEndian::[<write_u $bits>](buf, *$val);
                    ptr = ptr.add($size);
                }
            }};
        };

        for val in values {
            match val {
                WasmValue::I32(v) => copy_wasm_val!(WasmType::I32, v, 4, 32),
                WasmValue::I64(v) => copy_wasm_val!(WasmType::I64, v, 8, 64),
            }
        }
    }
}

// ///
// /// # Example
// ///
// /// ```rust
// /// use svm_ffi::svm_byte_array;
// ///
// /// let array = svm_byte_array::default();
// ///
// /// assert_eq!(std::ptr::null(), array.bytes);
// /// assert_eq!(0, array.length);
// /// assert_eq!(0, array.capacity);
// /// ```
// ///
impl Default for svm_byte_array {
    fn default() -> Self {
        Self {
            bytes: std::ptr::null(),
            length: 0,
            capacity: 0,
            type_id: 0,
        }
    }
}

impl From<(Type, Vec<u8>)> for svm_byte_array {
    fn from((ty, vec): (Type, Vec<u8>)) -> Self {
        let (ptr, len, cap) = vec.into_raw_parts();

        tracking::increment_live(ty);

        svm_byte_array {
            bytes: ptr,
            length: len as u32,
            capacity: cap as u32,
            type_id: tracking::interned_type(ty),
        }
    }
}

impl From<(Type, String)> for svm_byte_array {
    fn from((ty, s): (Type, String)) -> Self {
        let vec = s.into_bytes();

        (ty, vec).into()
    }
}

impl TryFrom<&svm_byte_array> for String {
    type Error = FromUtf8Error;

    fn try_from(bytes: &svm_byte_array) -> Result<Self, Self::Error> {
        let slice: &[u8] = bytes.into();

        // data is cloned here, so the new `String` won't be merely an alias,
        // and `bytes` will still require a separate deallocation.
        //
        // Making it an alias is unsafe because the data may not have
        // been dynamically allocated, or not by Rust's global allocator.
        let vec = slice.to_vec();

        String::from_utf8(vec)
    }
}

impl TryFrom<svm_byte_array> for String {
    type Error = FromUtf8Error;

    fn try_from(value: svm_byte_array) -> Result<Self, Self::Error> {
        String::try_from(&value)
    }
}

impl From<&svm_byte_array> for &[u8] {
    fn from(bytes: &svm_byte_array) -> Self {
        unsafe { std::slice::from_raw_parts(bytes.bytes, bytes.length as usize) }
    }
}

impl From<svm_byte_array> for &[u8] {
    fn from(bytes: svm_byte_array) -> Self {
        (&bytes).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_to_bytes() {
        let mut vec = Vec::with_capacity(4);
        vec.push(0x10u8);
        vec.push(0x20u8);
        vec.push(0x30u8);

        let ptr = vec.as_ptr();
        let bytes: svm_byte_array = ("Vec<u8>".into(), vec).into();

        assert_eq!(ptr, bytes.bytes); // `bytes` is an alias.
        assert_eq!(3, bytes.length);
        assert_eq!(4, bytes.capacity);
    }

    #[test]
    fn string_to_bytes_to_string() {
        let s1 = "Hello World!".to_string();
        let s1_ptr = s1.as_ptr();
        let s1_len = s1.len() as u32;
        let s1_capacity = s1.capacity() as u32;
        let bytes: svm_byte_array = (Type::of::<String>(), s1).into();

        assert_eq!(s1_ptr, bytes.bytes); // `bytes` is an alias.
        assert_eq!(s1_len, bytes.length);
        assert_eq!(s1_capacity, bytes.capacity);

        let s2 = String::try_from(bytes.clone()).unwrap();
        assert_eq!(s2, "Hello World!".to_string());
        assert_ne!(s2.as_ptr(), bytes.bytes); // `s2` is a clone.
        assert_eq!(s2.len() as u32, bytes.length);
        assert_eq!(s2.capacity() as u32, bytes.capacity);
    }
}
