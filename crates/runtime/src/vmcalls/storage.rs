use std::convert::TryInto;

use crate::FuncEnv;

/// Returns the data stored by variable `var_id` as 32-bit integer.
///
/// # Panics
///
/// Panics when variable `var_id` doesn't exist or when it consumes more than 32 bits.
pub fn get32(env: &FuncEnv, var_id: u32) -> u32 {
    let borrow = env.borrow();
    let storage = borrow.storage();
    storage.get_var_i64(var_id).unwrap() as u32
}

/// Sets the data of variable `var_id` to Little-Endian representation of `value`.
///
/// # Panics
///
/// Panics when variable `var_id` doesn't exist or when it consumes more than 32 bits,
/// or when it has not enough bytes to hold `value`.
pub fn set32(env: &FuncEnv, var_id: u32, value: u32) {
    let mut borrow = env.borrow_mut();
    let storage = borrow.storage_mut();
    storage.set_var_i32(var_id, value as i32).unwrap();
}

/// Returns the data stored by variable `var_id` as 64-bit integer.
///
/// # Panics
///
/// Panics when variable `var_id` doesn't exist or when it consumes more than 64 bits.
pub fn get64(env: &FuncEnv, var_id: u32) -> u64 {
    let borrow = env.borrow();
    let storage = borrow.storage();
    storage.get_var_i64(var_id).unwrap() as u64
}

/// Sets the data of variable `var_id` to Little-Endian representation of `value`.
///
/// # Panics
///
/// Panics when variable `var_id` consumes more than 64-bit,
/// or when it has not enough bytes to hold `value`.
pub fn set64(env: &FuncEnv, var_id: u32, value: u64) {
    let mut borrow = env.borrow_mut();
    let storage = borrow.storage_mut();
    storage.set_var_i64(var_id, value as i64).unwrap();
}

/// Stores memory cells `[mem_ptr, mem_ptr + 1, ..., mem_ptr + 19]` into variable `var_id`.
///
/// # Panics
///
/// Panics if variable `var_id`'s length isn't 20 bytes.
pub fn store160(env: &FuncEnv, mem_ptr: u32, var_id: u32) {
    store::<20>(env, mem_ptr, var_id);
}

/// Loads variable `var_id` data into memory cells `[mem_ptr, mem_ptr + 1, ..., mem_ptr + 19]`
///
/// Returns the variable's length.
///
/// # Panics
///
/// Panics if variable `var_id`'s length isn't 20 bytes.
pub fn load160(env: &FuncEnv, var_id: u32, mem_ptr: u32) {
    load::<20>(env, var_id, mem_ptr);
}

fn store<const N: usize>(env: &FuncEnv, mem_ptr: u32, var_id: u32) {
    let bytes: [u8; N] = {
        let borrow = env.borrow();
        let memory = borrow.memory();
        let start = mem_ptr as usize;
        let view = &memory.view::<u8>()[start..][..N];

        view.iter()
            .map(|cell| cell.get())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap()
    };

    let mut borrow = env.borrow_mut();
    let storage = borrow.storage_mut();
    storage.set_var_bytes(var_id, &bytes[..]).unwrap();
}

fn load<const N: usize>(env: &FuncEnv, var_id: u32, mem_ptr: u32) {
    let borrow = env.borrow();
    let storage = borrow.storage();

    let start = mem_ptr as usize;
    let view = &borrow.memory().view::<u8>()[start..][..N];

    let bytes = storage.get_var_array::<N>(var_id).unwrap();
    for (cell, &byte) in view.iter().zip(bytes.iter()) {
        cell.set(byte);
    }
}
