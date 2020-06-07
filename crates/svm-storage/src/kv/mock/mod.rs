mod fake;
mod ffi;

pub use fake::FakeKV;
pub use ffi::{commit, get, head, rewind, set};
