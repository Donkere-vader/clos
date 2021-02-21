#![no_main]
#![feature(asm)]
#![no_std]

use core::panic::PanicInfo;
mod vga_buffer;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::println("Hello world!\nThis is CLOS\n\nuser@clos: ~$");

    loop {};
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {};
}
