use core::panic::PanicInfo;
use crate::println;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(location) = _info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(), location.line(),
            _info.message()
            );
    }
    loop{}
}
