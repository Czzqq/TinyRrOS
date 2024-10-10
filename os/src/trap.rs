#![allow(dead_code)]

use core::arch::global_asm;
use crate::println;
use crate::timer::handler_timer_irq;
use crate::plic::handler_plic_irq;
use crate::read_csr;
use crate::write_csr;
#[cfg(feature = "with-symbol-table")]
use crate::backtrace::print_symbols;

global_asm!(include_str!("asm/entry.asm"));

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

struct FaultInfo {
    fn_ptr: fn(&PtRegs, &str) -> u32,
    name: &'static str,
}

fn show_regs(regs: &PtRegs) {
    println!("sepc: {:016x} ra: {:016x} sp : {:016x}", regs.sepc, regs.ra, regs.sp);
    println!(" gp : {:016x} tp: {:016x} t0 : {:016x}", regs.gp, regs.tp, regs.t0);
    println!(" t1 : {:016x} t2: {:016x} t3 : {:016x}", regs.t1, regs.t2, regs.s0);
    println!(" s1 : {:016x} a0: {:016x} a1 : {:016x}", regs.s1, regs.a0, regs.a1);
    println!(" a2 : {:016x} a3: {:016x} a4 : {:016x}", regs.a2, regs.a3, regs.a4);
    println!(" a5 : {:016x} a6: {:016x} a7 : {:016x}", regs.a5, regs.a6, regs.a7);
    println!(" s2 : {:016x} s3: {:016x} s4 : {:016x}", regs.s2, regs.s3, regs.s4);
    println!(" s5 : {:016x} s6: {:016x} s7 : {:016x}", regs.s5, regs.s6, regs.s7);
    println!(" s8 : {:016x} s9: {:016x} s10: {:016x}", regs.s8, regs.s9, regs.s10);
    println!(" s11: {:016x} t3: {:016x} t4 : {:016x}", regs.s11, regs.t3, regs.t4);
    println!(" t5 : {:016x} t6: {:016x}", regs.t5, regs.t6);
}

fn do_trap_error(regs: &PtRegs, str: &str) {
    println!("Oops - {}", str);
#[cfg(feature = "with-symbol-table")]
    print_symbols();
    show_regs(regs);
    println!("sstatus: {:016x} sbadaddr: {:016x} scause: {:016x}", regs.sstatus, regs.sbadaddr, regs.scause);
    panic!();
}

macro_rules! do_error_info {
    ($name:ident) => {
        fn $name(regs: &PtRegs, str: &str) -> u32 {
            do_trap_error(regs, str);
            0
        }
    }
}

do_error_info!(do_trap_unknown);
do_error_info!(do_trap_insn_misaligned);
do_error_info!(do_trap_insn_fault);
do_error_info!(do_trap_insn_illegal);
do_error_info!(do_trap_load_misaligned);
do_error_info!(do_trap_load_fault);
do_error_info!(do_trap_store_misaligned);
do_error_info!(do_trap_store_fault);
do_error_info!(do_trap_ecall_u);
do_error_info!(do_trap_ecall_s);
do_error_info!(do_trap_break);
do_error_info!(do_page_fault);

static FAULT_INFO: [FaultInfo; 16] = [
    FaultInfo { fn_ptr: do_trap_insn_misaligned, name: "Instruction address misaligned" },
    FaultInfo { fn_ptr: do_trap_insn_fault, name: "Instruction access fault" },
    FaultInfo { fn_ptr: do_trap_insn_illegal, name: "Illegal instruction" },
    FaultInfo { fn_ptr: do_trap_break, name: "Breakpoint" },
    FaultInfo { fn_ptr: do_trap_load_misaligned, name: "Load address misaligned" },
    FaultInfo { fn_ptr: do_trap_load_fault, name: "Load access fault" },
    FaultInfo { fn_ptr: do_trap_store_misaligned, name: "Store/AMO address misaligned" },
    FaultInfo { fn_ptr: do_trap_store_fault, name: "Store/AMO access fault" },
    FaultInfo { fn_ptr: do_trap_ecall_u, name: "Environment call from U-mode" },
    FaultInfo { fn_ptr: do_trap_ecall_s, name: "Environment call from S-mode" },
    FaultInfo { fn_ptr: do_trap_unknown, name: "unknown 10" },
    FaultInfo { fn_ptr: do_trap_unknown, name: "unknown 11" },
    FaultInfo { fn_ptr: do_page_fault, name: "Instruction page fault" },
    FaultInfo { fn_ptr: do_page_fault, name: "Load page fault" },
    FaultInfo { fn_ptr: do_trap_unknown, name: "unknown 14" },
    FaultInfo { fn_ptr: do_page_fault, name: "Store/AMO page fault" },
];

const SCAUSE_EC: usize = 0xf;
const SCAUSE_INT: usize = 0x1 << 63;

fn is_intterrupt_fault(scause: usize) -> bool {
    (scause & SCAUSE_INT) != 0
}

#[inline(always)]
fn ec_to_fault_info(scause: usize) -> &'static FaultInfo {
    println!("scause : {:x}", scause);
    let index = (scause as usize) & SCAUSE_EC;
    &FAULT_INFO[index]
}

const INTERRUPT_CAUSE_SOFTWARE: usize = 0x1;
const INTERRUPT_CAUSE_TIMER: usize = 0x5;
const INTERRUPT_CAUSE_EXTERNAL: usize = 0x9;

#[no_mangle]
fn do_exception(regs: &mut PtRegs, scause: usize) {
	//println!("do_exception scause:0x{:x}, sstatus=0x{:x}", scause, regs.sstatus);

    if is_intterrupt_fault(scause) {
        // Handle interrupt fault
        match scause & !SCAUSE_INT {
            INTERRUPT_CAUSE_TIMER => {
                handler_timer_irq();
            },
            INTERRUPT_CAUSE_EXTERNAL => {
                handler_plic_irq(regs);
                // Handle IRQ
            }
            INTERRUPT_CAUSE_SOFTWARE => {
                // Handle IPI
            }
            _ => {
                println!("unexpected interrupt cause");
                panic!();
            }
        }
    } else {
        let inf = ec_to_fault_info(scause);

        if (inf.fn_ptr)(regs, inf.name) == 0 {
            return;
        }
    }
}

extern "C" {
    fn do_exception_vector();
}
pub fn trap_init() {
    write_csr!(sscratch, 0);

	write_csr!(stvec, do_exception_vector);

    let stvec_val = read_csr!(stvec);
    println!("stevc=0x{:x}, 0x{:x}", stvec_val, do_exception_vector as usize);

	write_csr!(sie, -1);
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
