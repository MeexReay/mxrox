use vga::{fill, fill_with_color, put_char, put_string_by_index, read_char, VgaColor};

mod vga;
mod ps2;
mod acpi;
mod thread;

pub fn show_error(message: &str) {
    put_string_by_index(0, message, VgaColor::BLACK, VgaColor::LIGHT_RED);
}

pub fn start_kernel() {
    fill_with_color(VgaColor::BLACK);
}