/*
Rust编译器内部的某些功能(语言项)并不是硬编码在语言内部的，而是以一种可插入的库形式提供，库中为实现这些
功能的函数打上特定的标记，让编译器可以识别即可。这样即可以将特定功能解耦出来
*/

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