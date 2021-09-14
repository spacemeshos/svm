use lazy_static::lazy_static;
use svm_runtime::DefaultRuntime;

use std::collections::HashSet;
use std::ffi::c_void;
use std::sync::Mutex;

lazy_static! {
    static ref RUNTIMES: Mutex<HashSet<usize>> = Mutex::default();
}

pub struct RuntimeTracker;

impl RuntimeTracker {
    pub fn count() -> u64 {
        RUNTIMES.lock().unwrap().len() as u64
    }

    pub fn get(ptr: *mut c_void) -> Option<&'static mut DefaultRuntime> {
        let lock = RUNTIMES.lock().unwrap();
        if lock.contains(&(ptr as usize)) {
            let val = unsafe { Box::leak(Box::<DefaultRuntime>::from_raw(ptr.cast())) };
            Some(val)
        } else {
            None
        }
    }

    pub fn alloc() -> *mut c_void {
        use svm_runtime::testing::create_memory_runtime;

        let boxed = Box::new(create_memory_runtime());
        let ptr = Box::leak(boxed) as *mut _ as *mut c_void;

        RUNTIMES.lock().unwrap().insert(ptr as usize);

        ptr
    }

    pub fn free(ptr: *mut c_void) -> Option<()> {
        let found = RUNTIMES.lock().unwrap().remove(&(ptr as usize));

        if found {
            Some(())
        } else {
            None
        }
    }
}
