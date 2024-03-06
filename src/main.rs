#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rust_os::println;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(not(test))]{
        start();
    }
    loop {}
}
#[cfg(not(test))]
fn start(){
    println!("Hello World{}", "!");
}
#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> !{
    println!("{info}");
    loop{}
}

