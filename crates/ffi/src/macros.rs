///
/// # Examples
///
/// ```rust
/// use std::convert::TryFrom;
///
/// use svm_types::{Address, Type};
/// use svm_ffi::svm_byte_array;
///
/// let ty = Type::Str("@someone address");
/// let addr = Address::of("@someone");
///
/// let bytes: svm_byte_array = (ty, addr).into();
/// assert_eq!(Address::len(), bytes.len() as usize);
///
/// let res: Result<Address, String> = Address::try_from(bytes);
/// assert_eq!(Address::of("@someone"), res.unwrap());
/// ```
///
#[macro_export]
macro_rules! impl_from_svm_byte_array {
    ($struct:ident) => {
        impl std::convert::TryFrom<$crate::svm_byte_array> for $struct {
            type Error = String;

            fn try_from(bytes: $crate::svm_byte_array) -> Result<Self, Self::Error> {
                if bytes.len() != $struct::len() as u32 {
                    return Err(format!(
                        "Wrong `length` value for `svm_byte_array` representing `{}` (expected: {}, got: {})",
                        stringify!($struct),
                        $struct::len(),
                        bytes.len()
                    ));
                }

                let slice: &[u8] = bytes.as_slice();

                Ok($struct::from(slice))
            }
        }
    }
}

///
/// # Examples
///
/// ```rust
/// use std::convert::TryFrom;
///
/// use svm_types::{Address, Type};
/// use svm_ffi::svm_byte_array;

/// let ty = Type::Str("@someone address");
/// let addr = Address::of("@someone");
///
/// let bytes: svm_byte_array = (ty, addr).into();
/// assert_eq!(Address::len(), bytes.len() as usize);
///
/// let res: Result<Address, String> = Address::try_from(bytes);
/// assert_eq!(Address::of("@someone"), res.unwrap());
/// ```
///
#[macro_export]
macro_rules! impl_into_svm_byte_array {
    ($struct:ident) => {
        impl From<(svm_types::Type, &$struct)> for $crate::svm_byte_array {
            fn from((ty, value): (svm_types::Type, &$struct)) -> Self {
                // `bytes` is a copy of the underlying bytes.
                // and it is of type array (i.e: `[u8; N])`.
                let bytes = value.bytes();

                debug_assert_eq!($struct::len(), bytes.len());

                // API consumer will have to manually destroy `svm_byte_array`

                let bytes: &[u8] = Box::leak(Box::new(bytes));
                let length = bytes.len() as u32;

                crate::tracking::increment_live(ty);

                let type_id = crate::tracking::interned_type(ty);

                let bytes = bytes.as_ptr();
                let capacity = length;

                unsafe { $crate::svm_byte_array::from_raw_parts(bytes, length, capacity, type_id) }
            }
        }

        impl From<(svm_types::Type, $struct)> for $crate::svm_byte_array {
            fn from((ty, value): (svm_types::Type, $struct)) -> Self {
                (ty, (&value)).into()
            }
        }
    };
}
