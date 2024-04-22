    /*
     *  设置 SBI 的加载入口为 0x8000 0000
     * . 表示 LC Location Counter 位置计数器。
     */
    . = 0x80000000,

    .text.boot : { *(.text.boot) }
    .text : { *(text). }
    .rodata : { *(.rodata) }
    .data : { *(.data) }
    . = ALIGN(0x8)
    bss_begin = .;
    .bss = { *(.bss*) }
    bss_end = .;