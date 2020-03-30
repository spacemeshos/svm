mod buffer;
mod data_wrapper;
mod gas;
mod host_ctx;
mod register;
mod storage;

pub use buffer::{
    buffer_copy_to_reg, buffer_copy_to_storage, buffer_create, buffer_freeze, buffer_kill,
    wasmer_data_buffer,
};
pub use data_wrapper::DataWrapper;
pub use gas::use_gas;
pub use host_ctx::wasmer_data_host_ctx;
pub use register::wasmer_data_reg;
pub use storage::wasmer_data_app_storage;

use svm_storage::{
    page::{PageIndex, PageOffset, PageSliceLayout},
    AppStorage,
};

/// Reads App's storage page `page` bytes: `offset, offset + 1, ... offset + len - 1` and returns them as a `Vec<u8>`
pub fn storage_read_page_slice(
    storage: &mut AppStorage,
    page: u32,
    offset: u32,
    len: u32,
) -> Vec<u8> {
    let layout = page_slice_layout(page, offset, len);
    storage.read_page_slice(&layout)
}

/// Writes slice `data` into App's storage at page `page` under bytes: `offset, offest + 1, ..., offset + len - 1`
pub fn storage_write_page_slice(
    storage: &mut AppStorage,
    page: u32,
    offset: u32,
    len: u32,
    data: &[u8],
) {
    let layout = page_slice_layout(page, offset, len);
    storage.write_page_slice(&layout, data);
}

/// Helpers method for creating a `PageSliceLayout`.
pub fn page_slice_layout(page_idx: u32, page_offset: u32, len: u32) -> PageSliceLayout {
    assert!(page_idx <= u16::max_value() as u32);
    assert!(len > 0);

    PageSliceLayout::new(
        PageIndex(page_idx as u16),
        PageOffset(page_offset),
        len as u32,
    )
}
