#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]){
    serial_println!("Running {} tests",tests.len());
    for test in tests{
        test.run();
    }
    exit_qemu(Success);
}

use core::panic::PanicInfo;
use crate::{serial_print, serial_println};
use crate::testing::QemuExitCode::Success;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
    where
        T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("{}","[ok]");
    }
}

#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> !{
    serial_println!("{}","[failed]");
    serial_println!("Error: {}",info);
    exit_qemu(QemuExitCode::Failed);
    loop{}
}
#[test_case]
fn testing_test_modules(){
    assert_eq!(1,1);
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