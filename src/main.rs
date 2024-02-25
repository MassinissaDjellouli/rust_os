#![no_std]
#![no_main]

mod vga_buf;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buf::print();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}