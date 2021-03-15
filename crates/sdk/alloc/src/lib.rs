//! This crate implements memory allocation SDK for SVM.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (no Rust stdlib) annotation in order to reduce the compiled WASM size.

#![no_std]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

use core::panic;

extern crate alloc;

// use core::mem::MaybeUninit;

// use alloc::alloc::{GlobalAlloc, Layout};

// #[cfg(not(target_arch = "wasm32"))]
// const HEAP_SIZE: usize = 1024 * 10 * 10 * 64;

// #[cfg(not(target_arch = "wasm32"))]
// static HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

// // in order to use the following `Global Allocator` one should
// // call `extern crate svm_sdk;` (instead of `use svm_sdk;`)
// //
// //
// //
// // #[global_allocator]
// // pub static ALLOC: LimitedAllocator = LimitedAllocator::new();

// pub struct LimitedAllocator {
//     allocated: u64,
// }

// unsafe impl Send for LimitedAllocator {}
// unsafe impl Sync for LimitedAllocator {}

// impl LimitedAllocator {
//     const fn new() -> Self {
//         Self { allocated: 0 }
//     }

//     fn allocated(&self) -> u64 {
//         self.allocated
//     }

//     unsafe fn set_allocated(&self, val: u64) {
//         let ptr: *mut u64 = &self.allocated as *const u64 as _;
//         let dest = &mut *ptr;

//         core::mem::replace(dest, val);
//     }

//     #[cfg(not(target_arch = "wasm32"))]
//     #[inline]
//     unsafe fn address_of(&self, count: u64) -> *mut u8 {
//         let start = HEAP.as_ptr() as *mut u8;

//         start.offset(count as isize)
//     }

//     #[cfg(target_arch = "wasm32")]
//     #[inline]
//     unsafe fn address_of(&self, count: u64) -> *mut u8 {
//         let start = self.allocated();
//         let end = start + count;

//         end as _
//     }
// }

// unsafe impl GlobalAlloc for LimitedAllocator {
//     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//         let size = layout.size() as u64;

//         let old_allocated = self.allocated();
//         let new_allocated = old_allocated + size;

//         if new_allocated > HEAP_SIZE as u64 {
//             // allocation fails
//             return core::ptr::null_mut();
//         }

//         self.set_allocated(new_allocated);

//         ALLOC.address_of(old_allocated)
//     }

//     unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
//         // do-nothing
//     }
// }

/// This method uses the process's Global Allocator.
/// It allocates `nbytes` bytes on the Heap.
///
/// The allocated space is zeroed for security and deterministic concerns.
///
/// Returns `Ptr` to the allocated space.

pub fn alloc(size: usize) -> Ptr {
    use alloc::alloc::Layout;

    let layout = Layout::array::<u8>(size).unwrap();

    let ptr = unsafe { alloc::alloc::alloc(layout) };

    Ptr(ptr as usize)
}

/// WASM memory addresses are represented as `32` or `64` bit.
pub struct Ptr(usize);

impl Ptr {
    /// Returns the pointed address as an integer
    pub fn offset(&self) -> usize {
        self.0
    }

    /// Returns the pointed address as a raw pointer.
    pub fn as_ptr(&self) -> *const u8 {
        self.0 as _
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.0 as _
    }
}
