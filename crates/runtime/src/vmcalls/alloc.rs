use wasmer::{Bytes, Pages, WASM_PAGE_SIZE};

use std::convert::TryFrom;

use crate::Context;

pub fn allocate(ctx: &Context, size: u32) -> u32 {
    assert!(size > 0);

    let used = used_memory(ctx);
    let new_used = used + size as u64;

    let allocated = allocated_memory(ctx);

    dbg!(format!(
        "allocated (pages): {}, allocated (bytes): {}, new-used: {}, size: {}",
        allocated_pages(ctx),
        allocated,
        new_used,
        size
    ));

    if allocated < new_used {
        let delta = new_used - allocated;

        grow_memory(ctx, delta as u32);
    }

    set_used_memory(ctx, new_used);

    new_used as u32
}

fn allocated_memory(ctx: &Context) -> u64 {
    ctx.borrow().allocated_memory()
}

fn used_memory(ctx: &Context) -> u64 {
    ctx.borrow().used_memory()
}

fn grow_memory(ctx: &Context, bytes: u32) {
    let bytes = Bytes(bytes as usize);
    let mut pages = Pages::try_from(bytes).unwrap();

    if pages.0 * (WASM_PAGE_SIZE as u32) < bytes.0 as u32 {
        pages = pages + Pages(1);
    }

    let ctx_ref = ctx.borrow();
    let memory = ctx_ref.get_memory();

    memory.grow(pages).unwrap();
}

fn set_used_memory(ctx: &Context, used: u64) {
    ctx.borrow_mut().set_used_memory(used);
}

fn allocated_pages(ctx: &Context) -> u32 {
    ctx.borrow().get_memory().size().0
}
