#![allow(dead_code)]

use core::arch::global_asm;
global_asm!(include_str!("asm/entry.asm"));

use core::arch::asm;
use core::mem;
pub fn save_s_context() {
    unsafe {
        asm!(
            "addi sp, sp, -{pt_size}",

            "sd x1, 8(sp)",
            "sd x3, 24(sp)",
            "sd x5, 40(sp)",
            "sd x6, 48(sp)",
            "sd x7, 56(sp)",
            "sd x8, 64(sp)",
            "sd x9 72(sp)",
            "sd x10, 80(sp)",
            "sd x11, 88(sp)",
            "sd x12, 96(sp)",
            "sd x13, 104(sp)",
            "sd x14, 112(sp)",
            "sd x15, 120(sp)",
            "sd x16, 128(sp)",
            "sd x17, 136(sp)",
            "sd x18, 144(sp)",
            "sd x19, 152(sp)",
            "sd x20, 160(sp)",
            "sd x21, 168(sp)",
            "sd x22, 176(sp)",
            "sd x23, 184(sp)",
            "sd x24, 192(sp)",
            "sd x25, 200(sp)",
            "sd x26, 208(sp)",
            "sd x27, 216(sp)",
            "sd x28, 224(sp)",
            "sd x29, 232(sp)",
            "sd x30, 240(sp)",
            "sd x31, 248(sp)",

            /* save s-mode status */
            "csrr s1, sstatus",
            "sd, s1, 256(sp)",
            /* save s-mode sepc */
            "csrr s2, sepc",
            "sd, s2, 0(sp)",
            /* save s-mode sbadaddr */
            "csrr s3, sbadaddr",
            "sd, s3, 264(sp)",
            /* save s-mode scause */
            "csrr s4, scause",
            "sd, s4, 272(sp)",
            /* save s-mode sscratch*/
            "csrr s5, sscratch",
            "sd, s5, 32(sp)",
            /* save s-mode sp */
            "addi s0, {pt_size}",
            "sd, s0, 32(sp)",

            pt_size = const mem::size_of::<PtRegs>(),
        );
    }
}

pub fn recover_s_context() {
    unsafe {
        asm!(
            "ld a0, 256(sp)",
            "csrw sstatus, a0",

            "ld a2, 0(sp)",
            "csrw sepc, a2",

            "ld x1, 8(sp)",
            "ld x3, 24(sp)",
            "ld x5, 40(sp)",
            "ld x6, 48(sp)",
            "ld x7, 56(sp)",
            "ld x8, 64(sp)",
            "ld x9, 72(sp)",
            "ld x10, 80(sp)",
            "ld x11, 88(sp)",
            "ld x12, 96(sp)",
            "ld x13, 104(sp)",
            "ld x14, 112(sp)",
            "ld x15, 120(sp)",
            "ld x16, 128(sp)",
            "ld x17, 136(sp)",
            "ld x18, 144(sp)",
            "ld x19, 152(sp)",
            "ld x20, 160(sp)",
            "ld x21, 168(sp)",
            "ld x22, 176(sp)",
            "ld x23, 184(sp)",
            "ld x24, 192(sp)",
            "ld x25, 200(sp)",
            "ld x26, 208(sp)",
            "ld x27, 216(sp)",
            "ld x28, 224(sp)",
            "ld x29, 232(sp)",
            "ld x30, 240(sp)",
            "ld x31, 248(sp)",

            "ld, x0, 16(sp)",
        );
    }
}

#[repr(C)]
pub struct PtRegs {
    pub sepc: u64,
    pub ra: u64,
    pub sp: u64,
    pub gp: u64,
    pub tp: u64,
    pub t0: u64,
    pub t1: u64,
    pub t2: u64,
    pub s0: u64,
    pub s1: u64,
    pub a0: u64,
    pub a1: u64,
    pub a2: u64,
    pub a3: u64,
    pub a4: u64,
    pub a5: u64,
    pub a6: u64,
    pub a7: u64,
    pub s2: u64,
    pub s3: u64,
    pub s4: u64,
    pub s5: u64,
    pub s6: u64,
    pub s7: u64,
    pub s8: u64,
    pub s9: u64,
    pub s10: u64,
    pub s11: u64,
    pub t3: u64,
    pub t4: u64,
    pub t5: u64,
    pub t6: u64,
    /* Superivsor CSRs */
    pub sstatus: u64,
    pub sbadaddr: u64,
    pub scause: u64,
    /* a0 value before syscall */
    pub orig_a0: u64,
}

/* struct PtRegs members offset */
const PT_SIZE: usize = 288; /* sizeof(struct pt_regs) */
const PT_SEPC: usize = 0; /* offsetof(struct pt_regs, sepc) */
const PT_RA: usize =  8; /* offsetof(struct pt_regs, ra) */
const PT_FP: usize =  64; /* offsetof(struct pt_regs, s0) */
const PT_S0: usize =  64; /* offsetof(struct pt_regs, s0) */
const PT_S1: usize =  72; /* offsetof(struct pt_regs, s1) */
const PT_S2: usize =  144; /* offsetof(struct pt_regs, s2) */
const PT_S3: usize =  152; /* offsetof(struct pt_regs, s3) */
const PT_S4: usize =  160; /* offsetof(struct pt_regs, s4) */
const PT_S5: usize =  168; /* offsetof(struct pt_regs, s5) */
const PT_S6: usize =  176; /* offsetof(struct pt_regs, s6) */
const PT_S7: usize =  184; /* offsetof(struct pt_regs, s7) */
const PT_S8: usize =  192; /* offsetof(struct pt_regs, s8) */
const PT_S9: usize =  200; /* offsetof(struct pt_regs, s9) */
const PT_S10: usize = 208; /* offsetof(struct pt_regs, s10) */
const PT_S11: usize = 216; /* offsetof(struct pt_regs, s11) */
const PT_SP: usize = 16 ;/* offsetof(struct pt_regs, sp) */
const PT_TP: usize = 32 ;/* offsetof(struct pt_regs, tp) */
const PT_A0: usize = 80 ;/* offsetof(struct pt_regs, a0) */
const PT_A1: usize = 88 ;/* offsetof(struct pt_regs, a1) */
const PT_A2: usize = 96 ;/* offsetof(struct pt_regs, a2) */
const PT_A3: usize = 104; /* offsetof(struct pt_regs, a3) */
const PT_A4: usize = 112; /* offsetof(struct pt_regs, a4) */
const PT_A5: usize = 120; /* offsetof(struct pt_regs, a5) */
const PT_A6: usize = 128; /* offsetof(struct pt_regs, a6) */
const PT_A7: usize = 136; /* offsetof(struct pt_regs, a7) */
const PT_T0: usize = 40 ;/* offsetof(struct pt_regs, t0) */
const PT_T1: usize = 48 ;/* offsetof(struct pt_regs, t1) */
const PT_T2: usize = 56 ;/* offsetof(struct pt_regs, t2) */
const PT_T3: usize = 224; /* offsetof(struct pt_regs, t3) */
const PT_T4: usize = 232; /* offsetof(struct pt_regs, t4) */
const PT_T5: usize = 240; /* offsetof(struct pt_regs, t5) */
const PT_T6: usize = 248; /* offsetof(struct pt_regs, t6) */
const PT_GP: usize = 24 ;/* offsetof(struct pt_regs, gp) */
const PT_ORIG_A0: usize = 280; /* offsetof(struct pt_regs, orig_a0) */
const PT_SSTATUS: usize = 256; /* offsetof(struct pt_regs, sstatus) */
const PT_SBADADDR: usize = 264; /* offsetof(struct pt_regs, sbadaddr) */
const PT_SCAUSE: usize = 272; /* offsetof(struct pt_regs, scause) */
