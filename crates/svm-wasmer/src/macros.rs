/// Creates an instance of `SvmCtx` to be injected into `wasmer` context `data` field.
/// `svm vmcalls` will access that `SvmCtx` while runninng smart contracts
#[macro_export]
macro_rules! create_boxed_svm_ctx {
    ($node_data: expr, $addr: expr, $KV: ident, $PS: ident, $PC: ident, $max_pages: expr, $max_pages_slices: expr) => {{
        use std::cell::RefCell;
        use std::rc::Rc;

        use svm_common::Address;
        use svm_storage::PageSliceCache;
        use $crate::ctx::SvmCtx;

        let kv = $KV::new();
        let db = Rc::new(RefCell::new(kv));

        // pages storage
        let pages = $PS::new($addr, db);
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
    ($node_data: expr, $addr: expr, $KV: ident, $PS: ident, $PC: ident, $max_pages: expr, $max_pages_slices: expr) => {{
        use std::ffi::c_void;
        use $crate::ctx::SvmCtx;

        let ctx = create_boxed_svm_ctx!(
            $node_data,
            $addr,
            $KV,
            $PS,
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
    ($node_data: expr, $addr: expr, $KV: ident, $PS: ident, $PC: ident, $max_pages: expr, $max_pages_slices: expr) => {{
        move || {
            create_svm_state_gen!(
                $node_data,
                $addr,
                $KV,
                $PS,
                $PC,
                $max_pages,
                $max_pages_slices
            )
        }
    }};
}

/// Receives an array of `WasmerReg64` and returns the `reg_idx` register.
#[macro_export]
macro_rules! svm_regs_reg {
    ($regs: expr, $reg_idx: expr) => {{
        use $crate::ctx::REGS_64_COUNT;
        use $crate::register::WasmerReg64;

        assert!($reg_idx >= 0 && $reg_idx < REGS_64_COUNT as i32);

        /// We don't do:
        /// ```rust
        /// let reg: &mut WasmerReg64 = $regs.regs_64[$reg_idx as usize];
        /// ```
        ///
        /// Because we like to keep the option to  mutate a couple of registers simultaneously
        /// without the Rust borrow checker getting angry...
        /// so instead we use _Unsafe Rust_
        let regs_ptr: *mut WasmerReg64 = $regs.as_mut_ptr();

        let reg_idx_ptr: *mut WasmerReg64 = unsafe { regs_ptr.offset($reg_idx as isize) };
        let reg: &mut WasmerReg64 = unsafe { &mut *reg_idx_ptr };

        reg
    }};
}

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

/// Casts the `wasmer` instance context data field (of type `*mut c_void`) into `&mut [WasmerReg64; REGS_64_COUNT]`
#[macro_export]
macro_rules! wasmer_data_regs {
    ($data: expr, $PC: ident) => {{
        use $crate::ctx::{SvmCtx, REGS_64_COUNT};
        use $crate::register::WasmerReg64;

        let ctx: &mut SvmCtx<$PC> = cast_wasmer_data_to_svm_ctx!($data, $PC);

        &mut ctx.regs_64
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

/// Casts the `wasmer` instance context data field (of type `*mut c_void`) into `&mut PageSliceCache<PC>`
#[macro_export]
macro_rules! wasmer_data_storage {
    ($data: expr, $PC: ident) => {{
        use $crate::ctx::SvmCtx;

        let ctx: &mut SvmCtx<$PC> = cast_wasmer_data_to_svm_ctx!($data, $PC);
        &mut ctx.storage
    }};
}

/// Extracts from `wasmer` instance context data field (of type `*mut c_void`), a mutable borrow for the register indexed `reg_idx`
#[macro_export]
macro_rules! wasmer_data_reg {
    ($data: expr, $reg_idx: expr, $PC: ident) => {{
        use $crate::ctx::{SvmCtx, REGS_64_COUNT};

        assert!($reg_idx >= 0 && $reg_idx < REGS_64_COUNT as i32);

        let ctx: &mut SvmCtx<$PC> = cast_wasmer_data_to_svm_ctx!($data, $PC);

        svm_regs_reg!(ctx.regs_64, $reg_idx)
    }};
}

/// Returns a `wasmer` memory view of cells `mem_start, mem_start + 1, .. , mem_start + len` (exclusive)
#[macro_export]
macro_rules! wasmer_ctx_mem_cells {
    ($ctx: expr, $mem_idx: expr, $mem_start: expr, $len: expr) => {{
        let start = $mem_start as usize;
        let end = start + $len as usize;

        /// we must state explicitly that we view each mem cell as a `u8`
        &$ctx.memory($mem_idx as u32).view::<u8>()[start..end]
    }};
}

/// Copies input `data: &[u8]` into `wasmer` memory cells `mem_start, mem_start + 1, .. , mem_start + data.len()` (exclusive)
#[macro_export]
macro_rules! wasmer_ctx_mem_cells_write {
    ($ctx: expr, $mem_idx: expr, $mem_start: expr, $data: expr) => {{
        let cells = wasmer_ctx_mem_cells!($ctx, $mem_idx, $mem_start, $data.len());

        for (cell, byte) in cells.iter().zip($data.iter()) {
            cell.set(*byte);
        }
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

/// Extracts from `wasmer` instance context (type: `Ctx`) a mutable borrow for the register indexed `reg_idx`.
/// Will be used by storage vmcalls.
#[macro_export]
macro_rules! wasmer_ctx_reg {
    ($ctx: expr, $reg_idx: expr, $PC: ident) => {{
        wasmer_data_reg!($ctx.data, $reg_idx, $PC)
    }};
}

/// Extracts from `wasmer` instance context (type: `Ctx`) the register indexed `reg_idx` and calls
/// on it `set` with input `data`.  Will be used by storage vmcalls.
#[macro_export]
macro_rules! wasmer_ctx_reg_write {
    ($ctx: expr, $reg_idx: expr, $data: expr, $PC: ident) => {{
        let reg = wasmer_data_reg!($ctx.data, $reg_idx, $PC);
        reg.set($data);
    }};
}

#[cfg(test)]
mod tests {
    use crate::ctx::SvmCtx;
    use crate::register::WasmerReg64;

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

    #[test]
    fn node_data() {
        let s = String::from("Hello World");
        let s_ptr: *const c_char = CString::new(s).unwrap().into_raw();
        let node_data: *const c_void = s_ptr as *const c_void;

        let ctx = create_boxed_svm_ctx!(
            node_data,
            Address::from(0x12_34_56_78),
            MemKVStore,
            MemPages,
            MemPageCache32,
            5,
            100
        );

        let (data, _dtor) = wasmer_fake_import_object_data(&ctx);
        let raw_chars: *mut c_char = wasmer_data_node_data!(data, MemPageCache32) as _;
        let raw_string = unsafe { CString::from_raw(raw_chars) };
        let actual = raw_string.into_string().unwrap();

        assert_eq!("Hello World", actual);
    }

    #[test]
    fn reg_copy_from_wasmer_mem() {
        let ctx = create_boxed_svm_ctx!(
            std::ptr::null(),
            Address::from(0x12_34_56_78),
            MemKVStore,
            MemPages,
            MemPageCache32,
            5,
            100
        );

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
        let ctx = create_boxed_svm_ctx!(
            std::ptr::null(),
            Address::from(0x12_34_56_78),
            MemKVStore,
            MemPages,
            MemPageCache32,
            5,
            100
        );

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
        let ctx = create_boxed_svm_ctx!(
            std::ptr::null(),
            Address::from(0x12_34_56_78),
            MemKVStore,
            MemPages,
            MemPageCache32,
            5,
            100
        );

        let (data, _dtor) = wasmer_fake_import_object_data(&ctx);
        let storage = wasmer_data_storage!(data, MemPageCache32);
        let layout = svm_page_slice_layout!(1, 0, 100, 3);

        assert_eq!(None, storage.read_page_slice(&layout));
        storage.write_page_slice(&layout, &vec![10, 20, 30]);
        assert_eq!(vec![10, 20, 30], storage.read_page_slice(&layout).unwrap());
    }

    #[test]
    fn wasmer_storage_read_to_reg() {
        let ctx = create_boxed_svm_ctx!(
            std::ptr::null(),
            Address::from(0x12_34_56_78),
            MemKVStore,
            MemPages,
            MemPageCache32,
            5,
            100
        );
        let (data, _dtor) = wasmer_fake_import_object_data(&ctx);

        let layout = svm_page_slice_layout!(1, 0, 100, 3);
        let regs = wasmer_data_regs!(data, MemPageCache32);
        let reg0 = svm_regs_reg!(regs, 0);
        let storage = wasmer_data_storage!(data, MemPageCache32);

        storage.write_page_slice(&layout, &vec![10, 20, 30]);

        // reading from page `1`, slice `0`, 3 bytes starting from offset `100`
        let slice = svm_read_page_slice!(storage, 1, 0, 100, 3);

        reg0.set(&slice);
        assert_eq!([10, 20, 30, 0, 0, 0, 0, 0], reg0.get());
    }

    #[test]
    fn wasmer_storage_set_from_reg() {
        let ctx = create_boxed_svm_ctx!(
            std::ptr::null(),
            Address::from(0x12_34_56_78),
            MemKVStore,
            MemPages,
            MemPageCache32,
            5,
            100
        );

        let (data, _dtor) = wasmer_fake_import_object_data(ctx);

        let regs = wasmer_data_regs!(data, MemPageCache32);
        let storage = wasmer_data_storage!(data, MemPageCache32);

        // writing `[10, 20, 30, 0, 0, 0, 0, 0]` to register `0`
        let reg0 = svm_regs_reg!(regs, 0);
        reg0.set(&vec![10, 20, 30]);

        let slice = svm_read_page_slice!(storage, 1, 0, 100, 3);
        assert_eq!(Vec::<u8>::new(), slice);

        // writing at page `1`, slice `0`, starting from offset `100` the content of register `0`
        svm_write_page_slice!(storage, 1, 0, 100, 3, &reg0.get());

        let slice = svm_read_page_slice!(storage, 1, 0, 100, 3);
        assert_eq!(vec![10, 20, 30], slice);
    }
}
