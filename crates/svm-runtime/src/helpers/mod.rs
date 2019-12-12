#[macro_use]
mod register;

#[macro_use]
mod storage;

#[macro_use]
mod ctx;

#[macro_use]
mod import_object;

pub use import_object::{cast_wasmer_data_to_svm_ctx, create_svm_ctx};
pub use register::{wasmer_ctx_reg, wasmer_data_reg};
pub use storage::wasmer_data_storage;

#[cfg(test)]
mod tests {
    use crate::ctx::SvmCtx;

    use std::cell::Cell;
    use std::ffi::{c_void, CString};
    use std::os::raw::c_char;

    use svm_kv::memory::MemKVStore;

    use svm_storage::{memory::MemContractPageCache, traits::PageCache};

    #[test]
    fn node_data() {
        let s = String::from("Hello World");
        let s_ptr: *const c_char = CString::new(s).unwrap().into_raw();
        let node_data: *const c_void = s_ptr as *const c_void;

        let ctx = test_create_svm_ctx!(node_data);
        let (data, _dtor) = wasmer_fake_import_object_data(&ctx);
        let raw_chars: *mut c_char = wasmer_data_node_data!(data) as _;
        let raw_string = unsafe { CString::from_raw(raw_chars) };
        let actual = raw_string.into_string().unwrap();

        assert_eq!("Hello World", actual);
    }

    #[test]
    fn reg_copy_from_wasmer_mem() {
        let ctx = test_create_svm_ctx!();
        let (data, _dtor) = wasmer_fake_import_object_data(&ctx);

        let reg0 = wasmer_data_reg!(data, 64, 0);
        let reg1 = wasmer_data_reg!(data, 64, 1);

        // registers `0` and `1` are initialized with zeros
        assert_eq!(vec![0; 8], reg0.view());
        assert_eq!(vec![0; 8], reg1.view());

        // editing register `0` should not edit register `1`
        let cells = vec![
            Cell::new(10),
            Cell::new(20),
            Cell::new(30),
            Cell::new(40),
            Cell::new(50),
            Cell::new(60),
            Cell::new(70),
            Cell::new(80),
        ];

        reg0.copy_from_wasmer_mem(&cells);

        assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], reg0.view());
        assert_eq!(vec![00, 00, 00, 00, 00, 00, 00, 00], reg1.view());
    }

    #[test]
    fn reg_copy_to_wasmer_mem() {
        let ctx = test_create_svm_ctx!();
        let (data, _dtor) = wasmer_fake_import_object_data(&ctx);

        let reg0 = wasmer_data_reg!(data, 64, 0);
        reg0.set(&[10, 20, 30, 40, 50, 60, 70, 80]);

        let cells: Vec<Cell<u8>> = (0..8).map(|_| Cell::<u8>::new(255)).collect();

        // copying register `0` content into a fake wasmer memory
        reg0.copy_to_wasmer_mem(&cells);

        // asserting that the fake wasmer memory has the right changes
        assert_eq!(
            vec![10, 20, 30, 40, 50, 60, 70, 80],
            cells.iter().map(|c| c.get()).collect::<Vec<u8>>()
        );
    }

    #[test]
    fn wasmer_storage_read_write() {
        let ctx = test_create_svm_ctx!();
        let (data, _dtor) = wasmer_fake_import_object_data(&ctx);
        let storage = wasmer_data_storage!(data);
        let layout = svm_page_slice_layout!(1, 0, 3);

        assert_eq!(vec![0, 0, 0], storage.read_page_slice(&layout));
        storage.write_page_slice(&layout, &vec![10, 20, 30]);
        assert_eq!(vec![10, 20, 30], storage.read_page_slice(&layout));
    }

    #[test]
    fn wasmer_storage_read_to_reg() {
        let ctx = test_create_svm_ctx!();
        let (data, _dtor) = wasmer_fake_import_object_data(&ctx);

        let layout = svm_page_slice_layout!(1, 0, 3);
        let reg0 = wasmer_data_reg!(data, 64, 0);
        let storage = wasmer_data_storage!(data);

        storage.write_page_slice(&layout, &vec![10, 20, 30]);

        // reading from page `1`, slice `0`, 3 bytes starting from offset `100`
        let slice = svm_read_page_slice!(storage, 1, 0, 3);

        reg0.set(&slice);
        assert_eq!(vec![10, 20, 30, 0, 0, 0, 0, 0], reg0.view());
    }

    #[test]
    fn wasmer_storage_set_from_reg() {
        let ctx = test_create_svm_ctx!();
        let (data, _dtor) = wasmer_fake_import_object_data(ctx);

        let storage = wasmer_data_storage!(data);

        // writing `[10, 20, 30, 0, 0, 0, 0, 0]` to register `0`
        let reg0 = wasmer_data_reg!(data, 64, 0);
        reg0.set(&vec![10, 20, 30]);

        let slice = svm_read_page_slice!(storage, 1, 0, 3);
        assert_eq!(vec![0, 0, 0], slice);

        // writing at page `1`, starting from offset `0` the contents of register `0`
        svm_write_page_slice!(storage, 1, 0, 3, &reg0.view());

        let slice = svm_read_page_slice!(storage, 1, 0, 3);
        assert_eq!(vec![10, 20, 30], slice);
    }
}
