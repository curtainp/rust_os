OUTPUT_ARCH(riscv)

ENTRY(_start)

BASE_ADDRESS = 0x80200000;

SECTIONS
{
    /* . 表示当前地址 */
    . = BASE_ADDRESS;

    skernel = .;


    stext = .;
    .text : {
        /* entry函数放在text节开头 */
        *(.text.entry)
        *(.text .text.*)
    }


    . = ALIGN(4K);
    .etext = .;

    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
    }

    . = ALIGN(4K);
    .erodata = .;
    .sdata = .;
    .data : {
        *(.data .data.*)
    }

    . = ALIGN(4K);
    .edata = .;
    .sbss_with_stack = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    kernel_end = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}