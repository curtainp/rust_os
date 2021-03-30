// no_std attribute disable automatic include std
#![no_std]
// disable runtime entry
#![no_main]
// build ourself test frameworks
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
// change default test case entry to [test_main]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

// define self panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

// disable name mangling
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello Rust OS{}", "!");

    // init must before test case runtime
    blog_os::init();

    //trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u64) = 41;
    }

    //condition compile, this lines will only compile when cargo test
    #[cfg(test)]
    test_main();

    loop {}
}
