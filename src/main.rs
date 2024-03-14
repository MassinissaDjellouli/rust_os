#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
use rust_os::println;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(not(test))]{
        start();
    }
    #[cfg(test)]
    test_main();
    loop {}
}
#[cfg(not(test))]
fn start(){
    rust_os::init();
    println!("Hello World{}", "!");

    x86_64::instructions::interrupts::int3();

    println!("Still running!");
}
#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> !{
    println!("{info}");
    loop{}
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}