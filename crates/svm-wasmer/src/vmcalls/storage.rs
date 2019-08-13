/// When called, injects the code of the `svm wasmer storage vmcalls`.
/// The `vmcalls` are functions imported into each running `svm wasmer` instance.
#[macro_export]
macro_rules! include_wasmer_svm_storage_vmcalls {
    ($PC: ident) => {
        /// Copies the contents of `wasmer` memory cells under addresses:
        /// `src_mem_ptr, src_mem_ptr + 1, .. , src_mem_ptr + len (exclusive)`
        /// into `wasmer` register indexed `dst_reg`
        ///
        /// * `ctx`          - `wasmer` context (holds a `data` field. we use `SvmCtx`)
        /// * `src_mem_idx`  - The source memory index we want to copy from
        /// * `src_mem_ptr`  - Pointer for the first memory address we want to start copying from
        /// * `len`          - The length of the memory slice we want to copy (in bytes)
        /// * `dst_reg_bits` - The type of the register (determined by its #bits) we want to copy data to
        /// * `dst_reg_idx`  - The destination register we want to load the memory slice into
        pub fn mem_to_reg_copy(
            ctx: &mut wasmer_runtime::Ctx,
            src_mem_idx: i32,
            src_mem_ptr: i32,
            len: i32,
            dst_reg_bits: i32,
            dst_reg_idx: i32,
        ) {
            let cells = wasmer_ctx_mem_cells!(ctx, src_mem_idx, src_mem_ptr, len);
            let reg = wasmer_data_reg!(ctx.data, dst_reg_bits, dst_reg_idx, $PC);
            reg.copy_from_wasmer_mem(cells);
        }

        /// Copies the content of `wasmer` register indexed `src_reg` into `wasmer` memory cells under addresses:
        /// `dst_mem_ptr, dst_mem_ptr + 1, .. , dst_mem_ptr + len (exclusive)`
        ///
        /// * `ctx`          - `wasmer` context (holds a `data` field. we use `SvmCtx`)
        /// * `src_reg_bits` - The type of the register (determined by its #bits) we want to copy data from
        /// * `src_reg_idx`  - The source register index we want to load its content from
        /// * `len`          - The length of the register content we want to copy into memory (in bytes)
        ///                    This parameter *must* not be greater than the register capacity
        /// * `dst_mem_idx`  - The index of the memory we want to copy to
        /// * `dst_mem_ptr`  - Pointer to the first memory address we want to start copying content to
        pub fn reg_to_mem_copy(
            ctx: &mut wasmer_runtime::Ctx,
            src_reg_bits: i32,
            src_reg_idx: i32,
            len: i32,
            dst_mem_idx: i32,
            dst_mem_ptr: i32,
        ) {
            let reg = wasmer_data_reg!(ctx.data, src_reg_bits, src_reg_idx, $PC);
            let cells = wasmer_ctx_mem_cells!(ctx, dst_mem_idx, dst_mem_ptr, len);
            reg.copy_to_wasmer_mem(cells);
        }

        /// Loads from the `svm-wasmer` instance's storage a page-slice into the register indexed `dest_reg`
        ///
        /// * `ctx`          - `wasmer` context (holds a `data` field. we use `SvmCtx`)
        /// * `src_page`     - Page index
        /// * `src_slice`    - Page slice index
        /// * `offset`       - Slice starting offset (within the given page)
        /// * `len`          - The length of the slice in bytes
        /// * `dst_reg_bits` - The type of the register (determined by its #bits) we want to copy data to
        /// * `dst_reg_idx`  - The destination register index we want to load the page-slice into
        pub fn storage_read_to_reg(
            ctx: &mut wasmer_runtime::Ctx,
            src_page: i32,
            src_slice: i32,
            offset: i32,
            len: i32,
            dst_reg_bits: i32,
            dst_reg_idx: i32,
        ) {
            let reg = wasmer_data_reg!(ctx.data, dst_reg_bits, dst_reg_idx, $PC);
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
        /// * `dst_mem_idx` - The destination memory index we want to copy to
        /// * `dst_mem_ptr` - The destination memory address to start copying the page-slice into
        pub fn storage_read_to_mem(
            ctx: &mut wasmer_runtime::Ctx,
            src_page: i32,
            src_slice: i32,
            offset: i32,
            len: i32,
            dst_mem_idx: i32,
            dst_mem_ptr: i32,
        ) {
            let storage = wasmer_data_storage!(ctx.data, $PC);

            let mut slice = svm_read_page_slice!(
                storage,
                src_page as u32,
                src_slice as u32,
                offset as u32,
                len as u32
            );

            if slice.len() == 0 {
                // slice is empty, i.e it doesn't really exist
                // so we fallback to zeros page-slice
                slice.resize(len as usize, 0);
            }

            wasmer_ctx_mem_cells_write!(ctx, dst_mem_idx, dst_mem_ptr, slice);
        }

        /// Writes into `svm-wasmer` storage, a page-slice copied from `wasmer` memory
        ///
        /// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`)
        /// * `src_mem_idx` - The memory index we start to copy from
        /// * `src_mem_ptr` - Memory address to start copying from
        /// * `len`         - #memory cells to copy
        /// * `dst_page`    - Destination page
        /// * `dst_slice`   - Destination slice
        /// * `dst_offset`  - Destination slice offset
        pub fn storage_write_from_mem(
            ctx: &mut wasmer_runtime::Ctx,
            src_mem_idx: i32,
            src_mem_ptr: i32,
            len: i32,
            dst_page: i32,
            dst_slice: i32,
            dst_offset: i32,
        ) {
            let cells = wasmer_ctx_mem_cells!(ctx, src_mem_idx, src_mem_ptr, len);
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

        /// Writes into `svm-wasmer` storage, a page-slice copied from `svm wasmer` register
        ///
        /// * `ctx`          - `wasmer` context (holds a `data` field. we use `SvmCtx`)
        /// * `src_reg_bits` - The type of the register (determined by its #bits) we want to copy data from
        /// * `src_reg_idx`  - Source register to start copying from
        /// * `len`          - #register bytes to copy. Must be less than register capacity
        /// * `dst_page`     - Destination page
        /// * `dst_slice`    - Destination slice
        /// * `dst_offset`   - Destination slice offset
        pub fn storage_write_from_reg(
            ctx: &mut wasmer_runtime::Ctx,
            src_reg_bits: i32,
            src_reg_idx: i32,
            len: i32,
            dst_page: i32,
            dst_slice: i32,
            dst_offset: i32,
        ) {
            let reg = wasmer_data_reg!(ctx.data, src_reg_bits, src_reg_idx, $PC);
            let storage = wasmer_data_storage!(ctx.data, $PC);
            let data = reg.getn(len as usize);

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
