// kernel 不能依赖标准库实现
#![no_std]
// 禁用默认的入口点
#![no_main]

// 禁用标准库之后(link)，需要自己实现
// 1. panic_handler
// 2. exception_handler 语言项
// 3. start 语言项

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // TODO 程序panic时,进入死循环
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
