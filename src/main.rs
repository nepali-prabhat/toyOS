#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::test_runner)]

mod vga_buffer;

mod serial;

use core::panic::PanicInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    // translation fn: (a << 1) | 1
    // Can't use 0 because it'd be translated to 1 and clash
    // with default QEMU error
    Success = 0x10, // Translated to 33 [by (0x10 << 1 | 1)]. 0x10 = 16: 0001_0000
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32)
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world");
    println!("{}", "- Prabhat");

    #[cfg(test)]
    test_main();

    loop {}
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T where T: Fn() {
    fn run(&self) {
        serial_print!("{} ... \t", core::any::type_name::<T>());
        self();
        serial_println!("[OK]");
    }
}

#[test_case]
fn test_assertion() {
    assert_eq!(1, 1);
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

// ! is never type.
// This function doesn't return anything so, its called a diverging function
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[Failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {} // need this because compiler doesn't know exit_qemu causes the program to exit
}
