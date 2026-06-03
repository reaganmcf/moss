#[cfg(test)]
use core::panic::PanicInfo;

#[cfg(test)]
use crate::serial;

#[cfg(test)]
pub trait Testable {
    fn run(&self) -> ();
}

#[cfg(test)]
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) -> () {
        use crate::{serial_print, serial_println};
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
pub fn runner(tests: &[&dyn Testable]) {
    use crate::qemu::{QemuExitCode, exit_qemu};
    use crate::serial_println;

    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::qemu::{QemuExitCode, exit_qemu};
    use crate::serial_println;

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
