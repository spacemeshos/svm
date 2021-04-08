#[cfg(target_arch = "wasm32")]
pub fn panic() -> ! {
    core::panic!()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn panic() -> ! {
    core::panic!()
}
