#![no_std]
#![no_main]

#![feature(lang_items)]

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

use core::panic::PanicInfo;
use core::sync::atomic;
use core::sync::atomic::Ordering;


#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}


mod terminal;

use terminal::{fill, put_string, VgaColor};

#[no_mangle]
fn main() -> ! {
    fill(' ', VgaColor::BLACK, VgaColor::BLACK);
    put_string(0, 0, "Hello World from MxRox!", VgaColor::BLACK, VgaColor::WHITE);

    loop {}
}