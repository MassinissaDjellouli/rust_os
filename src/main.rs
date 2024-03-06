#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]

mod vga_buf;
mod serial;
mod testing;

use core::panic::PanicInfo;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();
    loop {}
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> !{
    println!("{info}");
    loop{}
}

