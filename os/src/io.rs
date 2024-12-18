#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};

#[inline(always)]
pub fn dmb() {
    unsafe {
        core::arch::asm!("", options(nostack, preserves_flags, readonly));
    }
}

pub fn writel(value: u32, addr: usize) {
    unsafe {
        let ptr = addr as *mut u32;
        dmb();
        write_volatile(ptr, value);
    }
}


pub fn readl(addr: usize) -> u32 {
    unsafe {
        let ptr = addr as *const u32;
        let value = read_volatile(ptr);
        dmb();
        value
    }
}

pub fn writeb(value: u8, addr: usize) {
    unsafe {
        let ptr = addr as *mut u8;
        dmb();
        write_volatile(ptr, value);
    }
}

pub fn readb(addr: usize) -> u8 {
    unsafe {
        let ptr = addr as *const u8;
        let value = read_volatile(ptr);
        dmb();
        value
    }
}

pub fn readq(addr: usize) -> u64 {
    unsafe {
        let ptr = addr as *const u64;
        let value = read_volatile(ptr);
        dmb();
        value
    }
}

pub fn writeq(value: u64, addr: usize) {
    unsafe {
        let ptr = addr as *mut u64;
        dmb();
        write_volatile(ptr, value);
    }
}

#[macro_export]
macro_rules! write_csr {
    ($csr:ident, $val:expr) => {
        unsafe {
            core::arch::asm!(
                concat!("csrw ", stringify!($csr), ", {0}"),
                in(reg) $val,
                options(nostack, preserves_flags),
                );
        }
    };
}

#[macro_export]
macro_rules! read_csr {
    ($csr:ident) => {{
        let val: usize;
        unsafe {
            core::arch::asm!(
                concat!("csrr ", "{0}, ", stringify!($csr)),
                out(reg) val,
                options(nostack, preserves_flags),
                );
        }
        val
    }};
}

#[macro_export]
macro_rules! csr_set {
    ($csr:ident, $val:expr) => {{
        unsafe {
            core::arch::asm!(
                concat!("csrs ", stringify!($csr), ", {0}"),
                in(reg) $val,
                options(nostack, preserves_flags),
                );
        }
    }};
}

#[macro_export]
macro_rules! csr_clear {
    ($csr:ident, $val:expr) => {{
        unsafe {
            core::arch::asm!(
                concat!("csrc ", stringify!($csr), ", {0}"),
                in(reg) $val,
                options(nostack, preserves_flags),
                );
        }
    }};
}
