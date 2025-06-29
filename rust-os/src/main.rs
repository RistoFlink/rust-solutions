//don't link the standard Rust library
#![no_std]
//disable all Rust-level entry points
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;
use bootloader::{BootInfo, entry_point};

pub mod gdt;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory;
    use x86_64::{structures::paging::Translate, VirtAddr};

    println!("Hello, world{}", "!");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // initialize a mapper
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        // identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    // IDE complains about this missing but it still runs..
    test_main();
    println!("All good! No crash!");
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
