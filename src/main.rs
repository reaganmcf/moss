#![feature(custom_test_frameworks)]
#![test_runner(moss::test::runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]

use moss::*;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello world");
    print!("This is ");
    print!("multiple calls");
    println!();

    println!("and this should be a new line");

    #[cfg(test)]
    test_main();

    loop {}
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
