//! This crate implements memory allocation SDK for SVM.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (no Rust stdlib) annotation in order to reduce the compiled WASM size.

#![no_std]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

extern crate alloc;

/// This method uses the process's Global Allocator.
/// It allocates `nbytes` bytes on the Heap.
///
/// The allocated space is zeroed for security and deterministic concerns.
///
/// Returns `Ptr` to the allocated space.

#[cfg(feature = "static-alloc")]
#[link(wasm_import_module = "svm")]
extern "C" {
    fn svm_static_alloc(size: u32) -> u32;
}

#[cfg(feature = "static-alloc")]
pub fn alloc(size: usize) -> Ptr {
    let ptr = unsafe { svm_static_alloc(size as u32) };

    Ptr(ptr as usize)
}

#[cfg(not(feature = "static-alloc"))]
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