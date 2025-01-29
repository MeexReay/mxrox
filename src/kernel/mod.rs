use heap::init_heap;
use ps2::read_ps2_status;
use vga::{
    fill_with_color, 
    put_string, 
    put_string_by_index, 
    VGA_COLOR_BLACK,
    VGA_COLOR_LIGHT_MAGENTA, 
    VGA_COLOR_RED
};
use no_std_compat::string::ToString;

mod vga;
mod ps2;
mod acpi;
mod thread;
mod heap;
mod util;

pub fn show_error(message: &str) {
    fill_with_color(VGA_COLOR_BLACK);
    put_string_by_index(0, message, VGA_COLOR_BLACK, VGA_COLOR_RED);
}

pub fn init_kernel() {
    init_heap(16400, 16384);

    fill_with_color(VGA_COLOR_BLACK);

    loop {
        put_string(
            0, 0, 
            &format!("ps/2 status: 0x{:x}", read_ps2_status()), 
            VGA_COLOR_BLACK, 
            VGA_COLOR_LIGHT_MAGENTA
        );
    }
}