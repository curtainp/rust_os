// kernel 不能依赖标准库实现
#![no_std]
// 禁用默认的入口点
#![no_main]
#![feature(llvm_asm)]
mod lang_items;
mod syscall;

use core::fmt::{self, Write};
use crate::syscall::{sys_write, sys_exit};
struct  Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) ->fmt::Result {
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


#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello, world!");
    sys_exit(9);
}
