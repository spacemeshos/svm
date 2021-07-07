/// When code is compiled for Wasm, we want to abort execution immediately
/// without any proper unwinding.
#[cfg(target_arch = "wasm32")]
pub fn panic() -> ! {
    core::intrinsics::abort();
}

/// When code isn't compiled for Wasm, we just fallback to `panic!`.
#[cfg(not(target_arch = "wasm32"))]
pub fn panic() -> ! {
    core::panic!()
}
