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
/// * `pc - the lifetime of the inner `PageCache` within `PageSliceCache` (`pc - stands for page-cache)
/// *  PS - a type implementing the trait `PagesStorage` (`PS` stands for `PagesStorage`)
#[repr(C)]
pub struct SvmCtx<'a, 'pc: 'a, PS> {
    pub(crate) regs_64: [WasmerReg64; REGS_64_COUNT],

    pub(crate) storage: &'a mut PageSliceCache<'pc, PS>,
}

impl<'a, 'pc: 'a, PS> SvmCtx<'a, 'pc, PS> {
    /// Initializes a new empty `SvmCtx`
    ///
    /// * `storage` - a mutably borrowed `PageSliceCache`
    pub fn new(storage: &'a mut PageSliceCache<'pc, PS>) -> Self {
        let regs_64 = [WasmerReg64::new(); REGS_64_COUNT];

        Self { regs_64, storage }
    }
}
