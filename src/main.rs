// kernel 不能依赖标准库实现
#![no_std]
// 禁用默认的入口点
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
use core::fmt::{self, Write};
use crate::sbi::sbi_call;
use core::panic::PanicInfo;
struct Stdout;

mod sbi;

// const SYSCALL_EXIT: usize = 93;
// const SYSCALL_WRITE: usize = 64;
const SBI_SHUTDOWN: usize = 8;
const SBI_CONSOLE_PUTCHAR: usize = 1;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]), "{x17}" (id)
            : "memory"
            : "volatile"
        );
    }
    ret
}

pub fn console_putchar(c: usize) {
    syscall(SBI_CONSOLE_PUTCHAR, [c, 0, 0]);
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("it should shutdown!");
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        //sys_write(1, s.as_bytes());
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

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

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("panic at {}:{} {}", location.file(), location.line(), info.message().unwrap());
    } else {
        println!("panic: {}", info.message().unwrap());
    }
    shutdown();
}

global_asm!(include_str!("entry.asm"));
#[no_mangle]
pub fn rust_main() -> ! {
    println!("hello world");
    panic!("it should shutdown!");
    //shutdown();
}
