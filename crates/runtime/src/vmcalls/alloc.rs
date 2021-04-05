use wasmer::Pages;

use crate::Context;

pub fn allocate(ctx: &Context, size: u32) -> u32 {
    dbg!(format!("allocate request for {} bytes", size));

    let allocated = ctx.borrow().allocated_memory();
    let used = ctx.borrow().used_memory();

    let new_used = used + size as u64;

    if allocated < new_used {
        grow_memory(ctx);
    }

    set_used_memory(ctx, new_used);

    new_used as u32
}

fn grow_memory(ctx: &Context) {
    let delta = Pages(1);

    let ctx_ref = ctx.borrow();
    let memory = ctx_ref.get_memory();

    memory.grow(delta).unwrap();
}

fn set_used_memory(ctx: &Context, used: u64) {
    ctx.borrow_mut().set_used_memory(used);
}
