use std::string::FromUtf8Error;

/// FFI representation for a byte-array
///
/// # Example
///
/// ```rust
/// use std::string::FromUtf8Error;
/// use svm_runtime_c_api::svm_byte_array;
///
/// let s1 = "Hello World!".to_string();
/// let ptr = s1.as_ptr();
/// let length = s1.len() as u32;
/// let bytes = svm_byte_array { bytes: ptr, length };
///
/// let s2: Result<String, FromUtf8Error> = bytes.into();
/// assert_eq!(s1, s2.unwrap());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct svm_byte_array {
    /// Raw pointer to the beginning of array.
    pub bytes: *const u8,

    /// Number of bytes,
    pub length: u32,
}

///
/// # Example
///
/// ```rust
/// use svm_runtime_c_api::svm_byte_array;
///
/// let bytes = svm_byte_array::default();

/// assert_eq!(0, bytes.length);
/// assert_eq!(std::ptr::null(), bytes.bytes);
/// ```
///
impl Default for svm_byte_array {
    fn default() -> Self {
        Self {
            bytes: std::ptr::null(),
            length: 0,
        }
    }
}

impl From<svm_byte_array> for Result<String, FromUtf8Error> {
    fn from(value: svm_byte_array) -> Self {
        let bytes =
            unsafe { std::slice::from_raw_parts(value.bytes as *mut u8, value.length as usize) };

        String::from_utf8(bytes.to_vec())
    }
}
