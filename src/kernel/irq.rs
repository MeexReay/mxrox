use core::ptr::write_volatile;
use core::arch::asm;

const APIC_BASE: u32 = 0xFEE00000;

const APIC_EOI: u32 = APIC_BASE + 0x0B0;
const APIC_SVR: u32 = APIC_BASE + 0x0F0;

const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;
const ICW4_8086: u8 = 0x01;

const PIC_EOI: u8 = 0x20;       /* End-of-interrupt command code */

const APIC_ENABLED: bool = false;


#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_middle: u16
}

pub const IDT_SIZE: usize = 256;

#[repr(C)]
pub struct Idt {
    entries: [IdtEntry; IDT_SIZE],
}

static mut IDT: Idt = Idt {
    entries: [IdtEntry {
        offset_low: 0,
        selector: 0,
        ist: 0,
        type_attr: 0,
        offset_middle: 0,
    }; IDT_SIZE],
};

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

unsafe fn init_idt() {
    asm!("lidt [{}]", "sti", in(reg) &IDT as *const _);
}

unsafe fn init_apic() {
    write_volatile(APIC_SVR as *mut u32, 0x100 | 0x1);
}

unsafe fn send_apic_eoi() {
    write_volatile(APIC_EOI as *mut u32, 0);
}

unsafe fn send_pic_eoi(irq: usize) {
    if irq >= 8 { write_volatile(PIC2_CMD as *mut u8, PIC_EOI); }
    write_volatile(PIC1_CMD as *mut u8, PIC_EOI);
}

unsafe fn init_pic() {
    write_volatile(PIC1_CMD as *mut u8, ICW1_INIT | ICW1_ICW4);
    write_volatile(PIC2_CMD as *mut u8, ICW1_INIT | ICW1_ICW4);
    
    write_volatile(PIC1_DATA as *mut u8, 0x20);
    write_volatile(PIC2_DATA as *mut u8, 0x28);
    
    write_volatile(PIC1_DATA as *mut u8, 4);
    write_volatile(PIC2_DATA as *mut u8, 2);
    
    write_volatile(PIC1_DATA as *mut u8, ICW4_8086);
    write_volatile(PIC2_DATA as *mut u8, ICW4_8086);

    write_volatile(PIC1_DATA as *mut u8, 0xFB);
    write_volatile(PIC2_DATA as *mut u8, 0xFF);
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

/// all interrupt vectors you can find here: https://wiki.osdev.org/Interrupt_Vector_Table
pub fn register_idt(handler: u32, int_vec: u8) {
    unsafe {
        let entry = &mut IDT.entries[int_vec as usize];

        entry.offset_low = handler as u16;
        entry.selector = 0x08;
        entry.ist = 0;
        entry.type_attr = 0x8E;
        entry.offset_middle = (handler >> 16) as u16;
    }
}

pub fn init_interrupts() {
    unsafe {
        init_idt();
        if is_apic_available() {
            init_apic();
        } else {
            init_pic();
        }
    }
}