use crate::register::WasmerReg64;

use svm_storage::PageSliceCache;

/// The number of allocated `64-bit` wasmer registers for each `SvmCtx`
pub const REGS_64_COUNT: usize = 8;

/// `SvmCtx` is a container for the accessible data by `wasmer` instances
/// Its fields are:
/// * `regs_64` - a static array (`REGS_64_COUNT` elements) of `WasmerReg64`
///
/// Explanation about `SvmCtx` lifetimes and generics:
/// * `a  - the lifetime of the mutable borrowed `PageSliceCache`
/// * `pc - the lifetime of the inner `PageCache` within `PageSliceCache` (`pc - stands for `PageCache`)
/// *  PC - a type implementing the trait `PageCache` (`PC` stands for `PageCache`)
#[repr(C)]
pub struct SvmCtx<'a, 'pc: 'a, PC> {
    pub(crate) regs_64: [WasmerReg64; REGS_64_COUNT],

    pub(crate) storage: &'a mut PageSliceCache<'pc, PC>,
}

impl<'a, 'pc: 'a, PC> SvmCtx<'a, 'pc, PC> {
    /// Initializes a new empty `SvmCtx`
    ///
    /// * `storage` - a mutably borrowed `PageSliceCache`
    pub fn new(storage: &'a mut PageSliceCache<'pc, PC>) -> Self {
        let regs_64 = [WasmerReg64::new(); REGS_64_COUNT];

        Self { regs_64, storage }
    }
}
