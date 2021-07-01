#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_item;
mod sbi;

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
    println!("[kernel] hello world");
    panic!("it should shutdown!");
}
