#![allow(dead_code)]
const SIE_STIE: usize = 0x20;
const CLINT_TIMEBASE_FREQ: usize = 10000000;
const HZ: usize = 1000;

const VIRT_CLINT_ADDR: usize = 0x2000000;
const VIRT_CLINT_TIMER_CMP: usize = VIRT_CLINT_ADDR + 0x4000;
const VIRT_CLINT_TIMER_VAL:usize = VIRT_CLINT_ADDR + 0xbff8;
use core::ptr::{read_volatile, write_volatile};

// Align the struct to a cache line (typically 64 bytes)
#[repr(C, align(64))]
struct CacheAlignedU64(u64);

static mut JIFFIES: CacheAlignedU64 = CacheAlignedU64(0);

/// Increments the `jiffies` counter using volatile operations.
unsafe fn increment_jiffies() {
    // Obtain a pointer to the `u64` within `JIFFIES`
    let ptr = &mut JIFFIES.0 as *mut u64;
    // Read the current value using a volatile read
    let current = read_volatile(ptr);
    // Increment the value safely (wrapping in case of overflow)
    let new_value = current.wrapping_add(1);
    // Write the new value back using a volatile write
    write_volatile(ptr, new_value);
}

/// Reads the current value of `jiffies` using a volatile read.
unsafe fn get_jiffies() -> u64 {
    let ptr = &JIFFIES.0 as *const u64;
    read_volatile(ptr)
}

/*
 * NOTE: the func is use the sbi not configure pmp 0x0 - ~0x0
 *      !!!! only can use one !!!!
 */
//use core::arch::asm;
//#[inline(always)]
//fn get_cycles() -> usize {
//    let val: usize;
//    unsafe {
//        asm!(
//            "rdtime {0}",
//            out(reg) val,
//        );
//    }
//    val
//}

/*
 * NOTE: the func is use the custom sbi that configure pmp 0x0 - ~0x0
 */
use crate::io::*;
#[inline(always)]
fn get_cycles() -> usize {
    let value = readq(VIRT_CLINT_TIMER_VAL);
    value as usize
}

use crate::csr_set;
use crate::sbi::sbi_set_timer;
fn reset_timer() {
    let val = get_cycles() + CLINT_TIMEBASE_FREQ / HZ;
    sbi_set_timer(val);
    csr_set!(sie, SIE_STIE);
}

pub fn timer_init() {
    reset_timer();
}

use crate::csr_clear;
use crate::println;
pub fn handler_timer_irq() {
    csr_clear!(sie, SIE_STIE);
    reset_timer();
    unsafe {
        increment_jiffies();
        let jiffies_value = get_jiffies();
        println!("Core0 Timer interrupt received, jiffies={}", jiffies_value);
    }
}

const SR_SIE: usize = 0x2; /* Supervisor Interrupt Enable */
pub fn arch_local_irq_enable() {
    csr_set!(sstatus, SR_SIE);

}

pub fn arch_local_irq_disable() {
    csr_clear!(sstatus, SR_SIE);
}

