#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World! This is my OS";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    // panic!("Some panic message");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // vga_buffer::panic_print(info);
    loop {}
}