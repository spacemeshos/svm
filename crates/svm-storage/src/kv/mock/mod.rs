mod fake;
mod ffi;

pub use fake::FakeKV;
pub use ffi::{checkpoint, discard, flush, get, set};
