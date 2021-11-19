extern crate alloc;
extern crate std;

use alloc::vec;
use alloc::vec::Vec;

use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::sync::Once;

use svm_sdk_types::Storage;

/// Regarding why we don't use any concurrency primitives for initializing `STORAGE`
/// see the explanation of `MockHost`.

static INIT: Once = Once::new();

static mut STORAGE: MaybeUninit<InnerStorage> = MaybeUninit::uninit();

#[derive(Debug, Clone, PartialEq)]
enum Var {
    I32(u32),
    I64(u64),
    Blob(Vec<u8>),
}

pub struct InnerStorage {
    vars: HashMap<u32, Var>,
}

impl InnerStorage {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn get32(&self, var_id: u32) -> u32 {
        let var = self.var(var_id, || Var::I32(0));

        match var {
            Var::I32(v) => v,
            _ => unreachable!(),
        }
    }

    pub fn get64(&self, var_id: u32) -> u64 {
        let var = self.var(var_id, || Var::I64(0));

        match var {
            Var::I64(v) => v,
            _ => unreachable!(),
        }
    }

    pub fn set32(&mut self, var_id: u32, value: u32) {
        self.set_var(var_id, Var::I32(value));
    }

    pub fn set64(&mut self, var_id: u32, value: u64) {
        self.set_var(var_id, Var::I64(value));
    }

    pub fn store160(&mut self, var_id: u32, offset: usize) {
        self.store_vec(var_id, offset, 20);
    }

    pub fn load160(&self, var_id: u32, offset: usize) {
        self.load_vec(var_id, offset, 20)
    }

    fn var<F>(&self, var_id: u32, default: F) -> Var
    where
        F: Fn() -> Var,
    {
        self.vars.get(&var_id).unwrap_or(&default()).clone()
    }

    fn set_var(&mut self, var_id: u32, var: Var) {
        self.vars.insert(var_id, var);
    }

    fn store_vec(&mut self, var_id: u32, offset: usize, len: usize) {
        let bytes = self.from_raw_parts(offset, len);
        let vec = bytes.to_vec();

        self.set_var(var_id, Var::Blob(vec))
    }

    fn load_vec(&self, var_id: u32, offset: usize, len: usize) {
        let var = self.var(var_id, || Var::Blob(vec![0; len]));

        match var {
            Var::Blob(vec) => unsafe {
                let src: *const u8 = vec.as_ptr();
                let dst = offset as *mut u8;
                let len = vec.len();

                core::ptr::copy_nonoverlapping(src, dst, len)
            },
            _ => unreachable!(),
        }
    }

    fn from_raw_parts(&self, offset: usize, len: usize) -> &[u8] {
        unsafe { core::slice::from_raw_parts(offset as *const u8, len) }
    }

    fn clear(&mut self) {
        self.vars.clear();
    }
}

pub struct MockStorage;

impl MockStorage {
    pub fn instance() -> &'static mut InnerStorage {
        unsafe {
            INIT.call_once(|| {
                STORAGE = MaybeUninit::new(InnerStorage::new());
            });

            std::mem::transmute(STORAGE.as_mut_ptr())
        }
    }

    pub fn clear() {
        let mut storage = Self::instance();

        storage.clear();
    }

    fn from_raw_parts<'a>(offset: usize, len: usize) -> &'a [u8] {
        unsafe { core::slice::from_raw_parts(offset as *const u8, len) }
    }
}

impl Storage for MockStorage {
    fn get32(var_id: u32) -> u32 {
        let mut storage = Self::instance();

        storage.get32(var_id)
    }

    fn get64(var_id: u32) -> u64 {
        let mut storage = Self::instance();

        storage.get64(var_id)
    }

    fn set32(var_id: u32, value: u32) {
        let mut storage = Self::instance();

        storage.set32(var_id, value)
    }

    fn set64(var_id: u32, value: u64) {
        let mut storage = Self::instance();

        storage.set64(var_id, value)
    }

    fn store160(var_id: u32, offset: usize) {
        let mut storage = Self::instance();

        storage.store160(var_id, offset)
    }

    fn load160(var_id: u32, offset: usize) {
        let mut storage = Self::instance();

        storage.load160(var_id, offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_sdk_alloc::alloc;

    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref TEST_LOCK: Mutex<()> = Mutex::new(());
    }

    fn storage_clear() {
        MockStorage::clear();
    }

    fn test(f: fn() -> ()) {
        // Holding `guard` throughout the test-lifetime.
        // By doing that, we make sure that the tests are running in a linear-order (one test at a time)..
        // That's crucial since `MockStorage` serves as a shared-memory resource.
        let guard = TEST_LOCK.lock().unwrap();

        storage_clear();

        f();
    }

    #[test]
    fn storage_mock_get32_set32() {
        test(|| {
            let var1 = 1;
            let var2 = 2;

            MockStorage::set32(var1, 10);
            MockStorage::set32(var2, 20);

            let v = MockStorage::get32(var1);
            assert_eq!(v, 10u32);

            let v = MockStorage::get32(var2);
            assert_eq!(v, 20u32);
        });
    }

    #[test]
    fn storage_mock_get64_set64() {
        test(|| {
            let var1 = 1;
            let var2 = 2;

            MockStorage::set64(var1, 10);
            MockStorage::set64(var2, 20);

            let v = MockStorage::get64(var1);
            assert_eq!(v, 10u64);

            let v = MockStorage::get64(var2);
            assert_eq!(v, 20u64);
        });
    }

    macro_rules! check_load_store {
        ($n:expr, $load_fn:ident, $store_fn:ident) => {{
            test(|| {
                let var1 = 1;
                let var2 = 2;
                let n = 20;

                let addr1 = vec![0x10u8; n];
                let addr2 = vec![0x20u8; n];

                let off1 = alloc(n).offset();
                let off2 = alloc(n).offset();

                MockStorage::$store_fn(var1, addr1.as_ptr() as usize);
                MockStorage::$store_fn(var2, addr2.as_ptr() as usize);

                MockStorage::$load_fn(var1, off1);
                MockStorage::$load_fn(var2, off2);

                let slice1 = MockStorage::from_raw_parts(off1, n);
                let slice2 = MockStorage::from_raw_parts(off2, n);

                assert_eq!(slice1, vec![0x10; n]);
                assert_eq!(slice2, vec![0x20; n]);
            })
        }};
    }

    #[test]
    fn storage_mock_load160_store160() {
        check_load_store!(20, load160, store160);
    }
}
