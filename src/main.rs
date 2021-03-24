// no_std attribute disable automatic include std
#![no_std]
// disable runtime entry
#![no_main]

use core::panic::PanicInfo;

// define self panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

// disable name mangling
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            // 0xb set colors
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
