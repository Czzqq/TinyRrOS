#![allow(dead_code)]
const SIE_STIE: usize = 0x20;
const CLINT_TIMEBASE_FREQ: usize = 10000000;
const HZ: usize = 1000;

const VIRT_CLINT_ADDR: usize = 0x2000000;
const VIRT_CLINT_TIMER_CMP: usize = VIRT_CLINT_ADDR + 0x4000;
const VIRT_CLINT_TIMER_VAL:usize = VIRT_CLINT_ADDR + 0xbff8;

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
