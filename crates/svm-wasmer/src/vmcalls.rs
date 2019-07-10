use super::macros::*;
use svm_storage::traits::PagesStorage;

use svm_storage::{MemPages, PageCache};

use wasmer_runtime_core::vm::Ctx;

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
            let reg = wasmer_ctx_data_reg!(ctx.data, dst_reg);
            let cells = wasmer_ctx_mem_cells!(ctx, src_mem_ptr, len);

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
            let reg = wasmer_ctx_data_reg!(ctx.data, src_reg);
            let cells = wasmer_ctx_mem_cells!(ctx, dst_mem_ptr, len);

            reg.copy_to_wasmer_mem(cells);
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
            let reg = wasmer_ctx_data_reg!(ctx.data, dst_reg);

            let storage = wasmer_data_storage!(ctx.data, $PS);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use svm_storage::{MemPages, PageCache};

    pub type MemPageCache<'pc, K = [u8; 32]> = PageCache<'pc, MemPages<K>>;

    include_wasmer_svm_vmcalls!(MemPageCache);
}
