use crate::{
    internal::{interrupts::InterruptStackFrame, pic::pic1_eoi, statics::PIC1_COMMAND},
    println,
    utils::asm::{inb, outb},
};

pub extern "x86-interrupt" fn keyboard_handler(_frame: &InterruptStackFrame) {
    unsafe {
        let key = inb(0x60);
        println!("{:x}", key);
        pic1_eoi();
    }
}
static TIMER_TICKS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);
pub unsafe extern "x86-interrupt" fn timer_handler(_frame: &InterruptStackFrame) {
    TIMER_TICKS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    unsafe {
        // Send EOI (0x20) to Master PIC Command Port (0x20)
        outb(PIC1_COMMAND, 0x20);
    }
}
