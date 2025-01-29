#![no_std]
#![no_main]

#![feature(lang_items)]

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

use core::panic::PanicInfo;

use kernel::{init_kernel, show_error};

mod kernel;

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    show_error(info.message().as_str().unwrap_or("mxrox death"));
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    init_kernel();
    loop {}
}