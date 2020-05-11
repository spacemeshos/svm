use super::StatefulKVStore;

use svm_common::State;
use svm_kv::traits::KVStore;

type GetFn = unsafe extern "C" fn(*const u8, u32, *mut u32) -> *const u8;
type HeadFn = unsafe extern "C" fn(*mut u32) -> *const u8;
type RewindFn = unsafe extern "C" fn(*const u8, u32);
type SetFn = unsafe extern "C" fn(*const u8, u32);
type CommitFn = unsafe extern "C" fn();

pub struct ExternKV {
    get_fn: GetFn,
    head_fn: HeadFn,
    rewind_fn: RewindFn,
    set_fn: SetFn,
    commit_fn: CommitFn,
}

impl KVStore for ExternKV {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;

        let mut value_len = 0;

        let value_ptr = unsafe { (self.get_fn)(key_ptr, key_len, &mut value_len) };

        if value_len > 0 {
            let value = unsafe { std::slice::from_raw_parts(value_ptr, value_len as usize) };

            Some(value.to_vec())
        } else {
            None
        }
    }

    fn store(&mut self, changes: &[(&[u8], &[u8])]) {
        for (k, v) in changes.iter() {
            let key_ptr = k.as_ptr();
            let key_len = k.len() as u32;

            unsafe {
                (self.set_fn)(key_ptr, key_len);
            }
        }

        unsafe {
            (self.commit_fn)();
        }
    }
}

impl StatefulKVStore for ExternKV {
    fn rewind(&mut self, state: &State) {
        let length = State::len() as u32;
        let ptr = state.as_ptr();

        unsafe {
            (self.rewind_fn)(ptr, length);
        }
    }

    #[must_use]
    fn head(&self) -> State {
        let mut length = 0u32;
        unsafe {
            let ptr = (self.head_fn)(&mut length);

            assert_eq!(length, State::len() as u32);

            State::from(ptr)
        }
    }
}
