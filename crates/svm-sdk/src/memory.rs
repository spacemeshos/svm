extern crate alloc;

use alloc::alloc::{alloc_zeroed, Layout};

pub fn alloc(n: usize) -> usize {
    let layout = Layout::array::<u8>(n).unwrap();

    let ptr: *mut u8 = unsafe { alloc_zeroed(layout) };

    ptr as _
}

#[derive(Debug, PartialEq, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct Ptr(pub usize);
