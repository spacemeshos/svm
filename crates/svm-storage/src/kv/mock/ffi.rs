use super::FakeKV;
use crate::kv::{ExternKV, StatefulKVStore};

use std::slice;
use std::sync::Mutex;

use lazy_static::lazy_static;

use svm_common::State;
use svm_kv::traits::KVStore;

lazy_static! {
    static ref KV: Mutex<FakeKV> = Mutex::new(FakeKV::new());
    static ref FFI_KV: ExternKV = ExternKV {
        get_fn: get,
        set_fn: set,
        head_fn: head,
        rewind_fn: rewind,
        commit_fn: commit
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

    // ...
}

pub unsafe extern "C" fn head(state_ptr: *mut u8) {
    let state = kv!().head();

    std::ptr::copy(state.as_ptr(), state_ptr, State::len());
}

pub unsafe extern "C" fn rewind(state_ptr: *const u8) {
    let state = State::from(state_ptr);

    kv!().rewind(&state);
}

pub unsafe extern "C" fn commit() {
    //
}
