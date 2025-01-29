use core::ptr::{read_volatile, write_volatile};

use no_std_compat::vec::Vec;

const DATA_PORT: *mut u8 = 0x60 as *mut u8;
const STATUS_PORT: *mut u8 = 0x64 as *mut u8;

fn write_ps2_data(data: u8) {
    unsafe { write_volatile(DATA_PORT, data) }
} 

fn read_ps2_data() -> u8 {
    unsafe { read_volatile(DATA_PORT) }
} 

fn send_ps2_command(command: u8) {
    unsafe { write_volatile(STATUS_PORT, command) }
} 

pub fn read_ps2_status() -> u8 {
    unsafe { read_volatile(STATUS_PORT) }
} 

/// returns device type bytes
fn init_ps2_controller() -> Vec<u8> {
    todo!()
}