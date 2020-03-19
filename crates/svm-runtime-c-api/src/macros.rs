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
/// dbg!(res);
/// // assert_eq!(Address::of("@someone"), res.unwrap());
/// ```
///
#[macro_export]
macro_rules! impl_from_svm_byte_array {
    ($struct:ident) => {
	impl std::convert::TryFrom<$crate::svm_byte_array> for $struct {
	    type Error = String;

	    fn try_from(value: $crate::svm_byte_array) -> Result<Self, Self::Error> {
		if value.length != $struct::len() as u32 {
		    return Err(format!(
			"Wrong `length` value for `svm_byte_array` representing `{}` (expected: {}, got: {})",
			stringify!($struct),
			$struct::len(),
			value.length
		    ));
		}

		let slice = unsafe { std::slice::from_raw_parts(value.bytes, value.length as usize) };
		let instance = $struct::from(slice);

		Ok(instance)
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
/// panic!(res);
/// // assert_eq!(Address::of("@someone"), res.unwrap());
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

                let ptr = bytes.as_ptr();
                let length = bytes.len() as u32;

                // API consumer will have to manually destroy `svm_byte_array`
                std::mem::forget(bytes);

                $crate::svm_byte_array { bytes: ptr, length }
            }
        }
    };
}
