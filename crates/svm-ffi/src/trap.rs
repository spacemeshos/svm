use std::convert::TryFrom;

pub use crate::svm_byte_array;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct svm_trap_t {
    pub error: svm_byte_array,
}

impl From<svm_trap_t> for String {
    fn from(trap: svm_trap_t) -> String {
        match String::try_from(trap.error) {
            Ok(s) => s,
            Err(..) => "svm_trap_t (exact error message had an interpretation error.".to_string(),
        }
    }
}
