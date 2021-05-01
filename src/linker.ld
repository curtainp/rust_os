OUTPUT_ARCH(riscv)

ENTRY(_start)

BASE_ADDRESS = 0x80200000;

SECTIONS
{
    /* . 表示当前地址 */
    . = BASE_ADDRESS;

    kernel_start = .;

    /* .text */
    text_start = .;
    .text : {
        /* entry函数放在text节开头 */
        *(.text.entry)
        *(.text.text.*)
    }

    /* .rodata */
    rodata_start = .;
    .rodata : {
        *(.rodata.rodata.*)
    }

    /* .data */
    .data_start = .;
    .data : {
        *(.data.data.*)
    }

    /* .bss */
    .bss_start = .;
    .bss : {
        *(.sbss .bss.bss.*)
    }

    kernel_end = .;
}