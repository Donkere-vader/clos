#![no_main]
#![feature(asm)]
#![no_std]

use core::panic::PanicInfo;
use core::fmt::Write;
mod vga_buffer;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("waaa");

    loop {};
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {};
}
