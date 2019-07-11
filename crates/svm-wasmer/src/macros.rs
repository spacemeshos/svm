use wasmer_runtime::Ctx;

use svm_storage::traits::PagesStorage;
use svm_storage::PageSliceCache;

#[macro_export]
macro_rules! create_boxed_svm_ctx {
    ($addr: expr, $KV: ident, $PS: ident, $PC: ident, $max_pages: expr, $max_pages_slices: expr) => {{
        use std::cell::RefCell;
        use std::rc::Rc;

        let kv = $KV::new();

        let addr = Address::from($addr as u32);
        let db = Rc::new(RefCell::new(kv));

        // pages storage
        let pages = $PS::new(addr, db);
        let boxed_pages = Box::new(pages);
        let leaked_pages: &mut _ = Box::leak(boxed_pages);

        /// page cache
        let page_cache = $PC::new(leaked_pages, $max_pages);
        let boxed_page_cache = Box::new(page_cache);
        let page_cache: &mut _ = Box::leak(boxed_page_cache);

        /// storage
        let storage = PageSliceCache::new(page_cache, $max_pages_slices);
        let boxed_storage = Box::new(storage);
        let storage: &mut _ = Box::leak(boxed_storage);

        let mut ctx = SvmCtx::new(storage);
        let boxed_ctx = Box::new(ctx);

        Box::leak(boxed_ctx)
    }};
}

#[macro_export]
macro_rules! create_svm_import_object {
    ($addr: expr, $KV: ident, $PS: ident, $PC: ident, $max_pages: expr, $max_pages_slices: expr) => {{
        let ctx = create_boxed_svm_ctx!($addr, $KV, $PS, $PC, $max_pages, $max_pages_slices);

        let data = ctx as *mut _ as *mut c_void;
        let dtor: fn(*mut c_void) = |_| {};

        let data_ptr = data as *mut _;

        (data, dtor)
    }};
}

#[macro_export]
macro_rules! lazy_create_svm_import_object {
    ($addr: expr, $KV: ident, $PS: ident, $PC: ident, $max_pages: expr, $max_pages_slices: expr) => {{
        || create_svm_import_object!($addr, $KV, $PS, $PC, $max_pages, $max_pages_slices)
    }};
}

#[macro_export]
macro_rules! svm_regs_reg {
    ($regs: expr, $reg_idx: expr) => {{
        use crate::ctx::REGS_64_COUNT;
        use crate::register::WasmerReg64;

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

#[macro_export]
macro_rules! svm_read_page_slice {
    ($storage: expr, $page_idx: expr, $slice_idx: expr, $offset: expr, $len: expr) => {{
        let layout = PageSliceLayout {
            page_idx: PageIndex($page_idx),
            slice_idx: SliceIndex($slice_idx),
            offset: $offset,
            len: $len,
        };

        let slice = $storage.read_page_slice(&layout);

        if slice.is_some() {
            slice.unwrap()
        } else {
            Vec::new()
        }
    }};
}

#[macro_export]
macro_rules! svm_write_page_slice {
    ($storage: expr, $page_idx: expr, $slice_idx: expr, $offset: expr, $len: expr, $data: expr) => {{
        let layout = PageSliceLayout {
            page_idx: PageIndex($page_idx),
            slice_idx: SliceIndex($slice_idx),
            offset: $offset,
            len: $len,
        };

        $storage.write_page_slice(&layout, $data);
    }};
}

/// Casts the `wasmer` instance context data field (of type `*mut c_void`) into `&mut [WasmerReg64; REGS_64_COUNT]`
#[macro_export]
macro_rules! wasmer_data_regs {
    ($data: expr) => {{
        use crate::ctx::REGS_64_COUNT;
        use crate::register::WasmerReg64;

        let data_ptr = $data as *mut _;

        let regs: &mut [WasmerReg64; REGS_64_COUNT] = unsafe { &mut *data_ptr };

        regs
    }};
}

/// Casts the `wasmer` instance context data field (of type `*mut c_void`) into `&mut PageSliceCache<PS>`
#[macro_export]
macro_rules! wasmer_data_storage {
    ($data: expr, $PS: ident) => {{
        use svm_storage::PageSliceCache;
        use crate::register::WasmerReg64;
        use crate::ctx::REGS_64_COUNT;

        let data_ptr: *mut u8 = $data as *mut _;

        // TODO: figure out why we need to add `std::mem::usize_of::<usize>()` to the starting offset of `SvmCtx` storage
        let storage_offset =
            std::mem::size_of::<WasmerReg64>() * REGS_64_COUNT + std::mem::size_of::<usize>();

        let storage_ptr: *mut u8 = unsafe { data_ptr.offset(storage_offset as isize) };
        let storage_ptr = storage_ptr as *mut PageSliceCache<$PS>;
        let storage: &mut PageSliceCache<$PS> = unsafe { &mut *storage_ptr };

        storage
    }};
}

/// Extracts from `wasmer` instance context data field (of type `*mut c_void`), a mutable borrow for the register indexed `reg_idx`
#[macro_export]
macro_rules! wasmer_data_reg {
    ($data: expr, $reg_idx: expr) => {{
        use crate::ctx::REGS_64_COUNT;

        assert!($reg_idx >= 0 && $reg_idx < REGS_64_COUNT as i32);

        let regs = wasmer_data_regs!($data);
        svm_regs_reg!(regs, $reg_idx)
    }};
}

/// Returns a `wasmer` memory view of cells `mem_start, mem_start + 1, .. , mem_start + len` (exclusive)
#[macro_export]
macro_rules! wasmer_ctx_mem_cells {
    ($ctx: expr, $mem_start: expr, $len: expr) => {{
        let start = $mem_start as usize;
        let end = start + $len as usize;

        /// we must state explicitly that we view each mem cell as a `u8`
        &$ctx.memory(0).view::<u8>()[start..end]
    }};
}

/// Extracts from `wasmer` instance context (type: `Ctx`) a mutable borrow for the register indexed `reg_idx`
#[macro_export]
macro_rules! wasmer_ctx_reg {
    ($ctx: expr, $reg_idx: expr) => {{
        wasmer_data_reg!($ctx.data, $reg_idx)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ctx::SvmCtx;
    use crate::register::WasmerReg64;

    use svm_common::Address;

    use svm_storage::null_storage::{NullPageCache, NullPageSliceCache, NullPagesStorage};

    use std::cell::{Cell, RefCell};
    use std::ffi::c_void;
    use std::rc::Rc;

    use svm_storage::{
        MemKVStore, MemPages, PageCache, PageIndex, PageSliceCache, PageSliceLayout, SliceIndex,
    };

    pub type MemPageCache<'pc, K = [u8; 32]> = PageCache<'pc, MemPages<K>>;

    fn wasmer_import_object_data<PS: PagesStorage>(
        ctx: &SvmCtx<PS>,
    ) -> (*mut c_void, fn(*mut c_void)) {
        let data: *mut c_void = ctx.clone() as *const _ as *mut c_void;
        let dtor: fn(*mut c_void) = |_| {};

        (data, dtor)
    }

    #[test]
    fn reg_copy_from_wasmer_mem() {
        let mut pages = NullPagesStorage::new();
        let mut page_cache = NullPageCache::new(&mut pages, 1);
        let mut storage = NullPageSliceCache::new(&mut page_cache, 10);
        let ctx = SvmCtx::new(&mut storage);

        let (data, _dtor) = wasmer_import_object_data(&ctx);

        let regs = wasmer_data_regs!(data);

        let reg0: &mut WasmerReg64 = svm_regs_reg!(regs, 0);
        let reg1: &mut WasmerReg64 = svm_regs_reg!(regs, 1);

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
        let mut pages = NullPagesStorage::new();
        let mut page_cache = NullPageCache::new(&mut pages, 1);
        let mut storage = NullPageSliceCache::new(&mut page_cache, 10);
        let ctx = SvmCtx::new(&mut storage);

        let (data, _dtor) = wasmer_import_object_data(&ctx);

        let regs = wasmer_data_regs!(data);
        let reg0: &mut WasmerReg64 = svm_regs_reg!(regs, 0);

        // initialize register `0` with data
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

        // copying register `0` to a fake wasmer memory at cells: `10, 11, ... 17` (inclusive)
        let cells: Vec<Cell<u8>> = (0..8).map(|_| Cell::<u8>::new(0)).collect();

        assert_eq!(
            vec![00, 00, 00, 00, 00, 00, 00, 00],
            cells.iter().map(|c| c.get()).collect::<Vec<u8>>()
        );

        // copying register `0` content into memory
        reg0.copy_to_wasmer_mem(&cells);

        // asserting that the fake wasmer memory has the right changes
        assert_eq!(
            vec![10, 20, 30, 40, 50, 60, 70, 80],
            cells.iter().map(|c| c.get()).collect::<Vec<u8>>()
        );
    }

    #[test]
    fn wasmer_storage_read_write() {
        let addr = Address::from(0x12_34_56_78 as u32);
        let db = Rc::new(RefCell::new(MemKVStore::new()));
        let mut inner = MemPages::new(addr, db);
        let mut page_cache = MemPageCache::new(&mut inner, 1);
        let mut storage = PageSliceCache::new(&mut page_cache, 10);
        let ctx = SvmCtx::new(&mut storage);
        let (data, _dtor) = wasmer_import_object_data(&ctx);

        // extracting `storage` out of `wasmer` instance `data` field
        let storage: &mut PageSliceCache<_> = wasmer_data_storage!(data, MemPageCache);

        let layout = PageSliceLayout {
            page_idx: PageIndex(1),
            slice_idx: SliceIndex(0),
            offset: 100,
            len: 3,
        };

        assert_eq!(Vec::<u8>::new(), storage.read_page_slice(&layout).unwrap());

        storage.write_page_slice(&layout, &vec![10, 20, 30]);

        // starting over, extracting `storage` out of `wasmer` instance `data` field
        let storage = wasmer_data_storage!(data, NullPageCache);
        assert_eq!(vec![10, 20, 30], storage.read_page_slice(&layout).unwrap());
    }

    #[test]
    fn wasmer_storage_read_to_reg() {
        let addr = Address::from(0x12_34_56_78 as u32);
        let db = Rc::new(RefCell::new(MemKVStore::new()));
        let mut inner = MemPages::new(addr, db);
        let mut page_cache = MemPageCache::new(&mut inner, 5);
        let mut storage = PageSliceCache::new(&mut page_cache, 100);
        let mut ctx = SvmCtx::new(&mut storage);

        let (data, _dtor) = wasmer_import_object_data(&ctx);

        let layout = PageSliceLayout {
            page_idx: PageIndex(1),
            slice_idx: SliceIndex(0),
            offset: 100,
            len: 3,
        };

        let regs = wasmer_data_regs!(data);
        let reg0 = svm_regs_reg!(regs, 0);
        let storage = wasmer_data_storage!(data, MemPageCache);

        storage.write_page_slice(&layout, &vec![10, 20, 30]);

        /// reading from page `1`, slice `0`, 3 bytes starting from offset `100`
        let slice = svm_read_page_slice!(storage, 1, 0, 100, 3);

        reg0.set(&slice);
        assert_eq!([10, 20, 30, 0, 0, 0, 0, 0], reg0.get());
    }

    #[test]
    fn wasmer_storage_set_from_reg() {
        let addr = Address::from(0x12_34_56_78 as u32);
        let db = Rc::new(RefCell::new(MemKVStore::new()));
        let mut inner = MemPages::new(addr, db);
        let mut page_cache = MemPageCache::new(&mut inner, 5);
        let mut storage = PageSliceCache::new(&mut page_cache, 100);
        let mut ctx = SvmCtx::new(&mut storage);

        let (data, _dtor) = wasmer_import_object_data(&ctx);

        let regs = wasmer_data_regs!(data);

        /// writing `[10, 20, 30, 0, 0, 0, 0, 0]` to register `0`
        let reg0 = svm_regs_reg!(regs, 0);
        reg0.set(&vec![10, 20, 30]);

        let slice = svm_read_page_slice!(storage, 1, 0, 100, 3);
        assert_eq!(Vec::<u8>::new(), slice);

        /// writing at page `1`, slice `0`, starting from offset `100` the content of register `0`
        svm_write_page_slice!(storage, 1, 0, 100, 3, &reg0.get());

        let slice = svm_read_page_slice!(storage, 1, 0, 100, 3);
        assert_eq!(vec![10, 20, 30], slice);
    }
}
