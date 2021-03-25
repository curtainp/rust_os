// no_std attribute disable automatic include std
#![no_std]
// disable runtime entry
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

// define self panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

// disable name mangling
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello Rust OS{}", "!");
    panic!("Some panic message");
    loop {}
}
