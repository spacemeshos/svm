use super::FakeKV;
use crate::kv::{ExternKV, StatefulKV};

use std::slice;
use std::sync::Mutex;

use svm_common::State;

use lazy_static::lazy_static;

// This file contains a mock implementation from the `Host`'s angle.
//
// The `Host` (i.e `go-spacemesh` but theoretically other Full-Node)
// exposes to `ExternKV` (the `FFI_KV` below) the following functions pointers:
// * `get`
// * `set`
// * `discard`
// * `checkpoint`
//
// Now, the mock implementation of these functions (i.e the `Host`) will be to use `FakeKV`
// which is an in-memory implementation of the `StatefulKV` trait. (the static `KV` below).
//
// +------------------------------------------------+
// |                                                |
// |     `Host` (mock for `go-spacemesh`)           |
// |  implements: `get, set, discard, checkpoint`   |
// |                                                |
// |   /-\                                          |
// +----|-------------------------------------------+
// |    |                                           |
// |    |     SVM Runtime (uses `StatefulKV`)       |
// |    |                                           |
// |    |                                           |
// !    !----  `ExternKV` (`impl StatefulKV`)       |
// |                                                |
// +------------------------------------------------+
//

lazy_static! {
    static ref KV: Mutex<FakeKV> = Mutex::new(FakeKV::new());
    static ref FFI_KV: ExternKV = ExternKV {
        get_fn: get,
        set_fn: set,
        discard_fn: discard,
        checkpoint_fn: checkpoint,
        head: None,
    };
}

macro_rules! kv {
    () => {{
        KV.lock().unwrap()
    }};
}

pub unsafe extern "C" fn get(
    key_ptr: *const u8,
    key_len: u32,
    value_ptr: *mut u8,
    value_len: *mut u32,
) {
    let key = slice::from_raw_parts(key_ptr, key_len as usize);
    let value = kv!().get(key);

    match value {
        Some(value) => {
            std::ptr::copy(value.as_ptr(), value_ptr, value.len());
            *value_len = value.len() as u32;
        }
        None => *value_len = 0,
    }
}

pub unsafe extern "C" fn set(
    key_ptr: *const u8,
    key_len: u32,
    value_ptr: *const u8,
    value_len: u32,
) {
    let key = slice::from_raw_parts(key_ptr, key_len as usize);
    let value = slice::from_raw_parts(value_ptr, value_len as usize);

    kv!().set(key, value);
}

pub unsafe extern "C" fn discard() {
    kv!().discard()
}

pub unsafe extern "C" fn checkpoint(state_ptr: *mut u8) {
    let state = kv!().checkpoint();

    std::ptr::copy(state.as_ptr(), state_ptr, State::len());
}
