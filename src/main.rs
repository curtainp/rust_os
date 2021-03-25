// no_std attribute disable automatic include std
#![no_std]
// disable runtime entry
#![no_main]
// build ourself test frameworks
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// change default test case entry to [test_main]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod vga_buffer;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("running {} test case", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial_assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
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

    //condition compile, this lines will only compile when cargo test
    #[cfg(test)]
    test_main();

    loop {}
}
