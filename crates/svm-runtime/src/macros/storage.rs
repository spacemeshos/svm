use crate::ctx::SvmCtx;
use std::ffi::c_void;
use svm_storage::ContractStorage;

#[macro_export]
macro_rules! svm_page_slice_layout {
    ($page_idx: expr, $offset: expr, $len: expr) => {{
        use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};

        PageSliceLayout::new(PageIndex($page_idx), PageOffset($offset), $len)
    }};
}

/// Calls `read_page_slice` on the given `ContractStorage`
#[macro_export]
macro_rules! svm_read_page_slice {
    ($storage: expr, $page_idx: expr, $offset: expr, $len: expr) => {{
        use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};

        let layout = PageSliceLayout::new(PageIndex($page_idx), PageOffset($offset), $len);
        $storage.read_page_slice(&layout)
    }};
}

/// Calls `write_page_slice` on the given `ContractStorage`
#[macro_export]
macro_rules! svm_write_page_slice {
    ($storage: expr, $page_idx: expr, $offset: expr, $len: expr, $data: expr) => {{
        use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};
        let layout = PageSliceLayout::new(PageIndex($page_idx), PageOffset($offset), $len);

        $storage.write_page_slice(&layout, $data);
    }};
}

/// Casts the `wasmer` instance context data field (of type `*mut c_void`) into `&mut ContractStorage`.
pub fn wasmer_data_storage<'a>(data: *const c_void) -> &'a mut ContractStorage {
    let ctx: &mut SvmCtx = crate::macros::cast_wasmer_data_to_svm_ctx(data);
    &mut ctx.storage
}
