#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(toy_os::test_runner)] 

use core::panic::PanicInfo;
use toy_os::println;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world");
    println!("{}", "- Prabhat");

    #[cfg(test)]
    test_main();

    loop {}
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
    toy_os::test_panic_handler(info);
}
