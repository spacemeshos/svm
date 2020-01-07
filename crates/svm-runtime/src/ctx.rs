use crate::alloc_regs;

use log::debug;
use std::ffi::c_void;

use svm_storage::AppStorage;

use crate::{
    helpers::PtrWrapper,
    register::{SvmReg, SvmReg160, SvmReg32, SvmReg512, SvmReg64},
};

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
/// * `host`     - A pointer to the *Host*
/// * `regs_32`  - A static array (`REGS_32_COUNT` elements)  of `SvmReg32`
/// * `regs_64`  - A static array (`REGS_64_COUNT` elements)  of `SvmReg64`
/// * `regs_160` - A static array (`REGS_160_COUNT` elements) of `SvmReg160`
/// * `regs_256` - A static array (`REGS_256_COUNT` elements) of `SvmReg256`
/// * `regs_512` - A static array (`REGS_512_COUNT` elements) of `SvmReg512`
/// * `storage`  - An instance of `AppStorage`
#[repr(C)]
pub struct SvmCtx {
    /// A pointer to the `host`.
    ///
    /// For example, `host` will point a to struct having an access to the balance of each account.
    pub host: *mut c_void,

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

    /// An accessor to the app's storage
    pub storage: AppStorage,
}

unsafe impl Sync for SvmCtx {}
unsafe impl Send for SvmCtx {}

impl SvmCtx {
    /// Initializes a new empty `SvmCtx`
    ///
    /// * `storage` - a mutably borrowed `AppStorage`
    pub fn new(host: PtrWrapper, storage: AppStorage) -> Self {
        let regs_32 = alloc_regs!(32, REGS_32_COUNT);
        let regs_64 = alloc_regs!(64, REGS_64_COUNT);
        let regs_160 = alloc_regs!(160, REGS_160_COUNT);
        let regs_256 = alloc_regs!(256, REGS_256_COUNT);
        let regs_512 = alloc_regs!(512, REGS_512_COUNT);

        Self {
            host: host.unwrap(),
            regs_32,
            regs_64,
            regs_160,
            regs_256,
            regs_512,
            storage,
        }
    }
}

impl Drop for SvmCtx {
    fn drop(&mut self) {
        debug!("Dropping `SvmCtx`...");
    }
}
