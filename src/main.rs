// kernel 不能依赖标准库实现
#![no_std]
// 禁用默认的入口点
#![no_main]
#![feature(llvm_asm)]

const SYSCALL_EXIT: usize = 93;

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

mod lang_items;

#[no_mangle]
pub extern "C" fn _start() {
    sys_exit(9);
}
