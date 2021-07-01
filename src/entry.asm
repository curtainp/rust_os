    .section .text.entry
    .globl _start
# impl _start lang_item here, and main.rs _start no needed !!!
_start:
    la sp, boot_stack_top
    call rust_main

    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top:
