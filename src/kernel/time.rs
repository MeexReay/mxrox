use core::{ptr::write_volatile, sync::atomic::{AtomicUsize, Ordering}};
use alloc::{sync::Arc, vec::Vec};
use spin::RwLock;

use super::{interrupt::{idt_set_descriptor, load_idt, send_eoi}, terminal::log_info, util::{cli, io_wait, outb, sti}};

const DIVISOR: u16 = ((1193182u32 + 500u32) / 100) as u16;

static TIMERS: RwLock<Vec<Arc<AtomicUsize>>> = RwLock::new(Vec::new());
static TIME_MILLIS: AtomicUsize = AtomicUsize::new(0);
static TIME: AtomicUsize = AtomicUsize::new(0);

extern "C" fn pit_handler() {
    for t in TIMERS.read().iter() {
        t.fetch_add(1, Ordering::SeqCst);
    }

    if TIME_MILLIS.fetch_add(1, Ordering::SeqCst) >= 999 {
        TIME_MILLIS.store(0, Ordering::SeqCst);
        TIME.fetch_add(1, Ordering::SeqCst);
        log_info("proshlo second");
    }

    send_eoi(0);
}

pub fn get_time_seconds() -> usize {
    TIME.load(Ordering::SeqCst)
}

pub fn get_time_millis() -> usize {
    TIME.load(Ordering::SeqCst) * 1000 + TIME_MILLIS.load(Ordering::SeqCst)
}

pub fn init_pit() {
    log_info("PIT initialization");

    unsafe {
        outb(0x43, 0x36);
        cli();
        outb(0x40, (DIVISOR & 0xFF) as u8);
        outb(0x40, ((DIVISOR & 0xFF) >> 8) as u8);

        log_info("PIT initialized");
        log_info(&format!("{}", pit_handler as u32));
    
        idt_set_descriptor(0x20, pit_handler as u32, 0x8E);
        log_info("PIT registered");
        load_idt();
    
        log_info("PIT loaded idt");
    }
}

pub fn sleep(millis: usize) {
    let atomic = Arc::new(AtomicUsize::new(0));

    TIMERS.write().push(atomic.clone());

    while atomic.load(Ordering::SeqCst) < millis {
        core::hint::spin_loop();
    }

    TIMERS.write().retain(|t| !Arc::ptr_eq(t, &atomic));
}