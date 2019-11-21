/// Builds an instance of `PageSliceLayout`
#[macro_export]
macro_rules! svm_page_slice_layout {
    ($page_idx: expr, $offset: expr, $len: expr) => {{
        use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};

        PageSliceLayout::new(PageIndex($page_idx), PageOffset($offset), $len)
    }};
}

/// Calls `read_page_slice` on the given `PageSliceCache`
#[macro_export]
macro_rules! svm_read_page_slice {
    ($storage: expr, $page_idx: expr, $offset: expr, $len: expr) => {{
        let layout = $crate::svm_page_slice_layout!($page_idx, $offset, $len);
        let slice = $storage.read_page_slice(&layout);

        if slice.is_some() {
            slice.unwrap()
        } else {
            Vec::new()
        }
    }};
}

/// Calls `write_page_slice` on the given `PageSliceCache`
#[macro_export]
macro_rules! svm_write_page_slice {
    ($storage: expr, $page_idx: expr, $offset: expr, $len: expr, $data: expr) => {{
        let layout = $crate::svm_page_slice_layout!($page_idx, $offset, $len);

        $storage.write_page_slice(&layout, $data);
    }};
}

/// Casts the `wasmer` instance context data field (of type `*mut c_void`) into `&mut PageSliceCache<PC>`.
#[macro_export]
macro_rules! wasmer_data_storage {
    ($data: expr, $PC: path) => {{
        use $crate::ctx::SvmCtx;

        let ctx: &mut SvmCtx<$PC> = $crate::cast_wasmer_data_to_svm_ctx!($data, $PC);
        &mut ctx.storage
    }};
}
