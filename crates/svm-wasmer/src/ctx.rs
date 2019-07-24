use crate::register::WasmerReg64;
use std::ffi::c_void;
use svm_storage::PageSliceCache;

/// The number of allocated `64-bit` wasmer registers for each `SvmCtx`
pub const REGS_64_COUNT: usize = 8;

/// `SvmCtx` is a container for the accessible data by `wasmer` instances
/// * `node_data` - a pointer to the *node* data
/// * `regs_64`   - a static array (`REGS_64_COUNT` elements) of `WasmerReg64`
/// * `storage`   - an instance of `PageSliceCache`
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

    /// An array that holds the `svm wasmer` registers
    pub regs_64: [WasmerReg64; REGS_64_COUNT],

    /// Accessor to the contract main storage (of type `PageSliceCache`)
    pub storage: &'a mut PageSliceCache<'pc, PC>,
}

impl<'a, 'pc: 'a, PC> SvmCtx<'a, 'pc, PC> {
    /// Initializes a new empty `SvmCtx`
    ///
    /// * `storage` - a mutably borrowed `PageSliceCache`
    pub fn new(node_data: *const c_void, storage: &'a mut PageSliceCache<'pc, PC>) -> Self {
        let regs_64 = [WasmerReg64::new(); REGS_64_COUNT];

        Self {
            node_data,
            regs_64,
            storage,
        }
    }
}

impl<'a, 'pc: 'a, PC> Drop for SvmCtx<'a, 'pc, PC> {
    fn drop(&mut self) {
        drop(&mut self.storage);
    }
}
