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
    fn ekernel();
    fn skernel();
}

pub const SYMBOL_TABLE_SIZE: usize = SYMBOL_TABLE.len();

//fn lookup_symbol(address: usize) -> Option<&'static str> {
fn lookup_symbol(address: usize) {
    let mut low: usize = 0;
    let mut high: usize = SYMBOL_TABLE_SIZE;
    let mut mid: usize;

    while high - low > 1 {
        mid = (high + low) / 2;
        if SYMBOL_TABLE[mid].address <= address {
            low = mid;
        } else {
            high = mid;
        }
    }
    let symbol = &SYMBOL_TABLE[low];
    let (target, name) = (symbol.address.clone(), symbol.name);
    let offset: usize = address - target;

    println!("<{}>+{:#016x}/{:#04x}", name, target, offset);
}

fn print_symbols(addr: usize) {
    lookup_symbol(addr);
}

fn print_trace_address(pc: u64) -> bool {
    print_symbols(pc as usize);
    false
}

fn is_kernel_text(addr: u64) -> bool {
    let stext: u64 = stext as u64;
    let etext: u64 = etext as u64;

    if addr >= stext && addr <= etext {
        true
    } else {
        false
    }
}

#[inline(always)]
unsafe fn __builtin_frame_address(level: u32) -> *const u8 {
    let addr: *const u8;
    if level == 0 {
        asm!("mv {}, s0", out(reg) addr);
    } else {
        addr = core::ptr::null();
    }
    addr
}

type StackFrameCallback = fn(u64) -> bool;

fn walk_stackframe<F>(regs: Option<&PtRegs>, mut callback: F)
where
    F: FnMut(u64) -> bool,
{
    let (pc, sp, fp): (u64, u64, u64);
    if let Some(regs) = regs {
        pc = regs.sepc;
        sp = regs.sp;
        fp = regs.s0;
    } else {
        unsafe {
            asm!("mv {}, sp", out(reg) sp);
            pc = walk_stackframe::<F> as u64;
            fp = __builtin_frame_address(0) as u64;
        }
    };

    walk_stackframe_inner(pc, sp, fp, &mut callback);
}

fn walk_stackframe_inner<F>(mut pc: u64, mut sp: u64, mut fp: u64, callback: &mut F)
where
    F: FnMut(u64) -> bool,
{
    let skernel: u64 = skernel as u64;
    let ekernel: u64 = ekernel as u64;

    loop {
        if !is_kernel_text(pc) || callback(pc) {
            break;
        }

        let low = sp + core::mem::size_of::<StackFrame>() as u64;
        if fp < low || fp & 0x7 != 0 {
            break;
        }

        unsafe {
            let frame = (fp as *const StackFrame).sub(1);
            if (frame as u64) > ekernel || (frame as u64) < skernel {
                break;
            }

            sp = fp;
            fp = (*frame).fp;
            pc = (*frame).ra.wrapping_sub(4);

        }
    }
}

pub fn backtrace(regs: &PtRegs) {
    println!("Call Trace:");
    walk_stackframe(Some(regs), print_trace_address)
}
