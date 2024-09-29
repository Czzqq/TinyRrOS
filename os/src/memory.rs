#![allow(dead_code)]
#![allow(unused_variables)]

use core::arch::asm;

unsafe fn memcpy_asm(dst: *mut u8, src: *const u8, size: usize) {
    let _tmp: usize;
    let end = src.add(size);

    unsafe {
        asm!(
            "1: ld {1}, 0({2})",
            "sd {1}, 0({0})",
            "addi {0}, {0}, 8",
            "addi {2}, {2}, 8",
            "blt {2}, {3}, 1b",
            /*
             * it's mean the dst variables inout,
             * don't care about the final value of `dst`
             */
            inout(reg) dst => _,
            lateout(reg) _tmp,
            inout(reg) src => _,
            in(reg) end,
        );
    }
    return
    //for i in 0..size {
    //    asm!(
    //        "lb t0, 0({src_ptr})", // Load byte from source 
    //        "sb t0, 0({dst_ptr})", // Store byte to destination 
    //        src_ptr = in(reg) src.add(i),
    //        dst_ptr = in(reg) dst.add(i),
    //        out("t0") _,
    //    );
    //}
}

pub fn memcpy(dst: *mut u8, src: *const u8, size: usize) {
    unsafe {
        memcpy_asm(dst, src, size);
    }

    return
}

unsafe fn memset_asm(src: *mut u8, val: u8, size: usize) {
    let end = src.add(size);
    asm!(
        "1: sd {1}, 0({0})",
        "addi {0}, {0}, 1",
        "blt {0}, {2}, 1b",
        inout(reg) src => _,
        in(reg) val,
        in(reg) end,
        );
}

pub fn memset(dst: *mut u8, val: u8, size: usize) {
    unsafe {
        memset_asm(dst, val, size);
    }

    return
}
