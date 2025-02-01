use core::ptr::{read_volatile, write_volatile};

use alloc::vec::Vec;

use crate::kernel::irq::register_idt;

use super::{irq::send_eoi, pit::sleep};

const DATA_PORT: u16 = 0x60;
const STATUS_PORT: u16 = 0x64;

const DEVICE1_IRQ: usize = 1;
const DEVICE2_IRQ: usize = 12;

static mut DEVICE1_TYPE: Vec<u8> = Vec::new();
static mut DEVICE2_TYPE: Vec<u8> = Vec::new();

unsafe fn send_data(data: u8) {
    write_volatile(DATA_PORT as *mut u8, data)
} 

unsafe fn read_data() -> u8 {
    read_volatile(DATA_PORT as *mut u8)
}

unsafe fn send_command(command: u8) {
    write_volatile(STATUS_PORT as *mut u8, command)
} 

unsafe fn read_status() -> u8 {
    read_volatile(STATUS_PORT as *mut u8)
} 

unsafe fn on_device_1_irq() {
    let data = read_data();
    send_eoi(DEVICE1_IRQ);
}

unsafe fn on_device_2_irq() {
    let data = read_data();
    send_eoi(DEVICE2_IRQ);
}

unsafe fn is_data_available() -> bool {
    (read_status() >> 0) & 1 == 1
}

// https://togglebit.io/posts/rust-bitwise/

unsafe fn read_identify_reply() -> Vec<u8> {
    let mut reply = Vec::new();
        
    for _ in 0..2 {
        if is_data_available() {
            reply.push(read_data());
        } else {
            sleep(10);

            if is_data_available() {
                reply.push(read_data());
            } else {
                break;
            }
        }
    }

    reply
}

unsafe fn send_identify() -> Vec<u8> {
    send_data(0xF5); // Disable Scanning command
    while !is_data_available() || read_data() != 0xFA { 
        sleep(1);
    }

    send_data(0xF2); // Identify command
    while !is_data_available() || read_data() != 0xFA { 
        sleep(1);
    }

    let reply = read_identify_reply();

    send_data(0xF4);

    reply
}

unsafe fn is_ps2_2_device_enabled() -> bool {
    true
}

pub fn init_ps2() {
    unsafe {
        send_command(0xAD); // disable device 1
        if is_ps2_2_device_enabled() {
            DEVICE2_TYPE = send_identify();
            send_command(0xA7); // disable device 2
        }

        read_data(); // flush device data
        send_command(0x60); // set CCB command
        send_command(0x000000u8); // clear CCB

        send_command(0xAE); // enable device 1
        if is_ps2_2_device_enabled() {
            DEVICE1_TYPE = send_identify();
            send_command(0xA8); // enable device 2
        }
    }

    register_idt(on_device_1_irq as u32, 0x09); // bind handler for device 1
    register_idt(on_device_2_irq as u32, 0x74); // bind handler for device 2
}