#![no_std]
#![feature(lang_items)]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

#[cfg(not(feature = "std"))]
pub mod types;

#[cfg(not(feature = "std'"))]
pub mod value;

use core::panic::PanicInfo;

#[panic_handler]
#[cfg(not(test))]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[cfg(not(test))]
extern "C" fn rust_eh_personality() {}
