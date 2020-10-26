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

pub fn alloc(n: usize) -> usize {
    let layout = Layout::array::<u8>(n).unwrap();

    let ptr: *mut u8 = unsafe { alloc_zeroed(layout) };

    ptr as _
}

#[derive(Debug, PartialEq, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct Ptr(pub usize);
