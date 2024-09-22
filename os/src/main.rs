#![no_main]
#![no_std]

use core::arch::global_asm;
global_asm!(include_str!("asm/entry.asm"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    loop{}
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
