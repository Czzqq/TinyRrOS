.section ".text.boot"

.globl _start
_start:
    /* close s-mode interrupt */
    csrw sie, zero

    /* set stack */
    la sp, stacks_start
    li t0, 4096
    add sp, sp, t0

    li x3, 0x100 // just for test

    call kernel_main

_loop:
    wfi
    j _loop

.section .data
.align 12
.global stacks_start
stacks_start:
    .skip 4096


