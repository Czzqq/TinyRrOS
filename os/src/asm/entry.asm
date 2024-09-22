.section ".text.boot"

/* text section */
.globl _start
_start:
    /* close s-mode interrupt */
    csrw sie, zero

    /* set stack */
    la sp, stacks_end_high_top

    li x0, 0x100 // just for test

    call kernel_main

_loop:
    wfi
    j _loop

/* data section */
.section .data
.globl stacks_start_low_bound
    .space 4096 * 16
    .globl stacks_end_high_top
stacks_end_high_top:

/* bss section */
;.section .bss.start
;.align 12
;.globl bss_start



