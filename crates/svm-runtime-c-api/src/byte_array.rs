use std::{convert::TryFrom, string::FromUtf8Error};

/// FFI representation for a byte-array
///
/// # Example
///
/// ```rust
/// use std::{convert::TryFrom, string::FromUtf8Error};
/// use svm_runtime_c_api::svm_byte_array;
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

///
/// # Example
///
/// ```rust
/// use svm_runtime_c_api::svm_byte_array;
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

        /// data is cloned here, so the new `String` won't be merely an alias,
        /// and `bytes` will still require a separate deallocation.
        ///
        /// Making it an alias is unsafe because the data may not have
        /// been dynamically allocated, or not by Rust's global allocator.
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
