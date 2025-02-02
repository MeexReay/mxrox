use core::arch::asm;

pub unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nostack, preserves_flags));
}

pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("inb %dx, %al", in("dx") port, out("al") result, options(nostack, preserves_flags, att_syntax));
    result
}

pub unsafe fn nop() {
    asm!("nop", options(nostack, preserves_flags));
}

pub unsafe fn io_wait() {
    outb(0x80, 0);
}

pub unsafe fn sti() {
    asm!("sti");
}

pub unsafe fn cli() {
    asm!("cli");
}