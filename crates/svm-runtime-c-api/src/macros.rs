///
/// # Example
///
/// ```rust
/// use std::convert::TryFrom;
///
/// use svm_common::Address;
/// use svm_runtime_c_api::svm_byte_array;
///
/// let bytes: svm_byte_array = Address::of("aaaa").into();
/// assert_eq!(Address::len(), bytes.length as usize);
///
/// let res: Result<Address, String> = Address::try_from(bytes);
/// assert_eq!(Address::of("aaaa"), res.unwrap());
/// ```
///
#[macro_export]
macro_rules! impl_from_svm_byte_array {
    ($struct:ident) => {
	impl std::convert::TryFrom<$crate::svm_byte_array> for $struct {
	    type Error = String;

	    fn try_from(bytes: $crate::svm_byte_array) -> Result<Self, Self::Error> {
		if bytes.length != $struct::len() as u32 {
		    return Err(format!(
			"Wrong `length` value for `svm_byte_array` representing `{}` (expected: {}, got: {})",
			stringify!($struct),
			$struct::len(),
			bytes.length
		    ));
		}

		let slice: &[u8] = unsafe { std::slice::from_raw_parts(bytes.bytes, bytes.length as usize) };

		Ok($struct::from(slice))
	    }
	}
    }
}

///
/// # Example
///
/// ```rust
/// use std::convert::TryFrom;
///
/// use svm_common::Address;
/// use svm_runtime_c_api::svm_byte_array;
///
/// let bytes: svm_byte_array = Address::of("@someone").into();
/// assert_eq!(Address::len(), bytes.length as usize);
///
/// let res: Result<Address, String> = Address::try_from(bytes);
/// assert_eq!(Address::of("@someone"), res.unwrap());
/// ```
///
#[macro_export]
macro_rules! impl_into_svm_byte_array {
    ($struct:ident) => {
        impl From<$struct> for $crate::svm_byte_array {
            fn from(value: $struct) -> Self {
                // `bytes` is a copy of the underlying bytes.
                let bytes = value.bytes();

                debug_assert_eq!($struct::len(), bytes.len());

                // API consumer will have to manually destroy `svm_byte_array`

                let bytes: &[u8] = Box::leak(Box::new(bytes));
                let length = bytes.len() as u32;

                $crate::svm_byte_array {
                    bytes: bytes.as_ptr(),
                    length,
                }
            }
        }
    };
}
