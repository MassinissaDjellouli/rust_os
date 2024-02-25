#![no_std]
#![no_main]
use core::panic::PanicInfo;

static OUTPUT:&[u8] = b"Hello world!";
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buf = 0xb8000 as *mut u8;

    for (i, &byte) in OUTPUT.iter().enumerate() {
        unsafe {
            *vga_buf.offset(i as isize * 2) = byte;
            *vga_buf.offset(i as isize * 2 + 1) = 0xb8;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}