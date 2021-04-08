// #[cfg(target_arch = "wasm32")]
// pub fn panic() -> ! {
//     core::intrinsics::abort()
// }

// #[cfg(not(target_arch = "wasm32"))]
// pub fn panic() -> ! {
//     core::panic!()
// }

pub fn panic() -> ! {
    core::panic!()
}
