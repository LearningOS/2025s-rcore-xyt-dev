//! The panic handler

use crate::sbi::shutdown;
use core::panic::PanicInfo;

use core::arch::global_asm;

global_asm!(include_str!("stackup.S"));
extern "C" {
    fn __stackup();
}

#[panic_handler]
/// panic handler
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("[kernel] Panicked: {}", info.message().unwrap());
    }
    // unsafe {__stackup();}
    shutdown()
}
