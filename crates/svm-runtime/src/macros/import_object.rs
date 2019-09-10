/// Creates an instance of `SvmCtx` to be injected into `wasmer` context `data` field.
/// `svm vmcalls` will access that `SvmCtx` while runninng smart contracts
#[macro_export]
macro_rules! create_svm_ctx {
    ($node_data: expr, $pages_storage_gen: expr, $page_cache_ctor: expr, $PC: path, $opts: expr) => {{
        use svm_storage::PageSliceCache;
        use $crate::ctx::SvmCtx;

        let pages = $pages_storage_gen();
        let page_cache = $page_cache_ctor(pages, $opts.max_pages);
        let storage = PageSliceCache::new(page_cache, $opts.max_pages_slices);

        let ctx = SvmCtx::<$PC>::new($node_data, storage);
        let boxed_ctx = Box::new(ctx);

        let ctx_ptr = Box::leak(boxed_ctx);
        let ctx = &mut *ctx_ptr;

        ctx
    }};
}

/// Builds a `svm wasmer` import object to be used when creating a `wasmer` instance.
#[macro_export]
macro_rules! create_svm_state_gen {
    ($node_data: expr, $pages_storage_gen: expr, $page_cache_ctor: expr, $PC: path, $opts: expr) => {{
        use std::ffi::c_void;
        use $crate::ctx::SvmCtx;

        let ctx =
            $crate::create_svm_ctx!($node_data, $pages_storage_gen, $page_cache_ctor, $PC, $opts);

        let data = ctx as *mut _ as *mut c_void;
        let dtor: fn(*mut c_void) = |ctx_data| {
            let ctx_ptr = ctx_data as *mut SvmCtx<$PC>;

            // triggers memory releasing
            unsafe { Box::from_raw(ctx_ptr) };
        };

        (data, dtor)
    }};
}

/// Returns a closure that when invoked (without args) calls `create_svm_state_gen`
#[macro_export]
macro_rules! lazy_create_svm_state_gen {
    ($node_data: expr, $pages_storage_gen: expr, $page_cache_ctor: expr, $PC: path, $opts: expr) => {{
        move || {
            $crate::create_svm_state_gen!(
                $node_data,
                $pages_storage_gen,
                $page_cache_ctor,
                $PC,
                $opts
            )
        }
    }};
}
