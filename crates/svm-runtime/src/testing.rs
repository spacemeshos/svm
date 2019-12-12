use std::ffi::c_void;

use crate::ctx::SvmCtx;
use svm_storage::helpers

pub fn wasmer_dummy_import_object_data(ctx: &SvmCtx) -> (*mut c_void, fn(*mut c_void)) {
    let data: *mut c_void = ctx.clone() as *const _ as *mut c_void;
    let dtor: fn(*mut c_void) = |_| {};

    (data, dtor)
}

pub fn create_memory_svm_ctx(addr: u32, state: u32) -> SvmCtx {
}

macro_rules! test_create_svm_ctx {
    () => {
        test_create_svm_ctx!(std::ptr::null())
    };
    ($node_data: expr) => {{
        use crate::ctx_data_wrapper::SvmCtxDataWrapper;
        use svm_common::{Address, State};
        use svm_storage::memory::{MemContractPageCache, MemContractPages};

        use std::cell::RefCell;
        use std::rc::Rc;

        let max_pages: u32 = 5;

        let pages_storage_gen = || {
            let addr = Address::from(0x12_34_56_78);
            let state = State::from(0x_00_00_00_00);
            let kv = Rc::new(RefCell::new(MemKVStore::new()));

            MemContractPages::new(addr, kv, state, max_pages)
        };

        let page_cache_ctor =
            |arg_pages, arg_max_pages| MemContractPageCache::new(arg_pages, arg_max_pages);

        let opts = crate::opts::Opts {
            max_pages: max_pages as usize,
        };

        create_svm_ctx!(
            SvmCtxDataWrapper::new($node_data),
            pages_storage_gen,
            page_cache_ctor,
            opts
        )
    }};
}
