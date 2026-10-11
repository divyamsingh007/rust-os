//removing standard rust code using the below
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

//PanicInfo parameter collect the data of file and line which occured the error
use core::panic::PanicInfo;

//our own panic controller, here is no error!
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

//made our own public(pub) entry point fn for kernel
// this function is the entry point, since the linker looks for a function
// named `_start` by default
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb
        }
    }
    loop {}
}
