use std::collections::HashSet;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::sync::Mutex;

#[derive(Debug)]
pub struct ResourceTracker<T> {
    // https://stackoverflow.com/a/50201389/5148606
    // `fn() -> T` always implements `Send` and `Sync`.
    phantom: PhantomData<fn() -> T>,
    pointers: Mutex<HashSet<usize>>,
}

impl<T> ResourceTracker<T> {
    pub fn count(&self) -> u64 {
        self.pointers.lock().unwrap().len() as u64
    }

    pub fn get(&self, ptr: *mut c_void) -> Option<&'static mut T> {
        let lock = self.pointers.lock().unwrap();
        if lock.contains(&(ptr as usize)) {
            let val = unsafe { Box::leak(Box::<T>::from_raw(ptr.cast())) };
            Some(val)
        } else {
            None
        }
    }

    pub fn alloc(&self, item: T) -> *mut c_void {
        let boxed = Box::new(item);
        let ptr = Box::leak(boxed) as *mut _ as *mut c_void;

        self.pointers.lock().unwrap().insert(ptr as usize);

        ptr
    }

    pub fn free(&self, ptr: *mut c_void) -> Option<()> {
        let found = self.pointers.lock().unwrap().remove(&(ptr as usize));

        if found {
            Some(())
        } else {
            None
        }
    }
}

impl<T> Default for ResourceTracker<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
            pointers: Mutex::new(HashSet::new()),
        }
    }
}
