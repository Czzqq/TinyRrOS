#![no_main]
#![no_std]

use core::arch::global_asm;
global_asm!(include_str!("asm/entry.asm"));

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

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    clear_bss();
    loop{}
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
