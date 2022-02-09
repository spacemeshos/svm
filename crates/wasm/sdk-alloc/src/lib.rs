//! This crate implements memory allocation SDK for SVM.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (no Rust stdlib) annotation in order to reduce the compiled WASM size.

#![no_std]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![deny(rustdoc::broken_intra_doc_links)]

extern crate alloc;

use alloc::alloc::Layout;

#[cfg(all(feature = "static-alloc", feature = "dynamic-alloc"))]
compile_error!("Cannot have both `static-alloc` and `dynamic-alloc` features turned-on");

#[cfg(not(any(feature = "static-alloc", feature = "dynamic-alloc")))]
compile_error!("Must have either `static-alloc` or `dynamic-alloc` features turned-on");

#[cfg(feature = "dynamic-alloc")]
extern crate wee_alloc;

#[cfg(feature = "static-alloc")]
pub struct StaticAlloc;

/// When using `static-alloc`, the `global allocator` will be assigned to `StaticAlloc`.
/// It means that each allocation request will be intercepted by it.
///
/// By doing that, we make sure that each allocation will be delegated to the `host` (by calling the `svm_static_alloc`)
/// This feature flag is meant to be used when the compilation target is Wasm.
#[cfg(feature = "static-alloc")]
#[global_allocator]
pub static SVM_ALLOC: StaticAlloc = StaticAlloc;

/// When using `dynamic-alloc`, the `global allocator` will be assigned to `wee_alloc::WeeAlloc`.
/// It means that each allocation request will be intercepted by it.
///
/// The `WeeAlloc` allocator is popular among Blockchain projects since it's footprint is very small
/// and it's also very fast.
///
/// The SVM project is using the `WeeAlloc for its `svm-codec` crate.
/// We compile the codec it into Wasm and because we want to keep it a standalone component (i.e no imports of host functions)
/// Then we can't used the `static-alloc` feature flag (which relies on the existence of `svm_static_alloc` import function).
#[cfg(feature = "dynamic-alloc")]
#[global_allocator]
static SVM_ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// This method uses the process's Global Allocator.
/// It allocates `nbytes` bytes on the Heap.
///
/// The allocated space is zeroed for security and deterministic concerns.
///
/// Returns `Ptr` to the allocated space.

/// When using `static-alloc` the allocation itself is delegated to the host.
#[cfg(feature = "static-alloc")]
pub fn alloc(size: usize) -> Ptr {
    let ptr = unsafe { svm_static_alloc(size as u32) };

    Ptr(ptr as usize)
}

/// Host function import of `svm_static_alloc` under module namespace `svm`
/// The implementation of the host function resides under the `svm-runtime` crate.
#[cfg(feature = "static-alloc")]
#[link(wasm_import_module = "svm")]
extern "C" {
    fn svm_static_alloc(size: u32) -> u32;
}

#[cfg(feature = "static-alloc")]
unsafe impl alloc::alloc::GlobalAlloc for StaticAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size() as u32;

        let offset = unsafe { svm_static_alloc(size) };

        offset as _
    }

    /// We do nothing when being asked to deallocate memory.
    /// This memory leaking is intentional - Running SVM Transactions are short-lived programs.
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        //
    }
}

#[cfg(feature = "dynamic-alloc")]
pub fn alloc(size: usize) -> Ptr {
    let layout = Layout::array::<u8>(size).unwrap();

    let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };

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
