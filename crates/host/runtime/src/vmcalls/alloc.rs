use wasmer::{Bytes, Pages, WASM_PAGE_SIZE};

use std::convert::TryFrom;

use crate::FuncEnv;

/// Statistically allocate `size` bytes within a [`FuncEnv`] `env`.
/// Returns the offset of the newly allocated memory.
pub fn static_alloc(env: &FuncEnv, size: u32) -> u32 {
    dbg!("static_alloc - being asked to allocate {} bytes", size);

    assert!(size > 0);

    let used = used_memory(env);
    let new_used = used + size as u64;

    let allocated = allocated_memory(env);

    if allocated < new_used {
        panic!("Reached memory limit")
    }

    set_used_memory(env, new_used);

    new_used as u32
}

#[allow(unused)]
pub fn dynamic_alloc(env: &FuncEnv, size: u32) -> u32 {
    assert!(size > 0);

    let used = used_memory(env);
    let new_used = used + size as u64;

    let allocated = allocated_memory(env);

    dbg!(format!(
        "allocated (pages): {}, allocated (bytes): {}, new-used: {}, size: {}",
        allocated_pages(env),
        allocated,
        new_used,
        size
    ));

    if allocated < new_used {
        let delta = new_used - allocated;

        grow_memory(env, delta as u32);
    }

    set_used_memory(env, new_used);

    new_used as u32
}

fn allocated_memory(env: &FuncEnv) -> u64 {
    env.borrow().allocated_memory()
}

fn used_memory(env: &FuncEnv) -> u64 {
    env.borrow().used_memory()
}

fn grow_memory(env: &FuncEnv, bytes: u32) {
    let bytes = Bytes(bytes as usize);
    let mut pages = Pages::try_from(bytes).unwrap();

    if pages.0 * (WASM_PAGE_SIZE as u32) < bytes.0 as u32 {
        pages = pages + Pages(1);
    }

    let env_ref = env.borrow();
    let memory = env_ref.memory();

    memory.grow(pages).unwrap();
}

fn set_used_memory(env: &FuncEnv, used: u64) {
    env.borrow_mut().set_used_memory(used);
}

fn allocated_pages(env: &FuncEnv) -> u32 {
    env.borrow().memory().size().0
}
