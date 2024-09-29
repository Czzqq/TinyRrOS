#include "include/sbi_offsets.h"

/*
 * M-mode exception vector should be align with 8 bytes
 */
.align 3
.global sbi_exception_vector
sbi_exception_vector:
    /* 
     * catch M-mode SP from mscratch,
     * load S-mode SP to mscratch reg
     */
    csrrw sp, mscratch, sp
    addi sp, sp, -(PT_SIZE)

	sd x1,  PT_RA(sp)
	sd x3,  PT_GP(sp)
	sd x5,  PT_T0(sp)
	sd x6,  PT_T1(sp)
	sd x7,  PT_T2(sp)
	sd x8,  PT_S0(sp)
	sd x9,  PT_S1(sp)
	sd x10, PT_A0(sp)
	sd x11, PT_A1(sp)
	sd x12, PT_A2(sp)
	sd x13, PT_A3(sp)
	sd x14, PT_A4(sp)
	sd x15, PT_A5(sp)
	sd x16, PT_A6(sp)
	sd x17, PT_A7(sp)
	sd x18, PT_S2(sp)
	sd x19, PT_S3(sp)
	sd x20, PT_S4(sp)
	sd x21, PT_S5(sp)
	sd x22, PT_S6(sp)
	sd x23, PT_S7(sp)
	sd x24, PT_S8(sp)
	sd x25, PT_S9(sp)
	sd x26, PT_S10(sp)
	sd x27, PT_S11(sp)
	sd x28, PT_T3(sp)
	sd x29, PT_T4(sp)
	sd x30, PT_T5(sp)
	sd x31, PT_T6(sp)

    /* save mepc */
	csrr t0, mepc
	sd t0, PT_MEPC(sp)

    /* save mstatus */
	csrr t0, mstatus
	sd t0, PT_MSTATUS(sp)

    /* 1. save m-mode sp to mscratch for next time exception,
     * 2. save s-mode sp to sbi_trap_regs->sp
     */
    addi t0, sp, PT_SIZE
	csrrw   t0, mscratch, t0
	sd t0, PT_SP(sp)

	mv a0, sp /* sbi_trap_regs */
	call sbi_trap_handler
