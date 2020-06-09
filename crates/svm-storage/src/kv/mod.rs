mod ffi;
mod mock;
mod traits;

pub use ffi::{DiscardFn, ExternKV, GetFn, SetFn};
pub use mock::FakeKV;
pub use traits::StatefulKV;
