use crate::{println, symbols::SYMBOL_TABLE, trap::PtRegs};
use core::arch::asm;

#[repr(C)]
struct StackFrame {
    fp: usize,
    ra: usize,
}

extern "C" {
    fn stext_boot();
    fn etext_boot();
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss();
    fn ebss();
    fn skernel();
    fn ekernel();
}

fn print_symbols(addr: usize) {
    println!("[<{:16x}>] ", addr);
}

fn print_trace_address(pc: usize) -> bool {
    print_symbols(pc);
    false
}

fn get_frame_address() -> usize {
    let mut fp: usize;
    unsafe {
        asm!(
            "mv {}, s0",
            out(reg) fp
            );
    }
    fp
}

fn is_kernel_text(addr: usize) -> bool {
    let stext: usize = stext as usize;
    let etext: usize = etext as usize;

    if addr >= stext && addr < etext {
        true;
    }
    false
}

fn walk_stackframe<F>(regs: Option<&PtRegs>, mut callback: F)
where
    F: FnMut(usize) -> bool,
{
    // let (mut pc, mut sp, mut fp) = if let Some(regs) = regs {
    //     (regs.sepc, regs.sp, regs.s0)
    // } else {
    //     unsafe {
    //         let mut current_sp: usize;
    //         asm!("mv {}, sp", out(reg) current_sp);
    //         (walk_stackframe as usize, current_sp, get_frame_address())
    //     }
    // };
    //
    // loop {
    //     if !is_kernel_text(pc) || callback(pc) {
    //         break;
    //     }
    //
    //     let low = sp + core::mem::size_of::<StackFrame>();
    //     if fp < low || fp & 0x7 != 0 {
    //         break;
    //     }
    //
    //     let frame = unsafe { &*(fp as *const StackFrame).offset(-1) };
    //     sp = fp;
    //     fp = frame.fp;
    //     pc = frame.ra.wrapping_sub(4);
    // }
}

pub fn backtrace(regs: &PtRegs) {
    println!("Call Trace:");
    // walk_stackframe(regs, print_trace_address)
}
