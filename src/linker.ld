OUTPUT_ARCH(riscv)

ENTRY(_start)

BASE_ADDRESS = 0x80200000;

SECTIONS
{
    /* . 表示当前地址 */
    . = BASE_ADDRESS;

    skernel = .;

    /* .text */
    text_start = .;
    .text : {
        /* entry函数放在text节开头 */
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);
    /* .rodata */
    rodata_start = .;
    .rodata : {
        *(.rodata .rodata.*)
    }

    . = ALIGN(4K);
    /* .data */
    .data_start = .;
    .data : {
        *(.data .data.*)
    }

    . = ALIGN(4K);
    /* .bss */
    .bss_start = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
    }

    . = ALIGN(4K);
    kernel_end = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}