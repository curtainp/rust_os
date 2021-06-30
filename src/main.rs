// kernel 不能依赖标准库实现
#![no_std]
// 禁用默认的入口点
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
use core::fmt::{self, Write};
use crate::sbi::sbi_call;
struct Stdout;

mod lang_items;
mod sbi;

const SYSCALL_EXIT: usize = 93;
const SYSCALL_WRITE: usize = 64;
const SBI_SHUTDOWN: usize = 8;

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
pub fn sys_exit(estate: i32) -> isize {
    syscall(SYSCALL_EXIT, [estate as usize, 0, 0])
}
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(1, s.as_bytes());
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

// #[no_mangle]
// pub extern "C" fn _start() {
//     println!("hello world");
//     shutdown();
//     //sys_exit(9);
// }

global_asm!(include_str!("entry.asm"));
#[no_mangle]
pub fn rust_main() -> ! {
    shutdown();
}
