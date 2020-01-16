mod buffer;
mod data_wrapper;
mod register;
mod storage;

pub use buffer::{
    buffer_copy_to_storage, buffer_create, buffer_freeze, buffer_kill, wasmer_data_buffer,
};
pub use data_wrapper::DataWrapper;
pub use register::wasmer_data_reg;
pub use storage::wasmer_data_app_storage;

use svm_storage::{
    page::{PageIndex, PageOffset, PageSliceLayout},
    AppStorage,
};

pub fn storage_read_page_slice(
    storage: &mut AppStorage,
    page: i32,
    offset: i32,
    len: i32,
) -> Vec<u8> {
    let layout = page_slice_layout(page, offset, len);
    storage.read_page_slice(&layout)
}

pub fn storage_write_page_slice(
    storage: &mut AppStorage,
    page: i32,
    offset: i32,
    len: i32,
    data: &[u8],
) {
    let layout = page_slice_layout(page, offset, len);
    storage.write_page_slice(&layout, data);
}

pub fn page_slice_layout(page: i32, offset: i32, len: i32) -> PageSliceLayout {
    assert!(page >= 0 && page <= u16::max_value() as i32);
    assert!(offset >= 0);
    assert!(len > 0);

    PageSliceLayout::new(
        PageIndex(page as u16),
        PageOffset(offset as u32),
        len as u32,
    )
}
