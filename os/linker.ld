OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS
{
    . =0x80200000;
    skernel = .; /* start of kernel */

    .stext = .; /* start of text :0x80200000 */
    .text : { 
        *(.text.boot)
        *(.text .text.*) 
    }

    . = ALIGN(4K);
    etext = .;
    srodata = .; /* start of rodata */
    .rodata : { 
        *(.rodata .rodata.*) 
        *(.srodata .srodata.*) 
    }

    . = ALIGN(4K);
    erodata = .;
    .sdata : { /* start of data */
        *(.data .data.*) 
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    .bss : { /* start of bss */
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
