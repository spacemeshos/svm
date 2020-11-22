use std::convert::TryFrom;
use std::string::FromUtf8Error;

use byteorder::{BigEndian, ByteOrder};

use svm_types::{WasmType, WasmValue};

/// FFI representation for a byte-array
///
/// # Example
///
/// ```rust
/// use std::convert::TryFrom;
/// use std::string::FromUtf8Error;
/// use svm_ffi::svm_byte_array;
///
/// let s1 = "Hello World!".to_string();
/// let bytes: svm_byte_array = s1.into();
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
}

impl svm_byte_array {
    pub unsafe fn destroy(self) {
        let ptr = self.bytes as *mut u8;
        let length = self.length as usize;
        let capacity = self.capacity as usize;

        let _ = Vec::from_raw_parts(ptr, length, capacity);
    }

    /// Copies the WASM values given by `values` into the raw format of `self`.
    /// This function doesn't modify `self` WASM values layout - it only overrides its values.
    ///
    /// The WASM values layout `self` should been allocated before-hand.
    /// (see `crate::alloc_wasm_values`)
    ///
    /// In case the layout of `self` is different from what has been given by `values`
    /// the function returns `false` (this is an undefined-behavior)
    ///
    /// When the copying process succeeds - the function returns `true`.
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::convert::TryFrom;
    ///
    /// use svm_types::{WasmType, WasmValue};
    /// use svm_ffi::{svm_byte_array, alloc_wasm_values};
    ///
    /// let types = vec![WasmType::I64, WasmType::I32, WasmType::I64];
    /// let src = vec![WasmValue::I64(10), WasmValue::I32(20), WasmValue::I64(30)];
    ////
    /// let mut dst: svm_byte_array = alloc_wasm_values(&types);
    ///
    /// let is_ok = unsafe { dst.copy_wasm_values(&src) };
    /// assert!(is_ok);
    ///
    /// let dst = Vec::<WasmValue>::try_from(&dst).unwrap();
    /// assert_eq!(dst, src);
    /// ```
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use svm_types::{WasmType, WasmValue};
    /// use svm_ffi::{svm_byte_array, alloc_wasm_values};
    ///
    /// let types = vec![WasmType::I64, WasmType::I32];
    /// let src = vec![WasmValue::I32(10)];
    ////
    /// let mut dst: svm_byte_array = alloc_wasm_values(&types);
    /// let is_ok = unsafe { dst.copy_wasm_values(&src) };
    ///
    /// // `types.len() = 2` and `src.len() = 1`
    /// assert!(is_ok == false);
    /// ```
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use svm_types::{WasmType, WasmValue};
    /// use svm_ffi::{svm_byte_array, alloc_wasm_values};
    ///
    /// let types = vec![WasmType::I64, WasmType::I32];
    /// let src = vec![WasmValue::I32(10), WasmValue::I32(20)];
    ////
    /// let mut dst: svm_byte_array = alloc_wasm_values(&types);
    /// let is_ok = unsafe { dst.copy_wasm_values(&src) };
    ///
    /// // `src` and `types` have different layouts
    /// assert!(is_ok == false);
    /// ```
    ///
    pub unsafe fn copy_wasm_values(&mut self, values: &[WasmValue]) -> bool {
        let nvalues = std::ptr::read::<u8>(self.bytes) as usize;

        if nvalues != values.len() {
            return false;
        }

        // `ptr` starts at the 2nd byte pointed by `self.bytes`.
        // (the skipped byte stands for the `#values`).
        let mut ptr = self.bytes.add(1);

        macro_rules! copy_wasm_val {
            ($val:expr, $size:expr, $bits:expr) => {{
                paste::item! {
                    // First, we want to ensure that `ptr` points now on `i32` type as well,
                    // (otherwise, the copy aborts)

                    let ty = WasmType::try_from(*ptr);
                    if ty.is_err() {
                        // `svm_byte_array` content is corrupted!
                        return false;
                    }

                    if ty.unwrap() != WasmType::[<I $bits>] {
                        // host function didn't abide to the agreed function return types.
                        return false;
                    }

                    // skip the type byte (we've already verified it's valid).
                    ptr = ptr.add(1);

                    // we override the zero-ed `i32` value with the data given by `val`
                    let buf = std::slice::from_raw_parts_mut(ptr as *mut u8, $size);

                    BigEndian::[<write_u $bits>](buf, *$val);

                    // advances `ptr` to point to the next wasm value.
                    ptr = ptr.add($size);
                }
            }};
        };

        for val in values {
            match val {
                WasmValue::I32(v) => copy_wasm_val!(v, 4, 32),
                WasmValue::I64(v) => copy_wasm_val!(v, 8, 64),
            }
        }

        true
    }
}

///
/// # Example
///
/// ```rust
/// use svm_ffi::svm_byte_array;
///
/// let array = svm_byte_array::default();
///
/// assert_eq!(std::ptr::null(), array.bytes);
/// assert_eq!(0, array.length);
/// assert_eq!(0, array.capacity);
/// ```
///
impl Default for svm_byte_array {
    fn default() -> Self {
        Self {
            bytes: std::ptr::null(),
            length: 0,
            capacity: 0,
        }
    }
}

impl From<String> for svm_byte_array {
    fn from(s: String) -> Self {
        s.into_bytes().into()
    }
}

impl From<Vec<u8>> for svm_byte_array {
    fn from(vec: Vec<u8>) -> Self {
        let (ptr, len, cap) = vec.into_raw_parts();

        svm_byte_array {
            bytes: ptr,
            length: len as u32,
            capacity: cap as u32,
        }
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
        let bytes: svm_byte_array = vec.into();

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
        let bytes: svm_byte_array = s1.into();

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
