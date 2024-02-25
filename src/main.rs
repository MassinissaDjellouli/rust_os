#![no_std]
#![no_main]

mod vga_buf;

use core::panic::PanicInfo;
use crate::vga_buf::WRITER;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    WRITER.lock().write_str("Hello everynyanssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssseverynyanssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssseverynyanssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssseverynyanssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssseverynyanssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssseverynyanssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssseverynyansssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss:").unwrap();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}