OUTPUT_ARCH(riscv)
ENTRY(_start)

TEXT_ROM = 0x80300000;

SECTIONS
{
    . =0x80200000;
    skernel = .; /* start of kernel */

    stext_boot = .; /* start of text.boot */
    .text.boot : {
        *(.text.boot)
    }
    etext_boot = .;

	. = ALIGN(4K);
    stext = .; /* start of text*/
    .text : AT(TEXT_ROM){
        *(.text .text.*)
    }

    . = ALIGN(4K);
    etext = .;
    srodata = .; /* start of rodata */
    .rodata : AT(ADDR(.rodata)){
        *(.rodata .rodata.*) 
        *(.srodata .srodata.*) 
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : { /* start of data */
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    .sbss = .;
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
