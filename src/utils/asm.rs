use core::arch::asm;

use crate::internal::gdt::GdtDescriptor;
use crate::internal::idt::IdtDescriptor;

#[inline(always)]
pub unsafe fn outb(port: u16, value: u8) {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}
#[inline(always)]
pub unsafe fn outw(port: u16, value: u16) {
    unsafe {
        core::arch::asm!(
            "outw dx, ax",
            in("dx") port,
            in("ax") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}
#[inline(always)]
pub unsafe fn outl(port: u16, value: u32) {
    unsafe {
        core::arch::asm!(
            "outl dx, eax",
            in("dx") port,
            in("eax") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let mut value: u8;
    unsafe {
        core::arch::asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}
#[inline(always)]
pub unsafe fn inw(port: u16) -> u16 {
    let mut value: u16;
    unsafe {
        core::arch::asm!(
            "in ax, dx",
            in("dx") port,
            out("ax") value,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}
#[inline(always)]
pub unsafe fn inl(port: u16) -> u32 {
    let mut value: u32;
    unsafe {
        core::arch::asm!(
            "inl eax, dx",
            in("dx") port,
            out("eax") value,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}

#[inline(always)]
pub unsafe fn io_wait() {
    unsafe {
        outb(0x80, 0);
    }
}

#[inline(always)]
pub unsafe fn outcr2() -> u32 {
    unsafe {
        let value: u32;
        asm!(
            "mov {0:e}, cr2",
            out(reg) value,
            options(nomem, nostack, preserves_flags)
        );
        value
    }
}
#[inline(always)]
pub unsafe fn incr3(addr: u32) {
    unsafe {
        asm!(
            "mov cr3, {0:e}",
            in(reg) addr,
            options(nostack, preserves_flags)
        )
    }
}
#[inline(always)]
pub unsafe fn outcr3() -> u32 {
    unsafe {
        let value: u32;
        asm!(
            "mov {0:e}, cr3",
            out(reg) value,
            options(nomem, nostack, preserves_flags)
        );
        value
    }
}

#[inline(always)]
pub unsafe fn cli() {
    unsafe { asm!("cli", "nop", options(nostack, preserves_flags)) }
}

#[inline(always)]
pub unsafe fn sti() {
    unsafe { asm!("sti", "nop", options(nostack, preserves_flags)) }
}

#[inline(always)]
pub fn hlt() {
    unsafe { asm!("hlt", options(nostack, nomem)) }
}

#[inline(always)]
pub fn lgdt(descriptor: *const GdtDescriptor) {
    unsafe {
        asm!("cli", "lgdt [{}]", in(reg) descriptor, options(readonly, nostack, preserves_flags));
    }
}
#[inline(always)]
pub fn lidt(descriptor: *const IdtDescriptor) {
    unsafe {
        asm!("cli", "lidt [{}]", in(reg) descriptor, 
            options(readonly, nostack, preserves_flags));
    }
}
#[inline(always)]
pub fn iretq() {
    unsafe { asm!("iretq", options(noreturn)) }
}
