#![no_std]
#![feature(maybe_uninit_uninit_array)]

//! This crate implements Storage SDK API for SVM.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (no Rust stdlib) annotation in order to reduce the compiled WASM size.

#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

mod ext;
mod mock;
mod traits;

pub use ext::ExtStorage;
pub use mock::MockStorage;
pub use traits::Storage;

use svm_sdk_types::{Address, Amount};

pub fn get32<S: Storage>(var_id: u32) -> u32 {
    S::get32(var_id)
}

pub fn set32<S: Storage>(var_id: u32, value: u32) {
    S::set32(var_id, value)
}

pub fn get64<S: Storage>(var_id: u32) -> u64 {
    S::get64(var_id)
}

pub fn set64<S: Storage>(var_id: u32, value: u64) {
    S::set64(var_id, value)
}

pub fn get_bool<S: Storage>(var_id: u32) -> bool {
    let value = S::get32(var_id);

    match value {
        0 => false,
        1 => true,
        _ => unreachable!(),
    }
}

pub fn set_bool<S: Storage>(var_id: u32, value: bool) {
    let value = match value {
        true => 1,
        false => 0,
    };

    S::set32(var_id, value)
}

pub fn get_amount<S: Storage>(var_id: u32) -> Amount {
    let value = get64::<S>(var_id);

    Amount(value)
}

pub fn set_amount<S: Storage>(var_id: u32, value: Amount) {
    let value = value.0;

    set64::<S>(var_id, value);
}

pub fn load160<S: Storage>(var_id: u32) -> &'static [u8] {
    use svm_sdk_alloc::alloc;
    let offset = alloc(20);

    S::load160(var_id, offset);

    unsafe { core::slice::from_raw_parts(offset as *const u8, 20) }
}

pub fn store160<S: Storage>(var_id: u32, slice: &[u8]) {
    let ptr: *const u8 = slice.as_ptr();
    let offset = ptr as usize;

    S::store160(var_id, offset);
}

pub fn get_addr<S: Storage>(var_id: u32) -> Address {
    let slice = load160::<S>(var_id);

    slice.into()
}

pub fn set_addr<S: Storage>(var_id: u32, value: &Address) {
    let slice = value.as_slice();

    store160::<S>(var_id, slice);
}

// Array

pub fn array_get_bool<S: Storage>(var_id: u32, index: usize, length: u32) -> bool {
    let var_id = cell_offset(var_id, index, length);

    get_bool::<S>(var_id)
}

pub fn array_set_bool<S: Storage>(var_id: u32, index: usize, length: u32, value: bool) {
    let var_id = cell_offset(var_id, index, length);

    set_bool::<S>(var_id, value);
}

pub fn array_get32<S: Storage>(var_id: u32, index: usize, length: u32) -> u32 {
    let var_id = cell_offset(var_id, index, length);

    get32::<S>(var_id)
}

pub fn array_set32<S: Storage>(var_id: u32, index: usize, length: u32, value: u32) {
    let var_id = cell_offset(var_id, index, length);

    set32::<S>(var_id, value)
}

pub fn array_get64<S: Storage>(var_id: u32, index: usize, length: u32) -> u64 {
    let var_id = cell_offset(var_id, index, length);

    get64::<S>(var_id)
}

pub fn array_set64<S: Storage>(var_id: u32, index: usize, length: u32, value: u64) {
    let var_id = cell_offset(var_id, index, length);

    set64::<S>(var_id, value)
}

#[inline]
pub fn array_get_amount<S: Storage>(var_id: u32, index: usize, length: u32) -> Amount {
    let value = array_get64::<S>(var_id, index, length);

    Amount(value)
}

#[inline]
pub fn array_set_amount<S: Storage>(var_id: u32, index: usize, length: u32, value: Amount) {
    let value = value.0;

    array_set64::<S>(var_id, index, length, value);
}

pub fn array_get_addr<S: Storage>(var_id: u32, index: usize, length: u32) -> Address {
    let var_id = cell_offset(var_id, index, length);
    let slice = load160::<S>(var_id);

    slice.into()
}

pub fn array_set_addr<S: Storage>(var_id: u32, index: usize, length: u32, value: &Address) {
    let var_id = cell_offset(var_id, index, length);
    let slice = value.as_slice();

    store160::<S>(var_id, slice)
}

#[inline]
fn cell_offset(var_id: u32, index: usize, length: u32) -> u32 {
    let index = index as u32;

    assert!(index < length);

    var_id + index
}
