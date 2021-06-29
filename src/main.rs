// kernel 不能依赖标准库实现
#![no_std]
// 禁用默认的入口点
#![no_main]

mod lang_items;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
