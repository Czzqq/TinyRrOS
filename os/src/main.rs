#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
