/// When called, injects the code of the `svm wasmer vmcalls`.
/// The `vmacalls` are functions imported into each running `svm wasmer` instance.
#[macro_export]
macro_rules! include_wasmer_svm_vmcalls {
    ($PC: ident) => {
        /// Copies the contents of `wasmer` memory cells under addresses:
        /// `src_mem_ptr, src_mem_ptr + 1, .. , src_mem_ptr + len (exclusive)`
        /// into `wasmer` register indexed `dst_reg`
        ///
        /// * `ctx`         - `wasmer` context (holds a `data` field. we use is `SvmCtx`)
        /// * `src_mem_ptr` - Pointer for the first memory address we want to start copying from
        /// * `len`         - The length of the memory slice we want to copy (in bytes)
        /// * `dst_reg`     - The destination register we want to load the memory slice into
        pub fn mem_to_reg_copy(
            ctx: &mut wasmer_runtime::Ctx,
            src_mem_ptr: i32,
            len: i32,
            dst_reg: i32,
        ) {
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
        pub fn reg_to_mem_copy(
            ctx: &mut wasmer_runtime::Ctx,
            src_reg: i32,
            len: i32,
            dst_mem_ptr: i32,
        ) {
            let reg = wasmer_data_reg!(ctx.data, src_reg);
            let cells = wasmer_ctx_mem_cells!(ctx, dst_mem_ptr, len);
            reg.copy_to_wasmer_mem(cells);
        }

        /// Loads from the `svm-wasmer` instance's storage a page-slice into the register indexed `dest_reg`
        ///
        /// * `ctx`      - `wasmer` context (holds a `data` field. we use is `SvmCtx`)
        /// * `src_page` - Page index
        /// * `src_slic` - Page slice index
        /// * `offset`   - Slice starting offset (within the given page)
        /// * `len`      - The length of the slice in bytes
        /// * `dst_reg`  - The destination register we want to load the page-slice into
        pub fn storage_read_to_reg(
            ctx: &mut wasmer_runtime::Ctx,
            src_page: i32,
            src_slice: i32,
            offset: i32,
            len: i32,
            dst_reg: i32,
        ) {
            let reg = wasmer_data_reg!(ctx.data, dst_reg);

            let storage = wasmer_data_storage!(ctx.data, $PC);

            dbg!(storage);

            // let slice = svm_read_page_slice!(
            //     storage,
            //     src_page as u32,
            //     src_slice as u32,
            //     offset as u32,
            //     len as u32
            // );
            //
            // dbg!(slice);
        }
    };
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::ffi::c_void;

    use crate::ctx::SvmCtx;

    use svm_common::Address;
    use svm_storage::{MemKVStore, MemPages, PageCacheImpl, PageSliceCache};

    use wasmer_runtime::{compile, error, func, imports, Func, Instance, Module};

    pub type MemPageCache<'pc, K = [u8; 32]> = PageCacheImpl<'pc, MemPages<K>>;

    // injecting the `svm vmcalls` implemented with `MemPageCache` as the `PageCache` type
    include_wasmer_svm_vmcalls!(MemPageCache);

    fn wasmer_compile_module(wasm: &str) -> error::CompileResult<Module> {
        let wasm = wabt::wat2wasm(&wasm).unwrap();

        compile(&wasm)
    }

    fn init_wasmer_instance_mem(instance: &mut Instance, start: usize, data: &[u8]) {
        let mem = instance.context_mut().memory(0);
        let cells: &[Cell<u8>] = &mem.view()[start..start + data.len()];

        for (i, byte) in data.iter().enumerate() {
            cells[start + i].set(*byte);
        }
    }

    fn init_wasmer_instance_reg(instance: &mut Instance, reg_idx: i32, data: &[u8]) {
        let ctx = instance.context_mut();
        let reg = wasmer_ctx_reg!(ctx, reg_idx);
        reg.set(data);
    }

    const WASM_MEM_TO_REG_COPY: &'static str = r#"
        (module
            ;; import `svm` vmcalls
            (func $svm_mem_to_reg_copy (import "svm" "mem_to_reg_copy") (param i32 i32 i32))

            (memory 1)  ;; memory `0` (default) is initialized with a `1 page`

            ;; exported function to be called
            (func (export "do_copy_to_reg") (param i32 i32 i32)
              get_local 0 ;; $src_mem_ptr
              get_local 1 ;; len
              get_local 2 ;; dst_reg
              call $svm_mem_to_reg_copy))"#;

    const WASM_REG_TO_MEM_COPY: &'static str = r#"
        (module
            ;; import `svm` vmcalls
            (func $svm_reg_to_mem_copy (import "svm" "reg_to_mem_copy") (param i32 i32 i32))

            (memory 1)  ;; memory `0` (default) is initialized with a `1 page`

            ;; exported function to be called
            (func (export "do_copy_to_mem") (param i32 i32 i32)
              get_local 0 ;; src_reg
              get_local 1 ;; len
              get_local 2 ;; dst_mem_ptr
              call $svm_reg_to_mem_copy))"#;

    const WASM_STORAGE_TO_REG_COPY: &'static str = r#"
        (module
            ;; import `svm` vmcalls
            (func $storage_read_to_reg (import "svm" "storage_read_to_reg") (param i32 i32 i32 i32 i32))

            (memory 1)  ;; memory `0` (default) is initialized with a `1 page`

            ;; exported function to be called
            (func (export "do_copy_to_reg") (param i32 i32 i32 i32 i32)
              get_local 0
              get_local 1
              get_local 2
              get_local 3
              get_local 4
              call $storage_read_to_reg))"#;

    #[test]
    fn vmcalls_mem_to_reg_copy() {
        let module = wasmer_compile_module(WASM_MEM_TO_REG_COPY).unwrap();

        let import_object = imports! {
            lazy_create_svm_import_object!(0x12_34_56_78, MemKVStore, MemPages, MemPageCache, 5, 100),

            "svm" => {
                "mem_to_reg_copy" => func!(mem_to_reg_copy),
            },
        };

        let mut instance = module.instantiate(&import_object).unwrap();

        // initializing memory cells `0..3` with values `10, 20, 30` respectively
        init_wasmer_instance_mem(&mut instance, 0, &[10, 20, 30]);

        // asserting register content is empty prior copy
        let reg = wasmer_ctx_reg!(instance.context(), 2);
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 0], reg.get());

        let do_copy: Func<(i32, i32, i32)> = instance.func("do_copy_to_reg").unwrap();
        assert!(do_copy.call(0, 3, 2).is_ok());

        // asserting regiter content is `10, 20, 30, 0, ... 0`
        let reg = wasmer_ctx_reg!(instance.context(), 2);
        assert_eq!([10, 20, 30, 0, 0, 0, 0, 0], reg.get());
    }

    #[test]
    fn vmcalls_reg_to_mem_copy() {
        let module = wasmer_compile_module(WASM_REG_TO_MEM_COPY).unwrap();

        let import_object = imports! {
            lazy_create_svm_import_object!(0x12_34_56_78, MemKVStore, MemPages, MemPageCache, 5, 100),

            "svm" => {
                "reg_to_mem_copy" => func!(reg_to_mem_copy),
            },
        };

        let mut instance = module.instantiate(&import_object).unwrap();

        // initializing reg `2` with values `10, 20, 30` respectively
        init_wasmer_instance_reg(&mut instance, 2, &[10, 20, 30]);

        // asserting memory is zeros before copy
        let cells = wasmer_ctx_mem_cells!(instance.context(), 0, 3);
        assert_eq!([Cell::new(0), Cell::new(0), Cell::new(0)], cells);

        // copying reg `2` content into memory cells `0..3`
        let do_copy: Func<(i32, i32, i32)> = instance.func("do_copy_to_mem").unwrap();
        assert!(do_copy.call(2, 3, 0).is_ok());

        // asserting memory cells `0..3` have the values `10, 20, 30` respectively
        let cells = wasmer_ctx_mem_cells!(instance.context(), 0, 3);
        assert_eq!([Cell::new(10), Cell::new(20), Cell::new(30)], cells);
    }

    #[test]
    fn vmcalls_storage_read_to_reg() {
        let module = wasmer_compile_module(WASM_STORAGE_TO_REG_COPY).unwrap();

        let import_object = imports! {
            lazy_create_svm_import_object!(0x12_34_56_78, MemKVStore, MemPages, MemPageCache, 5, 100),

            "svm" => {
                "storage_read_to_reg" => func!(storage_read_to_reg),
            },
        };

        let instance = module.instantiate(&import_object).unwrap();

        let do_copy: Func<(i32, i32, i32, i32, i32)> = instance.func("do_copy_to_reg").unwrap();

        let src_page = 1;
        let src_slice = 10;
        let offset = 100;
        let len = 3;
        let dest_reg = 2;

        let _ = do_copy.call(src_page, src_slice, offset, len, dest_reg);
    }
}
