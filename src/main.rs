#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rust_os::println;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(not(test))]{
        start();
    }
    #[cfg(test)]{
        run_tests()
    }
    loop {}
}
#[cfg(not(test))]
fn start(){
    println!("Hello World{}", "!");
}
#[cfg(test)]
fn run_tests(){
    test_main();
}
#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> !{
    println!("{info}");
    loop{}
}

