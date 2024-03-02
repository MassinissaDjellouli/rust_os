#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]){
    serial_println!("Running {} tests",tests.len());
    for test in tests{
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

mod vga_buf;
mod serial;

use core::panic::PanicInfo;




#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();
     loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{info}");
    loop{}
}

#[test_case]
fn trivial_assertion(){
    print!("trivial assertion... ");
    assert_eq!(1,1);
    println!("[ok]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode{
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode){
    use x86_64::instructions::port::Port;
    unsafe{
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}