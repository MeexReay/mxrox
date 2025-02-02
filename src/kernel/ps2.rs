use alloc::vec::Vec;

use crate::kernel::interrupt::idt_set_descriptor;

use super::{interrupt::send_eoi, terminal::log_info, time::sleep, util::*};

const DATA_PORT: u16 = 0x60;
const STATUS_PORT: u16 = 0x64;

const DEVICE1_IRQ: usize = 1;
const DEVICE2_IRQ: usize = 12;

static mut DEVICE1_TYPE: Vec<u8> = Vec::new();
static mut DEVICE2_TYPE: Vec<u8> = Vec::new();

unsafe fn send_data(data: u8) {
    outb(DATA_PORT, data)
} 

unsafe fn read_data() -> u8 {
    inb(DATA_PORT)
}

unsafe fn send_command(command: u8) {
    outb(STATUS_PORT, command)
} 

unsafe fn read_status() -> u8 {
    inb(STATUS_PORT)
} 

unsafe fn on_device_1_irq() {
    let data = read_data();
    log_info(&format!("[PS/2] device 1 data: 0x{:X}", data));
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
    false
}

pub fn init_ps2() {
    unsafe {
        send_command(0xAD); // disable device 1
    
        log_info("[PS/2] Disabled device 1");

        if is_ps2_2_device_enabled() {
            // DEVICE2_TYPE = send_identify();
            log_info("[PS/2] Inspected device 2 identify");
            send_command(0xA7); // disable device 2
            log_info("[PS/2] Disabled device 2");
        }

        read_data(); // flush device data
        send_command(0x60); // set CCB command
        send_command(0x000000u8); // clear CCB

        log_info("[PS/2] Flush device data and clear CCB");

        send_command(0xAE); // enable device 1
        log_info("[PS/2] Enable device 1");
        // DEVICE1_TYPE = send_identify();
        log_info("[PS/2] Inspected device 1 identify");

        if is_ps2_2_device_enabled() {
            send_command(0xA8); // enable device 2
            log_info("[PS/2] Disable device 2");
        }

        idt_set_descriptor(0x21, on_device_1_irq as u32, 0x09); // bind handler for device 1
        log_info("[PS/2] Bind handler for device 1");
        if is_ps2_2_device_enabled() {
            idt_set_descriptor(0x8B, on_device_2_irq as u32, 0x74); // bind handler for device 2
            log_info("[PS/2] Bind handler for device 2");
        }
    }
}