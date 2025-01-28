const VGA_BUFFER: *mut u16 = 0xB8000 as *mut u16;

pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;

pub struct VgaColor(u8);

impl VgaColor {
    pub const BLACK: u8 = 0;
    pub const BLUE: u8 = 1;
    pub const GREEN: u8 = 2;
    pub const CYAN: u8 = 3;
    pub const RED: u8 = 4;
    pub const MAGENTA: u8 = 5;
    pub const BROWN: u8 = 6;
    pub const LIGHT_GREY: u8 = 7;
    pub const DARK_GREY: u8 = 8;
    pub const LIGHT_BLUE: u8 = 9;
    pub const LIGHT_GREEN: u8 = 10;
    pub const LIGHT_CYAN: u8 = 11;
    pub const LIGHT_RED: u8 = 12;
    pub const LIGHT_MAGENTA: u8 = 13;
    pub const LIGHT_BROWN: u8 = 14;
    pub const WHITE: u8 = 15;
}

pub fn put_char_by_index(index: usize, c: char, bg: u8, fg: u8) {
    let c16 = c as u16;
    let color = fg | bg << 4;
    let color16 = color as u16;

    unsafe {
        ptr::write_volatile(VGA_BUFFER.add(index), c16 | (color16 << 8));
    }
}

pub fn put_char(x: usize, y: usize, c: char, bg: u8, fg: u8) {
    put_char_by_index(y * VGA_WIDTH + x, c, bg, fg)
}

pub fn put_string_by_index(index: usize, text: &str, bg: u8, fg: u8) {
    for (i, c) in text.char_indices() {
        put_char_by_index(index + i, c, bg, fg);
    }
}

pub fn put_string(x: usize, y: usize, text: &str, bg: u8, fg: u8) {
    put_string_by_index(y * VGA_WIDTH + x, text, bg, fg)
}

pub fn fill(c: char, bg: u8, fg: u8) {
    for x in 0..VGA_WIDTH {
        for y in 0..VGA_HEIGHT {
            put_char(x, y, c, bg, fg);
        }
    }
}

pub fn fill_with_color(color: u8) {
    fill(' ', color, 0)
}
