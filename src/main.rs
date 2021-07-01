#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

#[macro_use]
extern crate log;

#[macro_use]
mod console;
mod lang_item;
mod sbi;
mod logger;

global_asm!(include_str!("entry.asm"));
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a|{
        unsafe {
            (a as *mut u8).write_volatile(0)
        }
    })
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logger::init();
    error!("this is error info");
    warn!("this is warn info");
    info!("this is info info");
    debug!("this is debug info");
    trace!("this is trace info");
    println!("[kernel] hello world");
    panic!("it should shutdown!");
}
