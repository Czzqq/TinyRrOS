#![allow(dead_code)]
#![allow(unused_variables)]
use crate::println;
use crate::symbols::SYMBOL_TABLE;
use crate::trap::PtRegs;
use core::arch::asm;

#[repr(C)]
struct StackFrame {
    fp: u64,
    ra: u64,
}

extern "C" {
    fn stext();
    fn etext();
}

pub const SYMBOL_TABLE_SIZE: usize = SYMBOL_TABLE.len();

fn lookup_symbol(address: usize) -> Option<&'static str> {
    let mut low: usize = 0;
    let mut high: usize = SYMBOL_TABLE_SIZE;
    let mut mid: usize;

    println!("lookup_symbol, pc: {:016x}", address);

    while high - low > 1 {
        mid = (high + low) / 2;
        println!("SYMBOL_TABLE[mid].address: %{:016x}", SYMBOL_TABLE[mid].address);
        if SYMBOL_TABLE[mid].address <= address {
            low = mid;
        } else {
            high = mid;
        }
    }

    println!("found symbol index: {:?}", low);
    println!("address: {:016x}, name: {:?}", SYMBOL_TABLE[low].address, SYMBOL_TABLE[low].name);


    SYMBOL_TABLE.iter()
        .find(|symbol| symbol.address == SYMBOL_TABLE[low].address)
        .map(|symbol| symbol.name)
}

fn print_symbols(addr: usize) {
    println!("[<{:16x}>] ",addr);
    println!("{:?}", lookup_symbol(addr));
}

fn print_trace_address(pc: u64) -> bool {
    println!("print_trace_address, pc: {:016x}", pc);
    print_symbols(pc as usize);
    false
}

fn is_kernel_text(addr: u64) -> bool {
    let stext: u64 = stext as u64;
    let etext: u64 = etext as u64;

    if addr >= stext && addr <= etext {
        println!("is kernel text, stext: {:16x}, etext: {:16x}, addr: {:16x}", stext, etext, addr);
        true
    } else {
        println!("is not kernel text, stext: {:16x}, etext: {:16x}, addr: {:16x}", stext, etext, addr);
        false
    }
}

type StackFrameCallback = fn(u64) -> bool;

fn walk_stackframe<F>(regs: Option<&PtRegs>, mut callback: F)
where
    F: FnMut(u64) -> bool,
{
    let (pc, sp, fp) = if let Some(regs) = regs {
        println!("regs.sepc: {:016x}, regs.sp: {:016x}, regs.s0: {:016x}", regs.sepc, regs.sp, regs.s0);
        (regs.sepc, regs.sp, regs.s0)
    } else {
        unsafe {
            let mut current_pc: u64;
            let mut current_sp: u64;
            let mut current_fp: u64;
            asm!(
                "auipc {}, 0",
                "mv {}, sp",
                "mv {}, s0",
                out(reg) current_pc,
                out(reg) current_sp,
                out(reg) current_fp 
            );
            (current_pc, current_sp, current_fp)
        }
    };

    walk_stackframe_inner(pc, sp, fp, &mut callback);
}

fn walk_stackframe_inner<F>(mut pc: u64, mut sp: u64, mut fp: u64, callback: &mut F)
where
    F: FnMut(u64) -> bool,
{
    println!("pc: {:016x}, sp: {:016x}, fp: {:016x}", pc, sp, fp);
    loop {
        if !is_kernel_text(pc) || callback(pc) {
            break;
        }

        let low = sp + core::mem::size_of::<StackFrame>() as u64;
        if fp < low || fp & 0x7 != 0 {
            break;
        }

        let frame = unsafe { &*(fp as *const StackFrame).offset(-1) };
        sp = fp;
        fp = frame.fp;
        pc = frame.ra.wrapping_sub(4);
    }
}

pub fn backtrace(regs: &PtRegs) {
    println!("Call Trace:");
    walk_stackframe(Some(regs), print_trace_address)
}
