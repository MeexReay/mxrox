use core::ptr::write_volatile;
use core::arch::asm;

const APIC_BASE: *mut u32 = 0xFEE00000 as *mut u32;

const APIC_EOI: *mut u32 = unsafe { APIC_BASE.add(0x0B0) };
const APIC_SVR: *mut u32 = unsafe { APIC_BASE.add(0x0F0) };

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

fn register_idt(handler: u32) {
    unsafe {
        let entry = &mut IDT.entries[32];

        entry.offset_low = handler as u16;
        entry.selector = 0x08;
        entry.ist = 0;
        entry.type_attr = 0x8E;
        entry.offset_middle = (handler >> 16) as u16;
    }
}

unsafe fn load_idt() {
    asm!("lidt [{}]", "sti", in(reg) &IDT as *const _);
}

unsafe fn enable_apic() {
    let sv_reg = APIC_SVR as *mut u32;
    let mut value = 0x100 | 0x1;
    write_volatile(sv_reg, value);
}

unsafe fn send_eoi() {
    let eoi_reg = APIC_EOI as *mut u32;
    write_volatile(eoi_reg, 0);
}

pub fn init_apic() {
    unsafe {
        load_idt();
        enable_apic();
    }
}