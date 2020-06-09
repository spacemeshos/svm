// mod ffi;
mod mock;
mod traits;

pub use ffi::{CommitFn, ExternKV, GetFn, HeadFn, RewindFn, SetFn};
pub use mock::FakeKV;
pub use traits::StatefulKV;
