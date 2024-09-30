.section ".text.boot"

/* text section */
.globl _start
_start:
    /* load text section from ROM to RAM */
    la t0, TEXT_ROM
    la t1, stext
    la t2, etext
.L0:
    ld a5, (t0)
    sd a5, (t1)
    addi t1, t1, 8
    addi t0, t0, 8
    bltu t1, t2, .L0

/*
 * Just for test AT spectial in linker.ld
 */
/*
    la t0, 0x80307000
    la t1, srodata
    la t2, erodata
.L1:
    ld a5, (t0)
    sd a5, (t1)
    addi t1, t1, 8
    addi t0, t0, 8
    bltu t1, t2, .L1
*/

    /* disable s-mode interrupt */
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



