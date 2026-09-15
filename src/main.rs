#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod display;
mod internal;
mod serial;
pub mod utils;

use bootloader_api::{BootInfo, config::Mapping, entry_point};

const BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 128 * 1024;
    config
};

entry_point!(kernel_start, config = &BOOTLOADER_CONFIG);

#[unsafe(no_mangle)]
pub fn kernel_start(boot_info: &'static mut BootInfo) -> ! {
    serial::init();
    info!("Serial initialized");
    let framebuffer = boot_info.framebuffer.take();
    if framebuffer.is_none() {
        error!("Framebuffer is not found!");
        loop {
            hlt();
        }
    }
    info!("Founded Framebuffer!");
    internal::init();

    pen_init!(framebuffer.unwrap());
    for y in 0..512 {
        for x in 0..512 {
            draw!(x, y, Some(&Color::new(x as u8, y as u8, (x + y) as u8)));
        }
    }

    #[cfg(test)]
    test_main();

    loop {
        hlt();
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} Tests", tests.len());
    for test in tests {
        test()
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[cfg(test)]
use core::ops::Fn;
use core::panic::PanicInfo;

use crate::{display::colors::Color, utils::asm::hlt};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wcslen(ptr: *const u16) -> usize {
    let mut len = 0;
    unsafe {
        while *ptr.add(len) != 0 {
            len += 1;
        }
    }
    len
}
