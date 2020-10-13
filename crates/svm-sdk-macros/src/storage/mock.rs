extern crate alloc;
extern crate std;

use alloc::vec;
use alloc::vec::Vec;

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use lazy_static::lazy_static;

lazy_static! {
    static ref STORAGE: Mutex<InnerStorage> = {
        let storage = InnerStorage::new();

        Mutex::new(storage)
    };
}

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
        let var = self.get_var(var_id, || Var::I32(0));

        match var {
            Var::I32(v) => v,
            _ => unreachable!(),
        }
    }

    pub fn get64(&self, var_id: u32) -> u64 {
        let var = self.get_var(var_id, || Var::I64(0));

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

    pub fn store160(&mut self, var_id: u32, ptr: usize) {
        self.store_vec(var_id, ptr, 20);
    }

    pub fn store256(&mut self, var_id: u32, ptr: usize) {
        self.store_vec(var_id, ptr, 32);
    }

    pub fn load160(&self, var_id: u32, ptr: usize) {
        self.load_vec(var_id, ptr, 20)
    }

    pub fn load256(&self, var_id: u32, ptr: usize) {
        self.load_vec(var_id, ptr, 32)
    }

    fn get_var<F>(&self, var_id: u32, default: F) -> Var
    where
        F: Fn() -> Var,
    {
        self.vars.get(&var_id).unwrap_or(&default()).clone()
    }

    fn set_var(&mut self, var_id: u32, var: Var) {
        self.vars.insert(var_id, var);
    }

    fn store_vec(&mut self, var_id: u32, ptr: usize, len: usize) {
        let bytes = self.from_raw_parts(ptr, len);
        let vec = bytes.to_vec();

        self.set_var(var_id, Var::Blob(vec))
    }

    fn load_vec(&self, var_id: u32, ptr: usize, len: usize) {
        let var = self.get_var(var_id, || Var::Blob(vec![0; len]));

        match var {
            Var::Blob(vec) => unsafe {
                let src: *const u8 = vec.as_ptr();
                let dst = ptr as *mut u8;
                let len = vec.len();

                core::ptr::copy_nonoverlapping(src, dst, len)
            },
            _ => unreachable!(),
        }
    }

    fn from_raw_parts(&self, ptr: usize, len: usize) -> &[u8] {
        unsafe { core::slice::from_raw_parts(ptr as *const u8, len) }
    }

    fn clear(&mut self) {
        self.vars.clear();
    }
}

pub struct Storage;

impl Storage {
    pub fn get32(var_id: u32) -> u32 {
        let storage = storage();

        storage.get32(var_id)
    }

    pub fn get64(var_id: u32) -> u64 {
        let storage = storage();

        storage.get64(var_id)
    }

    pub fn set32(var_id: u32, value: u32) {
        let mut storage = storage();

        storage.set32(var_id, value)
    }

    pub fn set64(var_id: u32, value: u64) {
        let mut storage = storage();

        storage.set64(var_id, value)
    }

    pub fn store160(var_id: u32, ptr: usize) {
        let mut storage = storage();

        storage.store160(var_id, ptr)
    }

    pub fn store256(var_id: u32, ptr: usize) {
        let mut storage = storage();

        storage.store256(var_id, ptr)
    }

    pub fn load160(var_id: u32, ptr: usize) {
        let storage = storage();

        storage.load160(var_id, ptr)
    }

    pub fn load256(var_id: u32, ptr: usize) {
        let storage = storage();

        storage.load256(var_id, ptr)
    }

    pub fn clear() {
        let mut storage = storage();

        storage.clear();
    }

    pub fn from_raw_parts<'a>(ptr: usize, len: usize) -> &'a [u8] {
        unsafe { core::slice::from_raw_parts(ptr as *const u8, len) }
    }
}

fn storage() -> MutexGuard<'static, InnerStorage> {
    STORAGE.lock().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_sdk::memory::alloc;

    lazy_static! {
        static ref TEST_LOCK: Mutex<()> = Mutex::new(());
    }

    fn storage_clear() {
        Storage::clear();
    }

    fn test(f: fn() -> ()) {
        let guard = TEST_LOCK.lock().unwrap();

        storage_clear();

        f();

        // Holding `guard` throughout the test-lifetime.
        // By doing that, we make sure that the tests are running in a linear-order (one test at a time)..
        // That's crucial since `Storage` serves as a shared-memory resource.
        drop(guard);
    }

    #[test]
    fn storage_mock_get32_set32() {
        test(|| {
            let var1 = 1;
            let var2 = 2;

            Storage::set32(var1, 10);
            Storage::set32(var2, 20);

            let v = Storage::get32(var1);
            assert_eq!(v, 10u32);

            let v = Storage::get32(var2);
            assert_eq!(v, 20u32);
        });
    }

    #[test]
    fn storage_mock_get64_set64() {
        test(|| {
            let var1 = 1;
            let var2 = 2;

            Storage::set64(var1, 10);
            Storage::set64(var2, 20);

            let v = Storage::get64(var1);
            assert_eq!(v, 10u64);

            let v = Storage::get64(var2);
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

                let ptr1 = alloc(n);
                let ptr2 = alloc(n);

                Storage::$store_fn(var1, addr1.as_ptr() as usize);
                Storage::$store_fn(var2, addr2.as_ptr() as usize);

                Storage::$load_fn(var1, ptr1);
                Storage::$load_fn(var2, ptr2);

                let slice1 = Storage::from_raw_parts(ptr1, n);
                let slice2 = Storage::from_raw_parts(ptr2, n);

                assert_eq!(slice1, vec![0x10; n]);
                assert_eq!(slice2, vec![0x20; n]);
            })
        }};
    }

    #[test]
    fn storage_mock_load160_store160() {
        check_load_store!(20, load160, store160);
    }

    #[test]
    fn storage_mock_load256_store256() {
        check_load_store!(20, load256, store256);
    }
}
