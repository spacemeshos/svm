#[repr(C)]
pub struct my_struct {
    pub x: u32,

    pub y: u32,
}

mod ffi {
    pub use svm_ffi::svm_byte_array;
}

pub use ffi::svm_byte_array;
