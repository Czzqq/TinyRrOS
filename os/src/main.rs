#![allow(dead_code)]
#![allow(unused)]
#![no_main]
#![no_std]

mod io;
mod console;
mod lang_item;
mod memory;
mod sbi;
mod trap;
mod timer;
mod plic;
mod mm;
#[cfg(feature = "with-symbol-table")]
mod backtrace;
#[cfg(feature = "with-symbol-table")]
mod symbols;
mod drivers {
    pub mod serial {
        pub mod uart16550;
    }
}

use core::arch::global_asm;
use drivers::serial::uart16550::{enable_uart_plic, uart_send_string};
use memory::*;
use sbi::*;
use timer::timer_init;
use timer::arch_local_irq_enable;
use plic::plic_init;
use trap::trap_init;
global_asm!(include_str!("asm/boot.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    //(sbss as usize..ebss as usize).for_each(|a|{
    //    unsafe { (a as *mut u8).write_volatile(0) }
    //});

    /* rewrite by for loop */
    let start = sbss as usize;
    let end = ebss as usize;
    for a in start..end {
        unsafe {
            (a as *mut u8).write_volatile(0);
        }
    }
}

fn display_mem() {
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
    let stext_boot: usize = stext_boot as usize;
    let etext_boot: usize = etext_boot as usize;
    let stext: usize = stext as usize;
    let etext: usize = etext as usize;
    let srodata: usize = srodata as usize;
    let erodata: usize = erodata as usize;
    let sdata: usize = sdata as usize;
    let edata: usize = edata as usize;
    let sbss: usize = sbss as usize;
    let ebss: usize = ebss as usize;
    let skernel: usize = skernel as usize;
    let ekernel: usize = ekernel as usize;

    println!("------- image mem space info -------");
    println!(".text.boot mem info : {:#x} - {:#x} ({:?} B)", stext_boot, etext_boot,
                                                        etext_boot - stext_boot);
    println!("     .text mem info : {:#x} - {:#x} ({:?} B)", stext, etext,
                                                        etext - stext);
    println!("   .rodata mem info : {:#x} - {:#x} ({:?} B)", srodata, erodata,
                                                        erodata - srodata);
    println!("     .data mem info : {:#x} - {:#x} ({:?} B)", sdata, edata,
                                                        edata - sdata);
    println!("      .bss mem info : {:#x} - {:#x} ({:?} B)", sbss, ebss,
                                                        ebss - sbss);
    println!("   .kernel mem info : {:#x} - {:#x} ({:?} B)", skernel, ekernel,
                                                        ekernel - skernel);
    println!("------- image mem space info over -------");
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {

    clear_bss();

    /* configure trap */
    trap_init();

    plic_init();
    enable_uart_plic();
    arch_local_irq_enable();

    uart_send_string("Hello, TinyRrOS!\r\n");

    println!("Hello, World!");
    display_mem();

    /*
     * case1: test in data section
     */
    //let src: *mut u8 = 0x8020b200 as *mut u8;
    //let dst: *mut u8 = 0x8020b200 as *mut u8;
    //let size: usize = 0x2;
    //let value: usize = 0x1234;

    let src_data = [1u8, 2, 3, 4];
    let mut dst_data = [0u8; 4];

    /*
     * case2: test memcpy and memset
     */
    memcpy(dst_data.as_mut_ptr(), src_data.as_ptr(), src_data.len());
    println!("after memcpy {:?}", dst_data); // Should print: [1, 2, 3, 4]
    memset(dst_data.as_mut_ptr(), 0xff, src_data.len());
    for i in 0..dst_data.len() {
        print!("{:02x} ", dst_data[i]);
    }
    println!("");

    /*
     * case 3: sbi call
     */
    sbi_putstring("This is sbi push string\n");

    /*
     * case 4: exception test
     */
    //test_exception();

    /* case 5: enable timer */
    timer_init();
	println!("sstatus:0x{:x}\n", read_csr!(sstatus));
    arch_local_irq_enable();
	println!("sstatus:0x{:x}, sie:0x{:x}\n", read_csr!(sstatus), read_csr!(sie));

    panic!("over, test machine panic!");
}
