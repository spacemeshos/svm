use crate::register::{WasmerSvmReg160, WasmerSvmReg256, WasmerSvmReg512, WasmerSvmReg64};
use std::ffi::c_void;
use svm_common::Address;
use svm_storage::PageSliceCache;

/// The number of allocated wasmer `WasmerSvmReg64` registers for each `SvmCtx`
pub const REGS_64_COUNT: usize = 16;

/// The number of allocated wasmer `WasmerSvmReg160` registers for each `SvmCtx`
pub const REGS_160_COUNT: usize = 8;

/// The number of allocated wasmer `WasmerSvmReg256` registers for each `SvmCtx`
pub const REGS_256_COUNT: usize = 4;

/// The number of allocated wasmer `WasmerSvmReg512` registers for each `SvmCtx`
pub const REGS_512_COUNT: usize = 4;

/// `SvmCtx` is a container for the accessible data by `wasmer` instances
/// * `node_data` - A pointer to the *node* data
/// * `regs_64`   - A static array (`REGS_64_COUNT` elements) of `WasmerSvmReg64`
/// * `regs_160`  - A static array (`REGS_160_COUNT` elements) of `WasmerSvmReg160`
/// * `regs_256`  - A static array (`REGS_256_COUNT` elements) of `WasmerSvmReg256`
/// * `regs_512`  - A static array (`REGS_512_COUNT` elements) of `WasmerSvmReg512`
/// * `storage`   - An instance of `PageSliceCache`
///
/// Explanation about `SvmCtx` lifetimes and generics:
/// * `a  - the lifetime of the mutable borrowed `PageSliceCache`
/// * `pc - the lifetime of the inner `PageCache` within `PageSliceCache` (`pc - stands for `PageCache`)
/// *  PC - a type implementing the trait `PageCache` (`PC` stands for `PageCache`)
#[repr(C)]
pub struct SvmCtx<'a, 'pc: 'a, PC> {
    /// A pointer to the `node` data. For example the pointer will point a struct having an access
    /// to the Global State of each account, In order to query an account for its balance.
    pub node_data: *const c_void,

    /// An array that holds the `WasmerSvmReg64` registers
    pub regs_64: [WasmerSvmReg64; REGS_64_COUNT],

    /// An array that holds the `WasmerSvmReg160` registers
    pub regs_160: [WasmerSvmReg160; REGS_160_COUNT],

    /// An array that holds the `WasmerSvmReg256` registers
    pub regs_256: [WasmerSvmReg256; REGS_256_COUNT],

    /// An array that holds the `WasmerSvmReg512` registers
    pub regs_512: [WasmerSvmReg512; REGS_512_COUNT],

    /// An accessor to the contract's storage (of type `PageSliceCache`)
    pub storage: &'a mut PageSliceCache<'pc, PC>,
}

impl<'a, 'pc: 'a, PC> SvmCtx<'a, 'pc, PC> {
    /// Initializes a new empty `SvmCtx`
    ///
    /// * `storage` - a mutably borrowed `PageSliceCache`
    pub fn new(node_data: *const c_void, storage: &'a mut PageSliceCache<'pc, PC>) -> Self {
        let regs_64 = [WasmerSvmReg64::new(); REGS_64_COUNT];
        let regs_160 = [WasmerSvmReg160::new(); REGS_160_COUNT];
        let regs_256 = [WasmerSvmReg256::new(); REGS_256_COUNT];
        let regs_512 = [WasmerSvmReg512::new(); REGS_512_COUNT];

        Self {
            node_data,
            regs_64,
            regs_160,
            regs_256,
            regs_512,
            storage,
        }
    }
}

impl<'a, 'pc: 'a, PC> Drop for SvmCtx<'a, 'pc, PC> {
    fn drop(&mut self) {
        drop(&mut self.storage);
    }
}
