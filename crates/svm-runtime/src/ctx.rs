use crate::*;

use crate::register::{SvmReg, SvmReg160, SvmReg32, SvmReg512, SvmReg64};
use std::ffi::c_void;

use svm_storage::traits::PageCache;
use svm_storage::PageSliceCache;

use log::debug;

/// The number of allocated `SvmReg32` registers for each `SvmCtx`
pub const REGS_32_COUNT: usize = 16;

/// The number of allocated `SvmReg64` registers for each `SvmCtx`
pub const REGS_64_COUNT: usize = 16;

/// The number of allocated `SvmReg160` registers for each `SvmCtx`
pub const REGS_160_COUNT: usize = 8;

/// The number of allocated `SvmReg256` registers for each `SvmCtx`
pub const REGS_256_COUNT: usize = 4;

/// The number of allocated `SvmReg512` registers for each `SvmCtx`
pub const REGS_512_COUNT: usize = 4;

/// `SvmCtx` is a container for the accessible data by `wasmer` instances
/// * `node_data` - A pointer to the *node* data
/// * `regs_32`   - A static array (`REGS_32_COUNT` elements)  of `SvmReg32`
/// * `regs_64`   - A static array (`REGS_64_COUNT` elements)  of `SvmReg64`
/// * `regs_160`  - A static array (`REGS_160_COUNT` elements) of `SvmReg160`
/// * `regs_256`  - A static array (`REGS_256_COUNT` elements) of `SvmReg256`
/// * `regs_512`  - A static array (`REGS_512_COUNT` elements) of `SvmReg512`
/// * `storage`   - An instance of `PageSliceCache`
#[repr(C)]
pub struct SvmCtx<PC: PageCache> {
    /// A pointer to the `node` data. For example the pointer will point a struct having an access
    /// to the Global State of each account, In order to query an account for its balance.
    pub node_data: *const c_void,

    /// An array that holds the `SvmReg32` registers
    pub regs_32: [SvmReg; REGS_32_COUNT],

    /// An array that holds the `SvmReg64` registers
    pub regs_64: [SvmReg; REGS_64_COUNT],

    /// An array that holds the `SvmReg160` registers
    pub regs_160: [SvmReg; REGS_160_COUNT],

    /// An array that holds the `SvmReg256` registers
    pub regs_256: [SvmReg; REGS_256_COUNT],

    /// An array that holds the `SvmReg512` registers
    pub regs_512: [SvmReg; REGS_512_COUNT],

    /// An accessor to the contract's storage (of type `PageSliceCache`)
    pub storage: PageSliceCache<PC>,
}

impl<PC> SvmCtx<PC>
where
    PC: PageCache,
{
    /// Initializes a new empty `SvmCtx`
    ///
    /// * `storage` - a mutably borrowed `PageSliceCache`
    pub fn new(node_data: *const c_void, storage: PageSliceCache<PC>) -> Self {
        let regs_32 = alloc_regs!(32, REGS_32_COUNT);
        let regs_64 = alloc_regs!(64, REGS_64_COUNT);
        let regs_160 = alloc_regs!(160, REGS_160_COUNT);
        let regs_256 = alloc_regs!(256, REGS_256_COUNT);
        let regs_512 = alloc_regs!(512, REGS_512_COUNT);

        Self {
            node_data,
            regs_32,
            regs_64,
            regs_160,
            regs_256,
            regs_512,
            storage,
        }
    }
}

impl<PC> Drop for SvmCtx<PC>
where
    PC: PageCache,
{
    fn drop(&mut self) {
        debug!("Dropping `SvmCtx`...");
    }
}
