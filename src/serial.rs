use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

// Prints to host through the serial interface.
#[macro_export] // macro_export puts the macro under the root namespace
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*))
    }
}

// Prints to host through serial interface, appending a new line
#[macro_export] // macro_export puts the macro under the root namespace
macro_rules! serial_println {
    ()=> ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (
        $crate::serial_print!(concat!($fmt, "\n"), $($arg)*)
    );
}
