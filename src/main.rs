// no_std attribute disable automatic include std
#![no_std]
// disable runtime entry
#![no_main]
// build ourself test frameworks
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;
mod vga_buffer;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("running {} test case", tests.len());
    for test in tests {
        test();
    }
}

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
    loop {}
}
