use svm_runtime::Runtime;

use std::collections::HashSet;
use std::ffi::c_void;
use std::sync::Mutex;

use crate::config::Config;
use svm_runtime::PriceResolverRegistry;
use svm_state::GlobalState;

#[derive(Default, Debug)]
pub struct RuntimeTracker {
    tracker: Mutex<HashSet<usize>>,
}

impl RuntimeTracker {
    pub fn count(&self) -> u64 {
        self.tracker.lock().unwrap().len() as u64
    }

    pub fn get(&self, ptr: *mut c_void) -> Option<&'static mut Runtime> {
        let lock = self.tracker.lock().unwrap();
        if lock.contains(&(ptr as usize)) {
            let val = unsafe { Box::leak(Box::<Runtime>::from_raw(ptr.cast())) };
            Some(val)
        } else {
            None
        }
    }

    pub fn alloc(&self) -> *mut c_void {
        let config = Config::get();
        let imports = ("sm".to_string(), wasmer::Exports::new());
        let global_state = if let Some(db_path) = config.db_path {
            GlobalState::new(db_path.as_os_str().to_str().unwrap())
        } else {
            GlobalState::in_memory()
        };

        let runtime = Runtime::new(
            imports,
            global_state,
            PriceResolverRegistry::default(),
            None,
        );

        let boxed = Box::new(runtime);
        let ptr = Box::leak(boxed) as *mut _ as *mut c_void;

        self.tracker.lock().unwrap().insert(ptr as usize);

        ptr
    }

    pub fn free(&self, ptr: *mut c_void) -> Option<()> {
        let found = self.tracker.lock().unwrap().remove(&(ptr as usize));

        if found {
            Some(())
        } else {
            None
        }
    }
}
