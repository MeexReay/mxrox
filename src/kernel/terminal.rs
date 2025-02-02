use alloc::{string::{String, ToString}, vec::Vec};
use spin::RwLock;

use super::vga::{put_string, VGA_COLOR_BLACK, VGA_COLOR_WHITE, VGA_HEIGHT, VGA_WIDTH};
use super::time::get_time_millis;

static TEXT: RwLock<String> = RwLock::new(String::new());

pub fn update() {
    let text = TEXT.read().to_string();
    let mut lines: Vec<String> = Vec::new();

    for line in text.as_str().lines() {
        let chars: Vec<char> = line.chars().collect();
        lines.append(&mut chars.chunks(VGA_WIDTH)
            .map(|chunk| chunk.iter().collect())
            .collect::<Vec<String>>())
    }

    while lines.len() > VGA_HEIGHT {
        lines.remove(0);
    }

    for y in 0..VGA_HEIGHT {
        if let Some(line) = lines.get(y) {
            put_string(0, y, &(line.to_string()+&" ".repeat(VGA_WIDTH-line.len())), VGA_COLOR_BLACK, VGA_COLOR_WHITE);
        } else {
            put_string(0, y, &" ".repeat(VGA_WIDTH), VGA_COLOR_BLACK, VGA_COLOR_BLACK);
        }
    }
}

pub fn print(text: &str) {
    TEXT.write().push_str(text);
    update();
}

pub fn println(text: &str) {
    print(&format!("{text}\n"))
}

pub fn log_info(text: &str) {
    print(&format!("[{}] [INFO] {text}\n", get_time_millis()))
}

pub fn log_error(text: &str) {
    print(&format!("[{}] [FATAL] {text}\n", get_time_millis()))
}