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
/// let array = svm_byte_array::default();

/// assert_eq!(0, array.length);
/// assert_eq!(std::ptr::null(), array.bytes);
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

///
/// # Example
///
/// ```rust
/// use std::string::FromUtf8Error;
/// use svm_runtime_c_api::svm_byte_array;
///
/// let s = "Hello World!";
/// let ptr = s.as_ptr();
/// let array: svm_byte_array = s.into();
/// assert_eq!(ptr, array.bytes);
///
/// let s: Result<String, FromUtf8Error> = array.into();
/// assert_eq!("Hello World!".to_string(), s.unwrap());
/// ```
///
impl From<&str> for svm_byte_array {
    fn from(s: &str) -> Self {
        let bytes = s.as_ptr();
        let length = s.len() as u32;

        svm_byte_array { bytes, length }
    }
}

impl From<svm_byte_array> for Result<String, FromUtf8Error> {
    fn from(array: svm_byte_array) -> Self {
        let bytes =
            unsafe { std::slice::from_raw_parts(array.bytes as *mut u8, array.length as usize) };

        String::from_utf8(bytes.to_vec())
    }
}

///
/// # Example
///
/// ```rust
/// use svm_runtime_c_api::svm_byte_array;
///
/// let data = vec![0x10u8, 0x20u8, 0x30u8];
/// let ptr = data.as_ptr();
///
/// let array: svm_byte_array = data.into();
/// assert_eq!(ptr, array.bytes);
/// assert_eq!(3, array.length);
/// ```
///
impl From<&[u8]> for svm_byte_array {
    fn from(slice: &[u8]) -> Self {
        let bytes = slice.as_ptr();
        let length = slice.len() as u32;

        svm_byte_array { bytes, length }
    }
}

impl From<Vec<u8>> for svm_byte_array {
    fn from(vec: Vec<u8>) -> Self {
        (&vec[..]).into()
    }
}
