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
    use rust_os::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Hello, world{}", "!");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    use x86_64::structures::paging::PageTable;

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);

            // get the physical address from the entry and convert it
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };

            // print non-emptry entries of the level 3 table
            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("L3 Entry {}: {:?}", i, entry);
                }
            }
        }
    }


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
