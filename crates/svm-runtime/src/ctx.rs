use std::collections::HashMap;
use std::ffi::c_void;

use log::debug;

use crate::{buffer::BufferRef, helpers::DataWrapper, register::Registers};

use svm_app::types::HostCtx;
use svm_storage::AppStorage;

/// `SvmCtx` is a container for the accessible data by `wasmer` instances
/// * `host`     - A pointer to the `Host`
/// * `host_ctx` - A pointer to the `HostCtx` (i.e: `sender`, `block_id`, `nonce`, ...)
/// * `buffers`  - A `HashMap` between `buffer_id` to mutable/read-only `Buffer`.
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

    /// Raw pointer to host context fields.
    pub host_ctx: *const HostCtx,

    pub regs: Registers,

    pub buffers: HashMap<u32, BufferRef>,

    /// An accessor to the app's storage (`AppStorage`)
    pub storage: AppStorage,
}

unsafe impl Sync for SvmCtx {}
unsafe impl Send for SvmCtx {}

impl SvmCtx {
    /// Initializes a new empty `SvmCtx`
    ///
    /// * `storage` - a mutably borrowed `AppStorage`
    pub fn new(
        host: DataWrapper<*mut c_void>,
        host_ctx: DataWrapper<*const c_void>,
        storage: AppStorage,
    ) -> Self {
        let host = host.unwrap();
        let host_ctx = host_ctx.unwrap() as *const HostCtx;
        let buffers = HashMap::new();
        let regs = Registers::default();

        Self {
            host,
            host_ctx,
            buffers,
            regs,
            storage,
        }
    }
}

impl Drop for SvmCtx {
    fn drop(&mut self) {
        debug!("Dropping `SvmCtx`...");

        unsafe {
            let _ = Box::from_raw(self.host_ctx as *mut HostCtx);
        }
    }
}
