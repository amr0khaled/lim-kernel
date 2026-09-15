use core::fmt::{Arguments, Write};
use core::result::Result;

use crate::utils::asm::{inb, outb};
use spin::Mutex;

pub struct Writer {}
pub static WRITER: Mutex<Option<Writer>> = Mutex::new(None);
impl Write for Writer {
    fn write_str(&mut self, string: &str) -> Result<(), core::fmt::Error> {
        self.write_serial_str(string);
        Ok(())
    }
}
impl Writer {
    pub fn send_serial(&self, byte: u8) {
        unsafe {
            // Wait for the transmit buffer to be empty (Line Status Register bit 5)
            while (inb(0x3F8 + 5) & 0x20) == 0 {
                core::hint::spin_loop();
            }
            outb(0x3F8, byte);
        }
    }
    pub fn write_serial_str(&mut self, s: &str) {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.send_serial(b'\r');
            }
            self.send_serial(byte);
        }
    }
}

pub fn init() {
    let writer = Writer {};
    *WRITER.lock() = Some(writer);
    unsafe {
        // Enable Serial messaging for debugging
        outb(0x3F8 + 1, 0x00); // Disable interrupts
        outb(0x3F8 + 3, 0x80); // Enable DLAB (set baud rate divisor)
        outb(0x3F8 + 0, 0x03); // Set divisor to 3 (38400 baud) lo byte
        outb(0x3F8 + 1, 0x00); //                  hi byte
        outb(0x3F8 + 3, 0x03); // 8 bits, no parity, one stop bit
        outb(0x3F8 + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        outb(0x3F8 + 4, 0x0B); // IRQs enabled, RTS/DSR set
    }
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        $crate::serial::_print(format_args!($($args)*))
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! info {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("INFO: {}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! fault {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("FAULT: {}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! abort {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("ABORT: {}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! error {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("ERROR: {}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! debug {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("DEBUG: {}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    use core::fmt::Write;
    if let Some(writer) = WRITER.lock().as_mut() {
        writer.write_fmt(args).unwrap();
    }
}
