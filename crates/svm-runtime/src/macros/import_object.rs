use crate::ctx::SvmCtx;
use crate::ctx_data_wrapper::SvmCtxDataWrapper;
use crate::opts::Opts;

use svm_storage::traits::PageCache;
use svm_storage::ContractPages;
use svm_storage::PageSliceCache;

use std::ffi::c_void;

fn create_svm_ctx<PC: PageCache>(
    node_data: *const c_void,
    storage_creator: impl Fn(Opts) -> PageSliceCache<PC>,
    opts: Opts,
) -> *mut SvmCtx<PC> {
    log::trace!("create_svm_ctx...");

    let storage = storage_creator(opts);
    let ctx = SvmCtx::<PC>::new(SvmCtxDataWrapper::new(node_data), storage);
    let boxed_ctx = Box::new(ctx);

    Box::leak(boxed_ctx) as _
}

fn create_svm_state<PC: PageCache>(
    node_data: *const c_void,
    storage_creator: impl Fn(Opts) -> PageSliceCache<PC>,
    opts: Opts,
) -> (*mut c_void, fn(*mut c_void)) {
    use std::ffi::c_void;

    log::trace!("create_svm_state...");

    let ctx = create_svm_ctx(node_data, storage_creator, opts);

    let node_data = ctx as *mut _ as *mut c_void;

    let dtor: fn(*mut c_void) = |ctx_data| {
        let ctx_ptr = ctx_data as *mut SvmCtx<PC>;

        // triggers memory releasing
        unsafe { Box::from_raw(ctx_ptr) };
    };

    (node_data, dtor)
}

fn lazy_create_svm_state_gen<PC: PageCache>(
    node_data: *const c_void,
    storage_creator: impl Fn(Opts) -> PageSliceCache<PC> + Copy,
    opts: Opts,
) -> impl Fn() -> (*mut c_void, fn(*mut c_void)) {
    log::trace!("lazy_create_svm_state_gen...");

    move || create_svm_state(node_data, storage_creator, opts)
}

/// Creates an instance of `SvmCtx` to be injected into `wasmer` context `node_data` field.
/// `svm vmcalls` will access that `SvmCtx` while runninng smart contracts
#[macro_export]
macro_rules! create_svm_ctx {
    ($node_data: expr, $pages_storage_gen: expr, $page_cache_ctor: expr, $PC: path, $opts: expr) => {{
        use svm_storage::PageSliceCache;
        use $crate::ctx::SvmCtx;

        log::trace!("create_svm_ctx...");

        let pages = $pages_storage_gen();
        let page_cache = $page_cache_ctor(pages, $opts.max_pages);
        let storage = PageSliceCache::new(page_cache);

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

        log::trace!("create_svm_state_gen...");

        let ctx =
            $crate::create_svm_ctx!($node_data, $pages_storage_gen, $page_cache_ctor, $PC, $opts);

        let node_data = ctx as *mut _ as *mut c_void;
        let dtor: fn(*mut c_void) = |ctx_data| {
            let ctx_ptr = ctx_data as *mut SvmCtx<$PC>;

            // triggers memory releasing
            unsafe { Box::from_raw(ctx_ptr) };
        };

        (node_data, dtor)
    }};
}

/// Returns a closure that when invoked (without args) calls `create_svm_state_gen`
#[macro_export]
macro_rules! lazy_create_svm_state_gen {
    ($node_data: expr, $pages_storage_gen: expr, $page_cache_ctor: expr, $PC: path, $opts: expr) => {{
        log::trace!("lazy_create_svm_state_gen...");

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
