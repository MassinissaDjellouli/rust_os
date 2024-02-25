#![no_std]
#![no_main]

mod vga_buf;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    panic!("TESTEST");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{info}");
    loop{}
}