use super::macros::*;
use svm_storage::traits::PagesStorage;

use svm_storage::{MemPages, PageCache};

use wasmer_runtime::Ctx;

#[macro_export]
macro_rules! include_wasmer_svm_vmcalls {
    ($PS: ident) => {
        /// Copies the contents of `wasmer` memory cells under addresses:
        /// `src_mem_ptr, src_mem_ptr + 1, .. , src_mem_ptr + len (exclusive)`
        /// into `wasmer` register indexed `dst_reg`
        ///
        /// * `ctx`         - `wasmer` context (holds a `data` field. we use is `SvmCtx`)
        /// * `src_mem_ptr` - Pointer for the first memory address we want to start copying from
        /// * `len`         - The length of the memory slice we want to copy (in bytes)
        /// * `dst_reg`     - The destination register we want to load the memory slice into
        pub fn mem_to_reg_copy(ctx: &mut Ctx, src_mem_ptr: i32, len: i32, dst_reg: i32) {
            let cells = wasmer_ctx_mem_cells!(ctx, src_mem_ptr, len);
            let reg = wasmer_data_reg!(ctx.data, dst_reg);

            reg.copy_from_wasmer_mem(cells);
        }

        /// Copies the content of `wasmer` register indexed `src_reg` into `wasmer` memory cells under addresses:
        /// `dst_mem_ptr, dst_mem_ptr + 1, .. , dst_mem_ptr + len (exclusive)`
        ///
        /// * `ctx`         - `wasmer` context (holds a `data` field. we use is `SvmCtx`)
        /// * `src_reg`     - The source register we want to load its content from
        /// * `len`         - The length of the register content we want to copy into memory (in bytes)
        ///                   This parameter *must* not be greater than the register capacity
        /// * `dst_mem_ptr` - Pointer to the first memory address we want to start copying content to
        pub fn reg_to_mem_copy(ctx: &mut Ctx, src_reg: i32, len: i32, dst_mem_ptr: i32) {
            let reg = wasmer_data_reg!(ctx.data, src_reg);

            // let cells = wasmer_ctx_mem_cells!(ctx, dst_mem_ptr, len);
            //
            // reg.copy_to_wasmer_mem(cells);
        }

        /// Loads from the `svm-wasmer` instance's storage a page-slice into the register
        /// indexed `dest_reg`
        ///
        /// * `ctx`      - `wasmer` context (holds a `data` field. we use is `SvmCtx`)
        /// * `src_page` - Page index
        /// * `src_slic` - Page slice index
        /// * `offset`   - Slice starting offset (within the given page)
        /// * `len`      - The length of the slice in bytes
        /// * `dst_reg`  - The destination register we want to load the page-slice into
        pub fn storage_read_to_reg(
            ctx: &mut Ctx,
            src_page: i32,
            src_slice: i32,
            offset: i32,
            len: i32,
            dst_reg: i32,
        ) {
            let reg = wasmer_data_reg!(ctx.data, dst_reg);

            let storage = wasmer_data_storage!(ctx.data, $PS);
        }
    };
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::ffi::c_void;
    use std::rc::Rc;

    use super::*;
    use crate::register::WasmerReg64;
    use svm_common::Address;
    use svm_storage::{MemKVStore, MemPages, PageCache, PageSliceCache};

    #[macro_use]
    use wasmer_runtime::{Instance, func, Func, compile, imports};
    use wasmer_runtime::wasm::Type;

    pub type MemPageCache<'pc, K = [u8; 32]> = PageCache<'pc, MemPages<K>>;

    /// injecting the `svm vmcalls` implemented with `MemPageCache` as the `PageCache` type
    include_wasmer_svm_vmcalls!(MemPageCache);

    #[test]
    fn vmcalls_mem_to_reg_copy() {
        let wasm = r#"
            (module
                ;; import `svm` vmcalls
                (func $svm_mem_to_reg_copy (import "svm" "mem_to_reg_copy") (param i32 i32 i32))

                (memory 1)  ;; memory `0` (default) is initialized with a `1 page`

                ;; exported function to be called
                (func (export "do_copy_to_reg") (param $src_mem_ptr i32) (param $len i32) (param $dst_reg i32)
                  get_local $src_mem_ptr
                  get_local $len
                  get_local $dst_reg
                  call $svm_mem_to_reg_copy))"#;

        let wasm = wabt::wat2wasm(&wasm).unwrap();

        let module = compile(&wasm).unwrap();

        let import_object = imports! {
            || {
                 let addr = Address::from(0x12_34_56_78 as u32);
                 let db = Rc::new(RefCell::new(MemKVStore::new()));
                 let mut inner = MemPages::new(addr, db);
                 let mut page_cache = MemPageCache::new(&mut inner, 5);
                 let mut storage = PageSliceCache::new(&mut page_cache, 100);
                 let ctx = SvmCtx::new(&mut storage);

                 let boxed = Box::new(ctx);
                 let data = Box::into_raw(boxed) as *mut c_void;
                 let dtor: fn(*mut c_void) = |_| {};

                 let data_ptr = data as *mut _;

                 (data, dtor)
            },
            "svm" => {
                "mem_to_reg_copy" => func!(mem_to_reg_copy),
            },
        };

        let mut instance = module.instantiate(&import_object).unwrap();

        /// initializing memory cells `0..3` with values `10, 20, 30` respectively
        let mem = instance.context_mut().memory(0);
        let cells: &[Cell<u8>] = &mem.view()[0..3];
        cells[0].set(10);
        cells[1].set(20);
        cells[2].set(30);

        let reg = wasmer_ctx_reg!(instance.context(), 2);
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 0], reg.get());

        let do_copy: Func<(i32, i32, i32)> = instance.func("do_copy_to_reg").unwrap();
        do_copy.call(0, 3, 2);

        let reg = wasmer_ctx_reg!(instance.context(), 2);
        assert_eq!([10, 20, 30, 0, 0, 0, 0, 0], reg.get());
    }
}
