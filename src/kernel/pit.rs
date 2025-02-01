use core::{ptr::write_volatile, sync::atomic::{AtomicUsize, Ordering}};
use alloc::{sync::Arc, vec::Vec};
use spin::RwLock;
use core::sync::atomic::AtomicBool;

use super::irq::{register_idt, send_eoi};

const DIVISOR: u16 = ((1193182u32 + 500u32) / 1000) as u16;

static TIMERS: RwLock<Vec<Arc<AtomicUsize>>> = RwLock::new(Vec::new());

extern "C" fn pit_handler() {
    for t in TIMERS.read().iter() {
        t.fetch_add(1, Ordering::SeqCst);
    }
    send_eoi(0);
}

pub fn init_pit() {
    unsafe {
        write_volatile(0x43 as *mut u8, 0x36);
        write_volatile(0x40 as *mut u8, (DIVISOR & 0xFF) as u8);
        write_volatile(0x40 as *mut u8, ((DIVISOR >> 8) & 0xFF) as u8);
    }
    register_idt(pit_handler as u32, 0x08);
}

pub fn sleep(millis: usize) {
    let atomic = Arc::new(AtomicUsize::new(0));

    TIMERS.write().push(atomic.clone());

    while atomic.load(Ordering::SeqCst) < millis {
        core::hint::spin_loop(); // Уменьшаем нагрузку на CPU
    }

    TIMERS.write().retain(|t| !Arc::ptr_eq(t, &atomic));
}