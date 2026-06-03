#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello world");
    print!("This is ");
    print!("multiple calls");
    println!();

    println!("and this should be a new line");

    panic!("we're panicking");

    loop {}
}
