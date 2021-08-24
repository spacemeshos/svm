use std::convert::TryFrom;
use std::string::FromUtf8Error;

use svm_types::Type;

use crate::tracking;

/// FFI representation for a byte-array
///
/// # Examples
///
/// ```rust
/// use svm_runtime_ffi::svm_byte_array;
/// use svm_types::Type;
///
/// use std::convert::TryFrom;
/// use std::string::FromUtf8Error;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct svm_byte_array {
    bytes: *const u8,
    length: u32,
    capacity: u32,
    type_id: usize,
}

impl svm_byte_array {
    /// Creates a new [`svm_byte_array`] out of its raw parts.
    pub unsafe fn from_raw_parts(
        bytes: *const u8,
        length: u32,
        capacity: u32,
        type_id: usize,
    ) -> Self {
        Self {
            bytes,
            length,
            capacity,
            type_id,
        }
    }

    /// Creates a new [`svm_byte_array`] backed by a buffer of zeros sized `size`.
    pub fn with_capacity(size: usize, ty: Type) -> Self {
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

    /// Returns a shared slice over the contents.
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.bytes, self.length as usize) }
    }

    /// Returns a mutable slice over the contents.
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.bytes as _, self.length as usize) }
    }

    /// Copies `self` into a new [`Vec`].
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_slice().to_vec()
    }

    /// Total number of allocated bytes.
    ///
    /// It may be unequal and bigger than `length` if the `svm_byte_array` instance is an alias to
    /// an instance of a data structure such as `Vec` (which in order to properly get de-allocated
    /// needs first to be re-constructed using the proper allocated capacity).
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Number of allocated bytes
    pub fn len(&self) -> u32 {
        self.length
    }

    /// The [`Type`] associated with the data represented by `bytes`.
    /// It's the interned value of the type. (For more info see `tracking::interning.rs`)
    pub fn type_id(&self) -> usize {
        self.type_id
    }
}

///
/// # Examples
///
/// ```rust
/// use svm_runtime_ffi::svm_byte_array;
///
/// let array = svm_byte_array::default();
///
/// assert_eq!(array.len(), 0);
/// assert_eq!(array.capacity(), 0);
/// ```
///
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

impl AsRef<[u8]> for svm_byte_array {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
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
        // data is cloned here, so the new `String` won't be merely an alias,
        // and `bytes` will still require a separate de-allocation.
        //
        // Making it an alias is unsafe because the data may not have
        // been dynamically allocated, or not by Rust's global allocator.

        String::from_utf8(bytes.to_vec())
    }
}

impl TryFrom<svm_byte_array> for String {
    type Error = FromUtf8Error;

    fn try_from(value: svm_byte_array) -> Result<Self, Self::Error> {
        String::try_from(&value)
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
