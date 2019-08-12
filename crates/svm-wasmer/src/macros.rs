/// Creates an instance of `SvmCtx` to be injected into `wasmer` context `data` field.
/// `svm vmcalls` will access that `SvmCtx` while runninng smart contracts
#[macro_export]
macro_rules! create_svm_ctx {
    ($node_data: expr, $pages_storage_gen: expr, $PC: ident, $max_pages: expr, $max_pages_slices: expr) => {{
        use std::cell::RefCell;
        use std::rc::Rc;

        use svm_storage::PageSliceCache;
        use $crate::ctx::SvmCtx;

        // pages storage
        let pages = $pages_storage_gen();
        let boxed_pages = Box::new(pages);
        let leaked_pages: &mut _ = Box::leak(boxed_pages);

        // page cache
        let page_cache = $PC::new(leaked_pages, $max_pages);
        let boxed_page_cache = Box::new(page_cache);
        let page_cache: &mut _ = Box::leak(boxed_page_cache);

        // page-slice cache
        let storage = PageSliceCache::new(page_cache, $max_pages_slices);
        let boxed_storage = Box::new(storage);
        let storage: &mut _ = Box::leak(boxed_storage);

        let ctx = SvmCtx::new($node_data, storage);
        let boxed_ctx = Box::new(ctx);

        let ctx_ptr = Box::leak(boxed_ctx);
        let ctx = unsafe { &mut *ctx_ptr };

        ctx
    }};
}

/// Builds a `svm wasmer` import object to be used when creating a `wasmer` instance.
#[macro_export]
macro_rules! create_svm_state_gen {
    ($node_data: expr, $pages_storage_gen: expr, $PC: ident, $max_pages: expr, $max_pages_slices: expr) => {{
        use std::ffi::c_void;
        use $crate::ctx::SvmCtx;

        let ctx = create_svm_ctx!(
            $node_data,
            $pages_storage_gen,
            $PC,
            $max_pages,
            $max_pages_slices
        );

        let data = ctx as *mut _ as *mut c_void;
        let dtor: fn(*mut c_void) = |ctx_data| {
            let ctx_ptr = ctx_data as *mut SvmCtx<$PC>;
            let ctx: Box<SvmCtx<$PC>> = unsafe { Box::from_raw(ctx_ptr) };
            drop(ctx);
        };

        (data, dtor)
    }};
}

/// Returns a closure that when invoked (without args) calls `create_svm_state_gen`
#[macro_export]
macro_rules! lazy_create_svm_state_gen {
    ($node_data: expr, $pages_storage_gen: expr, $PC: ident, $max_pages: expr, $max_pages_slices: expr) => {{
        move || {
            create_svm_state_gen!(
                $node_data,
                $pages_storage_gen,
                $PC,
                $max_pages,
                $max_pages_slices
            )
        }
    }};
}

/// Casts a `wasmer` instance's `data` field (of type: `c_void`) into `SvmContext<PC>` (`PC` implements `PageCache`)
#[macro_export]
macro_rules! cast_wasmer_data_to_svm_ctx {
    ($data: expr, $PC: ident) => {{
        use $crate::ctx::SvmCtx;

        let ctx_ptr = $data as *mut SvmCtx<$PC>;
        let ctx: &mut SvmCtx<$PC> = unsafe { &mut *ctx_ptr };

        ctx
    }};
}

/// Extracts from `wasmer` instance context `data` (type: `SvmCtx`) the `node_data` field (type: `*const c_void`)
#[macro_export]
macro_rules! wasmer_data_node_data {
    ($data: expr, $PC: ident) => {{
        use $crate::ctx::SvmCtx;
        let ctx: &mut SvmCtx<$PC> = cast_wasmer_data_to_svm_ctx!($data, $PC);

        ctx.node_data
    }};
}

#[cfg(test)]
mod tests {
    use crate::ctx::SvmCtx;
    use crate::register::WasmerSvmReg64;

    use svm_common::Address;

    use svm_storage::null_storage::{NullPageCache, NullPageSliceCache, NullPagesStorage};

    use std::cell::{Cell, RefCell};
    use std::ffi::{c_void, CString};
    use std::os::raw::c_char;
    use std::rc::Rc;

    use svm_storage::{
        default::DefaultPageCache,
        memory::{MemKVStore, MemPageCache32, MemPages},
        page::{PageIndex, PageSliceLayout, SliceIndex},
        traits::{PageCache, PagesStorage},
        PageSliceCache,
    };

    pub fn wasmer_fake_import_object_data<PC: PageCache>(
        ctx: &SvmCtx<PC>,
    ) -> (*mut c_void, fn(*mut c_void)) {
        let data: *mut c_void = ctx.clone() as *const _ as *mut c_void;
        let dtor: fn(*mut c_void) = |_| {};

        (data, dtor)
    }

    macro_rules! test_create_svm_ctx {
        () => {
            test_create_svm_ctx!(std::ptr::null())
        };
        ($node_data: expr) => {{
            use std::cell::RefCell;
            use std::rc::Rc;
            use svm_common::Address;

            let pages_storage_gen = || {
                let addr = Address::from(0x12_34_56_78);
                let kv = Rc::new(RefCell::new(MemKVStore::new()));
                MemPages::new(addr, kv)
            };

            create_svm_ctx!($node_data, pages_storage_gen, MemPageCache32, 5, 100)
        }};
    }

    #[test]
    fn node_data() {
        let s = String::from("Hello World");
        let s_ptr: *const c_char = CString::new(s).unwrap().into_raw();
        let node_data: *const c_void = s_ptr as *const c_void;

        let ctx = test_create_svm_ctx!(node_data);
        let (data, _dtor) = wasmer_fake_import_object_data(&ctx);
        let raw_chars: *mut c_char = wasmer_data_node_data!(data, MemPageCache32) as _;
        let raw_string = unsafe { CString::from_raw(raw_chars) };
        let actual = raw_string.into_string().unwrap();

        assert_eq!("Hello World", actual);
    }

    #[test]
    fn reg_copy_from_wasmer_mem() {
        let ctx = test_create_svm_ctx!();
        let (data, _dtor) = wasmer_fake_import_object_data(&ctx);

        let regs = wasmer_data_regs!(data, NullPageCache);

        let reg0 = svm_regs_reg!(regs, 0);
        let reg1 = svm_regs_reg!(regs, 1);

        // registers `0` and `1` are initialized with zeros
        assert_eq!(vec![0; 8], &reg0.0[..]);
        assert_eq!(vec![0; 8], &reg1.0[..]);

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

        assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], &reg0.0[..]);
        assert_eq!(vec![00, 00, 00, 00, 00, 00, 00, 00], &reg1.0[..]);
    }

    #[test]
    fn reg_copy_to_wasmer_mem() {
        let ctx = test_create_svm_ctx!();
        let (data, _dtor) = wasmer_fake_import_object_data(&ctx);

        let regs = wasmer_data_regs!(data, NullPageCache);
        let reg0 = svm_regs_reg!(regs, 0);

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
        let storage = wasmer_data_storage!(data, MemPageCache32);
        let layout = svm_page_slice_layout!(1, 0, 100, 3);

        assert_eq!(None, storage.read_page_slice(&layout));
        storage.write_page_slice(&layout, &vec![10, 20, 30]);
        assert_eq!(vec![10, 20, 30], storage.read_page_slice(&layout).unwrap());
    }

    #[test]
    fn wasmer_storage_read_to_reg() {
        let ctx = test_create_svm_ctx!();
        let (data, _dtor) = wasmer_fake_import_object_data(&ctx);

        let layout = svm_page_slice_layout!(1, 0, 100, 3);
        let regs = wasmer_data_regs!(data, MemPageCache32);
        let reg0 = svm_regs_reg!(regs, 0);
        let storage = wasmer_data_storage!(data, MemPageCache32);

        storage.write_page_slice(&layout, &vec![10, 20, 30]);

        // reading from page `1`, slice `0`, 3 bytes starting from offset `100`
        let slice = svm_read_page_slice!(storage, 1, 0, 100, 3);

        reg0.set(&slice);
        assert_eq!([10, 20, 30, 0, 0, 0, 0, 0], reg0.view());
    }

    #[test]
    fn wasmer_storage_set_from_reg() {
        let ctx = test_create_svm_ctx!();
        let (data, _dtor) = wasmer_fake_import_object_data(ctx);

        let regs = wasmer_data_regs!(data, MemPageCache32);
        let storage = wasmer_data_storage!(data, MemPageCache32);

        // writing `[10, 20, 30, 0, 0, 0, 0, 0]` to register `0`
        let reg0 = svm_regs_reg!(regs, 0);
        reg0.set(&vec![10, 20, 30]);

        let slice = svm_read_page_slice!(storage, 1, 0, 100, 3);
        assert_eq!(Vec::<u8>::new(), slice);

        // writing at page `1`, slice `0`, starting from offset `100` the content of register `0`
        svm_write_page_slice!(storage, 1, 0, 100, 3, &reg0.view());

        let slice = svm_read_page_slice!(storage, 1, 0, 100, 3);
        assert_eq!(vec![10, 20, 30], slice);
    }
}
