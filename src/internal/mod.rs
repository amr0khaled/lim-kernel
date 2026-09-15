use crate::info;

pub mod gdt;
pub mod idt;
mod interrupts;
mod pic;
pub mod statics;

pub fn init() {
    info!("Internals Initializing");
    gdt::init();
    idt::init();
}
