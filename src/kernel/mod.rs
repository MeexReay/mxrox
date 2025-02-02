use interrupt::init_interrupts;
use heap::init_heap;
use time::init_pit;
use ps2::init_ps2;
use terminal::{log_info, println};
use vga::{
    fill_with_color, 
    put_string_by_index, 
    VGA_COLOR_BLACK,
    VGA_COLOR_RED
};

mod vga;
mod ps2;
mod interrupt;
mod time;
mod heap;
mod util;
mod terminal;

pub fn show_error(message: &str) {
    fill_with_color(VGA_COLOR_BLACK);
    put_string_by_index(0, message, VGA_COLOR_BLACK, VGA_COLOR_RED);

    loop {}
}

pub fn init_kernel() {
    init_heap(0x200000, 1048576);
    init_interrupts();
    init_pit();
    // init_ps2();

    loop {}
}