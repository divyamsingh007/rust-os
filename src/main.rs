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

//made our own public(pub) entry point fn for kernel
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop{}
}