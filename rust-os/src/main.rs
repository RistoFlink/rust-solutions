//don't link the standard Rust library
#![no_std]
//disable all Rust-level entry points
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

//create a new panic function since the normal one can't be used
//VSCode complains about this due to thinking the normal one still is in use..
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {} //indefinite loop
}

static HELLO: &[u8] = b"Hello, world!";

#[no_mangle] //don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    //this is the entry point since the linker loosk for a function named _start by default
    /*     let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap(); */

    println!("Hello, world{}", "!");
    panic!("Generic panic message");

    loop {} //indefinite loop
}

// to run it in the virtual machine: qemu-system-x86_64 -drive format=raw,file=target/os/debug/bootimage-rust-os.bin
