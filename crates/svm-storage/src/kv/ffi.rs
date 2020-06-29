use super::StatefulKV;

use svm_common::State;

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

/// # Discard
///
/// Notifies the `Host` to discard the current executed transaction changes.
pub type DiscardFn = unsafe extern "C" fn();

/// # Checkpoint
///
/// Notifies the `Host` to checkpoint key-value `State`.
/// Returns the `State` derived for the checkpoint.
///
/// Computing a checkpoint doesn't guarantee that the pending changes and checkpoint
/// have been persisted. It's up to the `Host` to determine when to save data for long-term usage.
pub type CheckpointFn = unsafe extern "C" fn(*mut u8);

/// `ExternKV` holds pointers to FFI functions for an external key-value store.
/// It implements the `StatefulKV` traits by delegation to the FFI functions.
pub struct ExternKV {
    /// A function-pointer for a key-value store `Get`
    pub get_fn: GetFn,

    /// A function-pointer for a key-value store `Set`
    pub set_fn: SetFn,

    /// A function-pointer for a key-value store `Discard`
    pub discard_fn: DiscardFn,

    /// A function-pointer for a key-value store `Checkpoint`
    pub checkpoint_fn: CheckpointFn,

    /// The current `State` (optional).
    /// used for testing/development/tracing purposes.
    pub head: Option<State>,
}

impl StatefulKV for ExternKV {
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

    fn set(&mut self, key: &[u8], value: &[u8]) {
        let key_ptr = key.as_ptr();
        let val_ptr = value.as_ptr();

        let key_len = key.len() as u32;
        let val_len = value.len() as u32;

        unsafe {
            (self.set_fn)(key_ptr, key_len, val_ptr, val_len);
        }
    }

    fn discard(&mut self) {
        unsafe {
            (self.discard_fn)();
        }
    }

    fn flush(&mut self) {
        // Do nothing.
        //
        // This implementation of `StatefulKV` is unique in the sense
        // that it's the responsibility of the so-called `Host` to decide
        // on when to actually persist data.
    }

    fn checkpoint(&mut self) -> State {
        unsafe {
            (self.checkpoint_fn)(BUF.as_mut_ptr());

            State::from(BUF.as_ptr())
        }
    }

    fn rewind(&mut self, _state: &State) {
        // This method isn't supposed to be called (only for tesing purposes)
        // since it's the role of the `Host` to manage to current  `State` of an key-value.
    }

    #[must_use]
    fn head(&self) -> State {
        // This method is supposed to be called only for testing/development/tracing purposes.

        self.head.clone().unwrap()
    }
}
