//don't link the standard Rust library
#![no_std]
//disable all Rust-level entry points
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

pub mod gdt;

#[no_mangle] //don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello, world{}", "!");
    rust_os::init();

    let ptr = 0xdeadbeef as *mut u8;
    unsafe { *ptr = 42 };

    #[cfg(test)]
    // IDE complains about this missing but it still runs..
    test_main();
    println!("All good!");
    rust_os::hlt_loop();
}

//create a new panic function since the normal one can't be used
//VSCode complains about this due to thinking the normal one still is in use..
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}


// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

// #[test_case]
// fn trivial_assertion() {
//     assert_eq!(1, 1);
// }

// to run it in the virtual machine: qemu-system-x86_64 -drive format=raw,file=target/os/debug/bootimage-rust-os.bin
