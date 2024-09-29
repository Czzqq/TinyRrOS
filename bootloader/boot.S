.section ".text.boot"

.global _start
_start:
    # close all m-mode ints
    csrw mie, zero

    # set sp, size is 4096
    la sp, stack_top
    li t0, 4096
    add sp, sp, t0

	csrw mscratch, sp

    tail sbi_start

# data section
.section .data
.align 12
.global stack_top
stack_top:
    .skip 4096
