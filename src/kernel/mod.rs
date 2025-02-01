use irq::init_interrupts;
use heap::init_heap;
use pit::init_pit;
use ps2::init_ps2;
use vga::{
    fill_with_color, 
    put_string, 
    put_string_by_index, 
    VGA_COLOR_BLACK,
    VGA_COLOR_LIGHT_MAGENTA, 
    VGA_COLOR_RED
};

mod vga;
mod ps2;
mod irq;
mod pit;
mod heap;
mod util;

pub fn show_error(message: &str) {
    fill_with_color(VGA_COLOR_BLACK);
    put_string_by_index(0, message, VGA_COLOR_BLACK, VGA_COLOR_RED);
}

pub fn init_kernel() {
    init_heap(16400, 16384);
    init_pit();
    init_ps2();
    init_interrupts();

    fill_with_color(VGA_COLOR_BLACK);

    loop {}
}