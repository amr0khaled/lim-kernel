use crate::internal::statics::*;
use crate::utils::asm::{inb, io_wait, outb};

pub unsafe fn remap(offset1: u8, offset2: u8) {
    unsafe {
        // initialise ICW1 & ICW4 on PIC1/2
        outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
        io_wait();
        outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
        io_wait();

        // ICW2: set vector offsets
        outb(PIC1_DATA, offset1);
        io_wait();
        outb(PIC2_DATA, offset2);
        io_wait();
        // ICW3: write Master with slave at IRQ2
        outb(PIC1_DATA, 1 << CASECADE_IRQ);
        io_wait();
        // ICW3: write slave with Master at IRQ2
        outb(PIC2_DATA, CASECADE_IRQ);
        io_wait();

        // ICW4: Enable 8086 mode and not 8080
        outb(PIC1_DATA, ICW4_8086);
        io_wait();
        outb(PIC2_DATA, ICW4_8086);
        io_wait();

        // Unmask both PICs
        outb(PIC1_DATA, 0b1111_1000);
        outb(PIC2_DATA, 0xff);
    }
}

pub unsafe fn disable() {
    unsafe {
        outb(PIC1_DATA, 0xff);
        outb(PIC2_DATA, 0xff);
    }
}

pub unsafe fn set_mask(irq_line: u8) {
    let port: u16;
    let value: u8;
    if irq_line < 8 {
        port = PIC1_DATA
    } else {
        port = PIC2_DATA
    }
    unsafe {
        value = inb(port) | (1 << irq_line);
        outb(port, value);
    }
}
pub unsafe fn clear_mask(irq_line: u8) {
    let port: u16;
    let value: u8;
    if irq_line < 8 {
        port = PIC1_DATA
    } else {
        port = PIC2_DATA
    }
    unsafe {
        value = inb(port) & !(1 << irq_line);
        outb(port, value);
    }
}

pub unsafe fn pic1_eoi() {
    unsafe {
        outb(PIC1_COMMAND, PIC_EOI);
    }
}

pub unsafe fn pic2_eoi() {
    unsafe {
        outb(PIC2_COMMAND, PIC_EOI);
    }
}

pub unsafe fn pic_ack(int: u8) {
    if int < PIC1_INTERRUPT || int > PIC2_INTERRUPT {
        return;
    }
    unsafe {
        if int < PIC2_INTERRUPT {
            outb(PIC1, 0x20);
        } else {
            outb(PIC2, 0x20)
        }
    }
}

pub unsafe fn enable_irq(irq: u8) {
    unsafe {
        if irq < 8 {
            // Master PIC
            let mask = inb(PIC1_DATA);
            outb(PIC1_DATA, mask & !(1 << irq));
        } else if irq < 16 {
            // Slave PIC
            let mask = inb(PIC2_DATA);
            outb(PIC2_DATA, mask & !(1 << (irq - 8)));
        }
    }
}

pub unsafe fn disable_irq(irq: u8) {
    unsafe {
        if irq < 8 {
            let mask = inb(PIC1_DATA);
            outb(PIC1_DATA, mask | (1 << irq));
        } else if irq < 16 {
            let mask = inb(PIC2_DATA);
            outb(PIC2_DATA, mask | (1 << (irq - 8)));
        }
    }
}

pub unsafe fn init_timer(frequency: u32) {
    let divisor = 1193182 / frequency;
    unsafe {
        // 0x36: Channel 0, Square Wave Mode, Lobyte/Hibyte access
        outb(PIT_COMMAND, 0x36);

        // Split the divisor into low and high bytes
        outb(PIT_CHANNEL_0, (divisor & 0xFF) as u8);
        outb(PIT_CHANNEL_0, ((divisor >> 8) & 0xFF) as u8);
    }
}
