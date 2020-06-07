use super::StatefulKVStore;

use svm_common::State;
use svm_kv::traits::KVStore;

const BUF_SIZE: usize = 1024;
static mut BUF: [u8; BUF_SIZE] = [0; BUF_SIZE];

/// # Get a Key's Value
///
/// Gets a key's matching value.
/// The value buffer to copy the value to is allocated by `SVM`.
/// The buffer maximum size is 1024 bytes. (This value should be more than enough).
///
/// * key_ptr   - a raw pointer to the key's first byte
/// * key_len   - key's byte-length
/// * value_ptr - a raw pointer to the value's buffer first byte
/// * value_len - a pointer
pub type GetFn = unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u32);

/// # Sets a Key's Value
///
/// Sets a value for a key.
///
/// * key_ptr   - a raw pointer to the key's first byte
/// * key_len   - key's byte-length
/// * value_ptr - a raw pointer to the value's first byte
/// * value_len - a raw pointer to the value's first byte
pub type SetFn = unsafe extern "C" fn(*const u8, u32, *const u8, u32);

/// # Head
///
/// Returns the current `State` pointed by the underlying App's key-value store.
/// The word `head` has been chosen for similarity reasons with git.
/// (`HEAD` in git holds a reference to the current commit).
///
/// The `state` buffer to copy the `State` to is allocated by `SVM`.
/// The buffer size will be of 32 bytes (at least).
pub type HeadFn = unsafe extern "C" fn(*mut u8);

/// # Rewind
///
/// Changes the current `State` pointed by the underlying App's key-value store.
/// In git it would be equivalent to doing `git reset COMMIT_SHA --hard`
pub type RewindFn = unsafe extern "C" fn(*const u8);

/// # Commit
///
/// Commits the pending changes of the underlying key-value store.
/// As a side-effect, a new `State` is being computed and the current `State` of the App's storage
/// is being rewinded to it.
///
/// See: `HeadFn` for how to retrieve that new `State`.
pub type CommitFn = unsafe extern "C" fn();

/// `ExternV` holds pointers to FFI functions for an external key-value store.
/// It implements the `svm_kv::traits::KVStore` traits by delegation to the FFI functions.
pub struct ExternKV {
    /// A function-pointer for key-value `Get`
    pub get_fn: GetFn,

    /// A function-pointer for key-value `Set`
    pub set_fn: SetFn,

    /// A function-pointer for key-value `Head`
    pub head_fn: HeadFn,

    /// A function-pointer for key-value `Rewind`
    pub rewind_fn: RewindFn,

    /// A function-pointer for key-value `Commit`
    pub commit_fn: CommitFn,
}

impl KVStore for ExternKV {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;
        let mut value_len = 0;

        unsafe {
            (self.get_fn)(key_ptr, key_len, BUF.as_mut_ptr(), &mut value_len);

            if value_len > 0 {
                let value = std::slice::from_raw_parts(BUF.as_ptr(), value_len as usize);

                Some(value.to_vec())
            } else {
                None
            }
        }
    }

    fn store(&mut self, changes: &[(&[u8], &[u8])]) {
        for (k, v) in changes.iter() {
            let key_ptr = k.as_ptr();
            let val_ptr = v.as_ptr();

            let key_len = k.len() as u32;
            let val_len = v.len() as u32;

            unsafe {
                (self.set_fn)(key_ptr, key_len, val_ptr, val_len);
            }
        }

        unsafe {
            (self.commit_fn)();
        }
    }
}

impl StatefulKVStore for ExternKV {
    fn rewind(&mut self, state: &State) {
        let state = state.as_ptr();

        unsafe {
            (self.rewind_fn)(state);
        }
    }

    #[must_use]
    fn head(&self) -> State {
        unsafe {
            (self.head_fn)(BUF.as_mut_ptr());

            State::from(BUF.as_ptr())
        }
    }
}
