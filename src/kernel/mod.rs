use stable_vec::StableVec;
use vga::{
    fill_with_color, 
    put_string_by_index, 
    VGA_COLOR_BLACK, 
    VGA_COLOR_RED
};

mod vga;
mod ps2;
mod acpi;
mod thread;

type Vec<T> = StableVec<T>;

pub fn show_error(message: &str) {
    fill_with_color(VGA_COLOR_BLACK);
    put_string_by_index(0, message, VGA_COLOR_BLACK, VGA_COLOR_RED);
}

pub fn start_kernel() {
    show_error("error test");
}