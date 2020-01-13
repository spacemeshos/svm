use std::ffi::c_void;

mod data_wrapper;
mod register;
mod storage;

pub use data_wrapper::DataWrapper;
pub use register::wasmer_data_reg;
pub use storage::wasmer_data_app_storage;
