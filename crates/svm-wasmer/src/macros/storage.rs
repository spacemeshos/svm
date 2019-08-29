/// Builds an instance of `PageSliceLayout`
#[macro_export]
macro_rules! svm_page_slice_layout {
    ($page_idx: expr, $slice_idx: expr, $offset: expr, $len: expr) => {{
        use svm_storage::page::{PageIndex, PageSliceLayout, SliceIndex};

        PageSliceLayout {
            page_idx: PageIndex($page_idx),
            slice_idx: SliceIndex($slice_idx),
            offset: $offset,
            len: $len,
        }
    }};
}

/// Calls `read_page_slice` on the given `PageSliceCache`
#[macro_export]
macro_rules! svm_read_page_slice {
    ($storage: expr, $page_idx: expr, $slice_idx: expr, $offset: expr, $len: expr) => {{
        let layout = svm_page_slice_layout!($page_idx, $slice_idx, $offset, $len);
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
    ($storage: expr, $page_idx: expr, $slice_idx: expr, $offset: expr, $len: expr, $data: expr) => {{
        let layout = svm_page_slice_layout!($page_idx, $slice_idx, $offset, $len);

        $storage.write_page_slice(&layout, $data);
    }};
}

/// Casts the `wasmer` instance context data field (of type `*mut c_void`) into `&mut PageSliceCache<PC>`.
#[macro_export]
macro_rules! wasmer_data_storage {
    ($data: expr, $PC: path) => {{
        use $crate::ctx::SvmCtx;

        let ctx: &mut SvmCtx<$PC> = cast_wasmer_data_to_svm_ctx!($data, $PC);
        &mut ctx.storage
    }};
}
