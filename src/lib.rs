#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

//for init routine
pub fn init() {
    //gdt mod init
    gdt::init();
    //interrupt mod init
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("running {} test case", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // the same as main, cause it indenpdent.
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
