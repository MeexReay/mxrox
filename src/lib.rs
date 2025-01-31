#![no_std]
#![no_main]

#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(rustc_private)]
#![feature(abi_x86_interrupt)]


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

use core::panic::PanicInfo;

use kernel::{init_kernel, show_error};

mod kernel;

#[macro_use]
extern crate alloc;

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    show_error(info.message().as_str().unwrap_or("Unknown Error"));
    loop {}
}

#[alloc_error_handler]
fn oom(_layout: core::alloc::Layout) -> ! {
    show_error("Out Of Memory");
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    init_kernel();
    loop {}
}