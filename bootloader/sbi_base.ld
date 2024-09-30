    /*
     *  set SBI application load address: 0x80000000
     *  . LC: Location Counter
     */
    . = 0x80000000,

    .text.boot : { *(.text.boot) }
    .text : { *(.text) }
    .rodata : { *(.rodata) }
    .data : { *(.data) }
    . = ALIGN(0x8);
    bss_begin = .;
    .bss : { *(.bss*) }
    bss_end = .;
