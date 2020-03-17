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
/// let res: Result<Address, String> = Address::try_from(bytes);
/// assert_eq!(Address::of("@someone"), res.unwrap());
/// ```
///
impl_from_svm_byte_array!(Address);
impl_into_svm_byte_array!(Address);
