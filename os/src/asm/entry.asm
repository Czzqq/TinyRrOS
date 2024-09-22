.section ".text.boot"

/* text section */
.globl _start
_start:
    /* close s-mode interrupt */
    csrw sie, zero

    /* set stack */
    ;la sp, stacks_top
    la sp, stacks_bottom

    // just for test
    li x3, 0x100 

    call kernel_main

_loop:
    wfi
    j _loop

/* data section */
.section .data
.globl stacks_top
stacks_bottom:
    .space 4096 * 16
    .globl stacks_bottom /* high address -> stack bottom */
stacks_top:

/* bss section */
;.section .bss.start
;.align 12
;.globl bss_start



