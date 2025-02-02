use core::ptr::write_volatile;
use core::arch::asm;

use crate::kernel::util::{cli, sti};

use super::show_error;
use super::terminal::{log_error, log_info};
use super::util::{inb, io_wait, outb};

const APIC_BASE: u32 = 0xFEE00000;
const APIC_EOI: u32 = APIC_BASE + 0x0B0; // End Of Interrupt
const APIC_SVR: u32 = APIC_BASE + 0x0F0; // Spurious Interrupt Vector Register

const IA32_APIC_BASE_MSR: u32 = 0x1B;
const APIC_BASE_ENABLE: u64 = 1 << 11;

const PIC1_COMMAND: u16 = 0x0020;
const PIC1_DATA: u16 = 0x0021;
const PIC2_COMMAND: u16 = 0x00A0;
const PIC2_DATA: u16 = 0x00A1;

const ICW1_ICW4: u8 = 0x01;     /* Indicates that ICW4 will be present */
const ICW1_SINGLE: u8 = 0x02;       /* Single (cascade) mode */
const ICW1_INTERVAL4: u8 = 0x04;        /* Call address interval 4 (8) */
const ICW1_LEVEL: u8 = 0x08;        /* Level triggered (edge) mode */
const ICW1_INIT: u8 = 0x10;     /* Initialization - required! */

const ICW4_8086: u8 = 0x01;     /* 8086/88 (MCS-80/85) mode */
const ICW4_AUTO: u8 = 0x02;     /* Auto (normal) EOI */
const ICW4_BUF_SLAVE: u8 = 0x08;        /* Buffered mode/slave */
const ICW4_BUF_MASTER: u8 = 0x0C;       /* Buffered mode/master */
const ICW4_SFNM: u8 = 0x10;     /* Special fully nested (not) */

const PIC_EOI: u8 = 0x20;       /* End-of-interrupt command code */

const APIC_ENABLED: bool = false;

#[repr(C, packed)]
struct IDTDescriptor {
    limit: u16,
    base: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IDTEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_mid: u16,
}

const IDT_MAX_DESCRIPTORS: usize = 256;
static mut IDT: [IDTEntry; IDT_MAX_DESCRIPTORS] = [IDTEntry {
    offset_low: 0,
    selector: 0,
    ist: 0,
    type_attr: 0,
    offset_mid: 0,
}; IDT_MAX_DESCRIPTORS];

// static mut VECTORS: [bool; 32] = [false; 32];
static mut IDTR: IDTDescriptor = IDTDescriptor { limit: 0, base: 0 };

// extern "C" {
//     static ist_stub_table: [u32; 32];
// }

pub unsafe fn idt_set_descriptor(vector: u8, handler: u32, type_attr: u8) {
    IDT[vector as usize] = IDTEntry {
        offset_low: handler as u16,
        selector: 0x08, // Kernel code segment
        ist: type_attr,
        type_attr,
        offset_mid: (handler >> 16) as u16
    };
}

pub unsafe fn load_idt() {
    IDTR.base = &IDT as *const _ as u32;
    IDTR.limit = (size_of::<IDTEntry>() * IDT_MAX_DESCRIPTORS - 1) as u16;

    // for vector in 0..32 {
    //     idt_set_descriptor(vector, ist_stub_table[vector as usize], 0x8E);
    //     VECTORS[vector as usize] = true;
    // }
    
    asm!("lidt [{}]", in(reg) &IDTR);
    asm!("sti");
}

unsafe fn is_apic_available() -> bool {
    if !APIC_ENABLED {
        return false;
    }

    let edx: u32;

    asm!(
        "cpuid",
        inout("eax") 1 => _,
        lateout("edx") edx
    );

    (edx & (1 << 9)) != 0
}

pub unsafe extern "C" fn exception_handler() { 
    log_error("Unknown error"); 
    asm!("cli; hlt");
    loop {}
}

unsafe fn send_apic_eoi() {
    write_volatile(APIC_EOI as *mut u32, 0);
}

unsafe fn send_pic_eoi(irq: usize) {
    if irq >= 8 { 
        write_volatile(PIC2_COMMAND as *mut u8, PIC_EOI); 
    }
    write_volatile(PIC1_COMMAND as *mut u8, PIC_EOI);
}

pub unsafe fn init_pic() {
    let offset1 = 0x20;
    let offset2 = 0x28;

    let a1 = inb(PIC1_DATA);                        // save masks
    let a2 = inb(PIC2_DATA);
    
    outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);  // starts the initialization sequence (in cascade mode)
    io_wait();
    outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
    io_wait();
    outb(PIC1_DATA, offset1);                 // ICW2: Master PIC vector offset
    io_wait();
    outb(PIC2_DATA, offset2);                 // ICW2: Slave PIC vector offset
    io_wait();
    outb(PIC1_DATA, 4);                       // ICW3: tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
    io_wait();
    outb(PIC2_DATA, 2);                       // ICW3: tell Slave PIC its cascade identity (0000 0010)
    io_wait();
    
    outb(PIC1_DATA, ICW4_8086);               // ICW4: have the PICs use 8086 mode (and not 8080 mode)
    io_wait();
    outb(PIC2_DATA, ICW4_8086);
    io_wait();
    
    outb(PIC1_DATA, a1);   // restore saved masks.
    outb(PIC2_DATA, a2);
}

pub unsafe fn disable_pic() {
    outb(PIC1_DATA, 0xff);
    outb(PIC2_DATA, 0xff);
}

pub fn send_eoi(irq: usize) {
    unsafe {
        if is_apic_available() {
            send_apic_eoi()
        } else {
            send_pic_eoi(irq)
        }
    }
}

unsafe fn read_msr(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    asm!("rdmsr", in("ecx") msr, out("eax") low, out("edx") high);
    ((high as u64) << 32) | (low as u64)
}

unsafe fn write_msr(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    asm!("wrmsr", in("ecx") msr, in("eax") low, in("edx") high);
}

pub unsafe fn init_apic() {
    let apic_base = read_msr(IA32_APIC_BASE_MSR) | APIC_BASE_ENABLE;
    write_msr(IA32_APIC_BASE_MSR, apic_base);
    let svr_value: u32 = 0x1FF;
    write_volatile(APIC_SVR as *mut u32, svr_value);
    write_volatile(APIC_EOI as *mut u32, 0);
}

pub fn init_interrupts() {
    unsafe {
        init_pic();
        log_info("PIC initialized");

        if is_apic_available() {
            disable_pic();
            log_info("Disable PIC");
            init_apic();
            log_info("APIC initialized");
        }

        load_idt();
    }
}
