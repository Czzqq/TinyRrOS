.section ".text.boot"

/* text section */
.globl _start
_start:
    /* close s-mode interrupt */
    csrw sie, zero

    /* set stack */
    la sp, stacks_top

    // just for test
    li x3, 0x100 

    call kernel_main

_loop:
    wfi
    j _loop

/* data section */
.section .data
.globl stacks_lower_bound
stacks_lower_bound:
    .space 4096 * 16
    .globl stacks_top
stacks_bottom:
stacks_top: /* high address -> stack bottom == stack top in init */
.globl stacks_higher_bound
stacks_higher_bound:

/* bss section */
;.section .bss.start
;.align 12
;.globl bss_start



