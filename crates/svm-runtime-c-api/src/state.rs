use crate::{impl_from_svm_byte_array, impl_into_svm_byte_array};

use svm_common::State;

///
/// # Example
///
/// ```rust
/// use svm_common::State;
/// use svm_runtime_c_api::svm_byte_array;
///
/// let data = [0x10, 0x20, 0x30];
/// let state = State::from(&data);
/// let bytes: svm_byte_array = state.into();
///
/// let state: Result<State, String> = bytes.into();
/// assert_eq!(State::from(&data), state.unwrap());
/// ```
///
impl_from_svm_byte_array!(State);
impl_into_svm_byte_array!(State);
