use core::{arch::asm, ptr::addr_of};

use spin::Mutex;

use crate::{
    internal::idt::{Idt, IdtDescriptor},
    println,
    utils::asm::{lgdt, lidt},
};

#[repr(C, packed)]
pub struct GdtDescriptor {
    size: u16,
    address: u64,
}

#[repr(C, align(8))]
pub struct Gdt([GdtEntry; 5]);

impl Gdt {
    pub const fn new() -> Self {
        let ker_code: u64 = 1 << 44 | 1 << 47 | 1 << 41 | 1 << 53 | 1 << 43;
        let ker_data: u64 = 1 << 44 | 1 << 47 | 1 << 41 | 1 << 53;
        let usr_code: u64 = 1 << 44 | 1 << 47 | 1 << 41 | 1 << 53 | 1 << 43 | 3 << 45;
        let usr_data: u64 = 1 << 44 | 1 << 47 | 1 << 41 | 1 << 53 | 3 << 45;
        Self([
            // Entry 0: Null (required)
            GdtEntry(0),
            // Entry 1: Kernel Code (selector 0x08)
            GdtEntry(ker_code),
            // Entry 2: Kernel Data (selector 0x10)
            GdtEntry(ker_data),
            // Entry 3: User Code (selector 0x18)
            GdtEntry(usr_code),
            // Entry 4: User Data (selector 0x20)
            GdtEntry(usr_data),
        ])
    }

    pub fn address(&self) -> u64 {
        self as *const _ as u64
    }
}

#[repr(C, packed)]
pub struct GdtEntry(u64);

pub static GDT: Mutex<Gdt> = Mutex::new(Gdt::new());

pub extern "C" fn load_gdt() {
    let gdt = GDT.lock();
    let gdt_ptr = addr_of!((*gdt).0) as *const u64;

    let gdt_desc = GdtDescriptor {
        address: gdt_ptr as u64,
        size: (core::mem::size_of::<Gdt>() - 1) as u16,
    };
    lgdt(&gdt_desc as *const GdtDescriptor);
}

pub fn init() {
    println!("GDT: Loading");
    load_gdt();
    println!("GDT: Reloading segments");
    reload_segments();
}

pub fn load_basic_idt() {
    let idt = Idt::new();
    let desc = IdtDescriptor::from((core::mem::size_of::<Idt>() - 1) as u16, idt.address());

    lidt(&desc as *const IdtDescriptor);
}

#[inline(always)]
pub extern "C" fn reload_segments() {
    let code_sel: u16 = 0x8;
    let data_sel: u16 = 0x10;
    unsafe {
        asm!(
            "mov ds, {data_sel:x}",
            "mov es, {data_sel:x}",
            "mov fs, {data_sel:x}",
            "mov gs, {data_sel:x}",
            "mov ss, {data_sel:x}",
            "push {code_sel:r}",
            "lea {tmp}, [2f]",
            "push {tmp}",
            "retfq",
            "2:",
            data_sel = in(reg) data_sel,
            code_sel = in(reg) code_sel,
            tmp = out(reg) _,
            options(preserves_flags)
        );
    }
}
