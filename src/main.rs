// no_std attribute disable automatic include std
#![no_std]
// disable runtime entry
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

// define self panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// disable name mangling
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER
        .lock()
        .write_str("Hello Rust OS!")
        .unwrap();
    write!(vga_buffer::WRITER.lock(), ", some number: {} {}", 44, 1.334).unwrap();
    loop {}
}
