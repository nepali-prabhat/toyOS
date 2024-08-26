#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world");
    print!("{}", "- Prabhat");
    loop {}
}

// ! is never type.
// This function doesn't return anything so, its called a diverging function
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


