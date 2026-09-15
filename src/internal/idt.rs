use core::ptr::addr_of;

use spin::{Mutex, MutexGuard};

use crate::{
    internal::{
        interrupts::{
            exceptions::exception_handlers_init,
            io::{keyboard_handler, timer_handler},
        },
        pic::{enable_irq, remap},
        statics::{PIC1_INTERRUPT, PIC2_INTERRUPT},
    },
    println,
    utils::asm::{lidt, sti},
};

// OFFSETS
const SEGMENT: usize = 16;
const OFFSET_LOW: usize = 0;
const OFFSET_MID: usize = 48;
const OFFSET_HIG: usize = 64;

// FLAGS
const PRESNET_FLAG: usize = 47;
const RING_FLAG: usize = 45;
const GATE_TYPE_FLAG: usize = 40;
const IST_FLAG: usize = 40;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IdtEntry(u128);

impl IdtEntry {
    pub const KERNEL_CODE_SEGMENT: u16 = 0x8;
    pub const USER_CODE_SEGMENT: u16 = 0x18;
    pub const fn new(handler: u64, ring: u8, is_not_interrupt: bool, segment: u16) -> Self {
        let offset_low = (handler & 0xffff) as u16;
        let offset_mid = ((handler >> 16) & 0xffff) as u16;
        let offset_high = ((handler >> 32) & 0xffff) as u16;
        let mut v: u128 = 0;
        v |= (offset_low as u128) << OFFSET_LOW
            | (offset_mid as u128) << OFFSET_MID
            | (offset_high as u128) << OFFSET_HIG;
        // SEGMENT
        v |= (segment as u128) << SEGMENT;
        // FLAGS
        let flags: u64 = 1u64 << PRESNET_FLAG
            | (ring as u64 & 3) << RING_FLAG
            | 1 << GATE_TYPE_FLAG
            | if is_not_interrupt { 0xF } else { 0xE } << IST_FLAG;
        v |= flags as u128;

        Self(v)
    }
    pub fn new_kernel_handler(handler: u64, is_not_interrupt: bool) -> Self {
        IdtEntry::new(handler, 0, is_not_interrupt, Self::KERNEL_CODE_SEGMENT)
    }
    pub fn new_user_handler(handler: u64, is_not_interrupt: bool) -> Self {
        IdtEntry::new(handler, 3, is_not_interrupt, Self::USER_CODE_SEGMENT)
    }
}

pub struct Idt([IdtEntry; 256]);
impl Idt {
    pub const fn new() -> Self {
        let default = IdtEntry::new(0x0, 0, false, 0x0);

        Self([default; 256])
    }
    pub fn set(&mut self, index: u8, entry: IdtEntry) {
        let target = &mut self.0[index as usize];
        *target = entry;
    }
    pub fn address(&self) -> u64 {
        self as *const _ as u64
    }
}

#[repr(C, packed)]
pub struct IdtDescriptor {
    size: u16,
    address: u64,
}

impl IdtDescriptor {
    pub const fn new() -> Self {
        Self {
            size: 0,
            address: 0,
        }
    }
    pub fn set(&mut self, limit: u16, base: u64) {
        self.size = limit;
        self.address = base;
    }
    pub fn from(size: u16, address: u64) -> Self {
        Self { size, address }
    }
}

pub static IDT: Mutex<Idt> = Mutex::new(Idt::new());
pub static IDT_DESC: Mutex<IdtDescriptor> = Mutex::new(IdtDescriptor::new());

pub fn load() {
    let idt = IDT.lock();
    let idt_ptr = addr_of!((*idt).0) as *const u64;
    // let mut desc = IDT_DESC.lock();
    // desc.set((core::mem::size_of::<Idt>() - 1) as u16, idt_ptr as u64);

    let desc = IdtDescriptor {
        size: (core::mem::size_of::<Idt>() - 1) as u16,
        address: idt_ptr as u64,
    };
    lidt(&desc as *const IdtDescriptor);
}

pub fn init() {
    unsafe {
        println!("IDT: Remapping PICs");
        remap(PIC1_INTERRUPT, PIC2_INTERRUPT);
        println!("IDT: Init exception handlers");
        {
            let mut idt: MutexGuard<'static, Idt> = IDT.lock();
            println!("IDT: Init interrupts handlers");
            exception_handlers_init(&mut idt);
            idt.set(
                32,
                IdtEntry::new_kernel_handler(timer_handler as *const () as u64, false),
            );
            idt.set(
                33,
                IdtEntry::new_kernel_handler(keyboard_handler as *const () as u64, false),
            );
        }
        println!("IDT: Loading");
        load();
        println!("IDT: Initilize Timer");
        // init_timer(100);
        println!("IDT: Enable IRQ");
        enable_irq(0);
        enable_irq(1);
        println!("IDT: Enable interrupts");
        sti();
    }
    println!("IDT: Done");
}
