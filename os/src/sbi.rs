#![allow(dead_code)]
use core::arch::asm;

const SBI_SET_TIMER: usize = 0x0;
const SBI_CONSOLE_PUTCHAR: usize = 0x1;
const SBI_CONSOLE_GETCHAR: usize = 0x2;

fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        asm!(
            "ecall",
            in("a0") arg0,
            in("a1") arg1,
            in("a2") arg2,
            in("a7") which,
            lateout("a0") ret,
            options(nostack),
            );
    }
    ret
}

#[inline(always)]
pub fn sbi_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn sbi_putstring(s: &str) {
    for c in s.chars() {
        sbi_call(SBI_CONSOLE_PUTCHAR, c as usize, 0, 0);
    }
}

pub fn sbi_set_timer(stime_val: usize) {
    sbi_call(SBI_SET_TIMER, stime_val, 0, 0);
}
