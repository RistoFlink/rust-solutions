//don't link the standard Rust library
#![no_std]
//disable all Rust-level entry points
#![no_main]

use core::panic::PanicInfo;

#[no_mangle] //don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    //this is the entry point since the linker loosk for a function named _start by default
    loop {} //indefinite loop
}
//create a new panic function since the normal one can't be used
//VSCode complains about this due to thinking the normal one still is in use..
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} //indefinite loop
}
