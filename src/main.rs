#![feature(custom_test_frameworks)]
#![test_runner(moss::test::runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]

use bootloader::{BootInfo, entry_point};
use moss::println;

use core::panic::PanicInfo;

entry_point!(kernel_main);

#[unsafe(no_mangle)]
pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello world");

    moss::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    moss::hlt_loop();
}

/// This function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    moss::test::panic_handler(info);
}
