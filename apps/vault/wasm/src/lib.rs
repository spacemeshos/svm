#![no_std]
#![feature(core_intrinsics, lang_items, alloc_error_handler)]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

use core::alloc::{GlobalAlloc, Layout};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use svm_app_vault_core as api;

#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn initialize(vault_ty: i32, spending_limit: i64) {
    api::initialize(vault_ty.into(), spending_limit.into())
}

#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn withdraw(vault_ty: i32, amount: i64) {
    api::withdraw(vault_ty.into(), amount.into())
}

#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn daily_withdraw(amount: i64) {
    api::daily_withdraw(amount.into())
}

#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn set_spending_account() {
    api::set_spending_account();
}

#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn set_spending_limit(amount: i64) {
    api::set_spending_limit(amount.into())
}

#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn cancel_action(action: i32) {
    // api::cancel_action(action.into());
}

#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn get_pending_withdraw() -> i32 {
    api::get_pending_withdraw() as _
}

#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn get_pending_spending_account() -> i32 {
    api::get_pending_spending_account() as _
}

#[no_mangle]
#[cfg(not(test))]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn get_pending_spending_limit() -> i32 {
    api::get_pending_spending_limit() as _
}

#[panic_handler]
#[cfg(not(test))]
#[no_mangle]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
    unsafe {
        ::core::intrinsics::abort();
    }
}

#[alloc_error_handler]
#[allow(improper_ctypes_definitions)]
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn oom(_: ::core::alloc::Layout) -> ! {
    unsafe {
        ::core::intrinsics::abort();
    }
}

#[lang = "eh_personality"]
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn eh_personality() {}
