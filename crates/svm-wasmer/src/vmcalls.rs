/// When called, injects the code of the `svm wasmer vmcalls`.
/// The `vmacalls` are functions imported into each running `svm wasmer` instance.
#[macro_export]
macro_rules! include_wasmer_svm_vmcalls {
    ($PC: ident) => {
        /// Copies the contents of `wasmer` memory cells under addresses:
        /// `src_mem_ptr, src_mem_ptr + 1, .. , src_mem_ptr + len (exclusive)`
        /// into `wasmer` register indexed `dst_reg`
        ///
        /// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`)
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
            let reg = wasmer_data_reg!(ctx.data, dst_reg, $PC);
            reg.copy_from_wasmer_mem(cells);
        }

        /// Copies the content of `wasmer` register indexed `src_reg` into `wasmer` memory cells under addresses:
        /// `dst_mem_ptr, dst_mem_ptr + 1, .. , dst_mem_ptr + len (exclusive)`
        ///
        /// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`)
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
            let reg = wasmer_data_reg!(ctx.data, src_reg, $PC);
            let cells = wasmer_ctx_mem_cells!(ctx, dst_mem_ptr, len);
            reg.copy_to_wasmer_mem(cells);
        }

        /// Loads from the `svm-wasmer` instance's storage a page-slice into the register indexed `dest_reg`
        ///
        /// * `ctx`       - `wasmer` context (holds a `data` field. we use `SvmCtx`)
        /// * `src_page`  - Page index
        /// * `src_slice` - Page slice index
        /// * `offset`    - Slice starting offset (within the given page)
        /// * `len`       - The length of the slice in bytes
        /// * `dst_reg`   - The destination register we want to load the page-slice into
        pub fn storage_read_to_reg(
            ctx: &mut wasmer_runtime::Ctx,
            src_page: i32,
            src_slice: i32,
            offset: i32,
            len: i32,
            dst_reg: i32,
        ) {
            let reg = wasmer_data_reg!(ctx.data, dst_reg, $PC);
            let storage = wasmer_data_storage!(ctx.data, $PC);

            let slice = svm_read_page_slice!(
                storage,
                src_page as u32,
                src_slice as u32,
                offset as u32,
                len as u32
            );

            reg.set(&slice);
        }

        /// Loads from the `svm-wasmer` instance's storage a page-slice into the memory address given
        ///
        /// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`)
        /// * `src_page`    - Page index
        /// * `src_slice`   - Page slice index
        /// * `offset`      - Slice starting offset (within the given page)
        /// * `len`         - The length of the slice in bytes
        /// * `dst_mem_ptr` - The destination memory address to start copying the page-slice into
        pub fn storage_read_to_mem(
            ctx: &mut wasmer_runtime::Ctx,
            src_page: i32,
            src_slice: i32,
            offset: i32,
            len: i32,
            dst_mem_ptr: i32,
        ) {
            let storage = wasmer_data_storage!(ctx.data, $PC);

            let slice = svm_read_page_slice!(
                storage,
                src_page as u32,
                src_slice as u32,
                offset as u32,
                len as u32
            );

            wasmer_ctx_mem_cells_write!(ctx, dst_mem_ptr, slice);
        }

        /// Write into `svm-wasmer` storage a page-slice copied from memory
        ///
        /// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`)
        /// * `src_mem_ptr` - Memory address to start copying from
        /// * `len`         - #memory cells to copy
        /// * `dst_page`    - Destination page
        /// * `dst_slice`   - Destination slice
        /// * `dst_offset`  - Destination slice offset
        pub fn storage_write_from_mem(
            ctx: &mut wasmer_runtime::Ctx,
            src_mem_ptr: i32,
            len: i32,
            dst_page: i32,
            dst_slice: i32,
            dst_offset: i32,
        ) {
            let cells = wasmer_ctx_mem_cells!(ctx, src_mem_ptr, len);
            let data = cells.iter().map(|cell| cell.get()).collect::<Vec<u8>>();
            let storage = wasmer_data_storage!(ctx.data, $PC);

            let slice = svm_write_page_slice!(
                storage,
                dst_page as u32,
                dst_slice as u32,
                dst_offset as u32,
                len as u32,
                &data
            );
        }
    };
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::ffi::c_void;

    use crate::ctx::SvmCtx;

    use svm_common::Address;
    use svm_storage::{
        MemKVStore, MemPages, PageCacheImpl, PageIndex, PageSliceCache, PageSliceLayout, SliceIndex,
    };

    use wasmer_runtime::{compile, error, func, imports, Func, Instance, Module};

    pub type MemPageCache<'pc, K = [u8; 32]> = PageCacheImpl<'pc, MemPages<K>>;

    // injecting the `svm vmcalls` implemented with `MemPageCache` as the `PageCache` type
    include_wasmer_svm_vmcalls!(MemPageCache);

    fn wasmer_compile_module(wasm: &str) -> error::CompileResult<Module> {
        let wasm = wabt::wat2wasm(&wasm).unwrap();

        compile(&wasm)
    }

    fn init_wasmer_instance_reg<PC>(instance: &mut Instance, reg_idx: i32, data: &[u8]) {
        let ctx = instance.context_mut();
        let reg = wasmer_ctx_reg!(ctx, reg_idx, PC);
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
              get_local 0 ;; src_page
              get_local 1 ;; src_slice
              get_local 2 ;; offset
              get_local 3 ;; len
              get_local 4 ;; dst_reg
              call $storage_read_to_reg))"#;

    const WASM_STORAGE_TO_MEM_COPY: &'static str = r#"
        (module
            ;; import `svm` vmcalls
            (func $storage_read_to_mem (import "svm" "storage_read_to_mem") (param i32 i32 i32 i32 i32))

            (memory 1)  ;; memory `0` (default) is initialized with a `1 page`

            ;; exported function to be called
            (func (export "do_copy_to_mem") (param i32 i32 i32 i32 i32)
              get_local 0 ;; src_page
              get_local 1 ;; src_slice
              get_local 2 ;; offset
              get_local 3 ;; len
              get_local 4 ;; dst_mem_ptr
              call $storage_read_to_mem))"#;

    const WASM_STORAGE_WRITE_FROM_MEM: &'static str = r#"
        (module
            ;; import `svm` vmcalls
            (func $storage_write_from_mem (import "svm" "storage_write_from_mem") (param i32 i32 i32 i32 i32))

            (memory 1)  ;; memory `0` (default) is initialized with a `1 page`

            ;; exported function to be called
            (func (export "do_write_from_mem") (param i32 i32 i32 i32 i32)
              get_local 0 ;; src_mem_ptr
              get_local 1 ;; len
              get_local 2 ;; dst_page
              get_local 3 ;; dst_slice
              get_local 4 ;; dst_offset
              call $storage_write_from_mem))"#;

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

        // initializing memory cells `200..203` with values `10, 20, 30` respectively
        wasmer_ctx_mem_cells_write!(instance.context(), 200, &[10, 20, 30]);

        // asserting register content is empty prior copy
        let reg = wasmer_ctx_reg!(instance.context(), 2, MemPageCache);
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 0], reg.get());

        let do_copy: Func<(i32, i32, i32)> = instance.func("do_copy_to_reg").unwrap();
        assert!(do_copy.call(200, 3, 2).is_ok());

        // asserting regiter content is `10, 20, 30, 0, ... 0`
        let reg = wasmer_ctx_reg!(instance.context(), 2, MemPageCache);
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
        init_wasmer_instance_reg::<MemPageCache>(&mut instance, 2, &[10, 20, 30]);

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

        let mut instance = module.instantiate(&import_object).unwrap();
        let storage = wasmer_data_storage!(instance.context_mut().data, MemPageCache);
        let layout = svm_page_slice_layout!(1, 10, 100, 3);

        // we write `[10, 20, 30]` into storage slice `10` (page `1`, cells: `100..103`)
        storage.write_page_slice(&layout, &vec![10, 20, 30]);

        // we first initialize register `2` with some garbage data which should be overriden
        // after calling the exported `do_copy` function
        let reg = wasmer_ctx_reg!(instance.context(), 2, MemPageCache);
        reg.set(&[255; 8]);

        let do_copy: Func<(i32, i32, i32, i32, i32)> = instance.func("do_copy_to_reg").unwrap();

        // we copy storage `slice 0` (page `1`, cells: `100..103`) into register `2`
        assert!(do_copy.call(1, 10, 100, 3, 2).is_ok());

        let reg = wasmer_ctx_reg!(instance.context(), 2, MemPageCache);
        assert_eq!([10, 20, 30, 0, 0, 0, 0, 0], reg.get());
    }

    #[test]
    fn vmcalls_storage_read_to_mem() {
        let module = wasmer_compile_module(WASM_STORAGE_TO_MEM_COPY).unwrap();

        let import_object = imports! {
            lazy_create_svm_import_object!(0x12_34_56_78, MemKVStore, MemPages, MemPageCache, 5, 100),

            "svm" => {
                "storage_read_to_mem" => func!(storage_read_to_mem),
            },
        };

        let mut instance = module.instantiate(&import_object).unwrap();
        let storage = wasmer_data_storage!(instance.context_mut().data, MemPageCache);
        let layout = svm_page_slice_layout!(1, 10, 100, 3);

        // we write `[10, 20, 30]` into storage slice `10` (page `1`, cells `100..103`)
        storage.write_page_slice(&layout, &vec![10, 20, 30]);

        let do_copy: Func<(i32, i32, i32, i32, i32)> = instance.func("do_copy_to_mem").unwrap();

        // we copy storage `slice 0` (page `1`, cells: `100..103`) into memory starting from address = 200
        assert!(do_copy.call(1, 10, 100, 3, 200).is_ok());

        let cells = wasmer_ctx_mem_cells!(instance.context(), 200, 3);
        assert_eq!(&[Cell::new(10), Cell::new(20), Cell::new(30)], cells);
    }

    #[test]
    fn vmcalls_storage_write_from_mem() {
        let module = wasmer_compile_module(WASM_STORAGE_WRITE_FROM_MEM).unwrap();

        let import_object = imports! {
            lazy_create_svm_import_object!(0x12_34_56_78, MemKVStore, MemPages, MemPageCache, 5, 100),

            "svm" => {
                "storage_write_from_mem" => func!(storage_write_from_mem),
            },
        };

        let mut instance = module.instantiate(&import_object).unwrap();
        let storage = wasmer_data_storage!(instance.context_mut().data, MemPageCache);

        wasmer_ctx_mem_cells_write!(instance.context(), 200, &[10, 20, 30]);

        let layout = svm_page_slice_layout!(1, 10, 100, 3);

        assert_eq!(None, storage.read_page_slice(&layout));

        let do_write: Func<(i32, i32, i32, i32, i32)> = instance.func("do_write_from_mem").unwrap();

        // we copy memory cells `200..`203` into storage (`page 1`, `slice 10`, cells: `100..103`)
        assert!(do_write.call(200, 3, 1, 10, 100).is_ok());

        assert_eq!(Some(vec![10, 20, 30]), storage.read_page_slice(&layout));
    }
}
