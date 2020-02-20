use crate::{impl_from_svm_byte_array, impl_into_svm_byte_array};

use svm_common::Address;

///
/// # Example
///
/// ```rust
/// use svm_common::Address;
/// use svm_runtime_c_api::svm_byte_array;
///
/// let bytes: svm_byte_array = Address:of("@someone").into();
///
/// let addr: Result<Address, String> = bytes.into();
/// assert_eq!(Address::of("@someone"), addr.unwrap());
/// ```
///
impl_from_svm_byte_array!(Address);
impl_into_svm_byte_array!(Address);
