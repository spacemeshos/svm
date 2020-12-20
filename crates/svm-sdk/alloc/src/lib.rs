#![no_std]
#![feature(maybe_uninit_uninit_array)]

//! This crate implements memory allocation SDK for SVM.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (no Rust stdlib) annotation in order to reduce the compiled WASM size.

#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

extern crate alloc;

use alloc::alloc::{alloc_zeroed, Layout};

#[global_allocator]
pub static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// This method uses the process's Global Allocator.
/// It allocates `nbytes` bytes on the Heap.
///
/// The allocated space is zeroed for security and deterministic concerns.
///
/// Returns `Ptr` to the allocated space.
pub fn alloc(nbytes: usize) -> Ptr {
    let layout = Layout::array::<u8>(nbytes).unwrap();

    let ptr: *mut u8 = unsafe { alloc_zeroed(layout) };

    Ptr(ptr as _)
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
}
