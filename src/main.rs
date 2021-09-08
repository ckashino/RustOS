#![no_std]
#![no_main]
mod vga_buffer;
//mod game_funcs;

use core::panic::PanicInfo;

use vga_buffer::Color;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

static HELLO: &[u8] = b"Hello World";

#[no_mangle]
pub extern "C" fn _start() -> ! {

    vga_buffer::print_something();


    loop{}

}
