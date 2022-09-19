#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rust_os::hlt_loop();
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    rust_os::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    /*fn stack_overflow(){
        stack_overflow();
    }*/

    let ptr = 0x205623 as *mut u32;
    unsafe {
        let x = *ptr;
    }
    println!("read worked");
    unsafe {
        *ptr = 42;
    }
    println!("Write worked");

    //uncomment line below to trigger a stack overflow
    //  stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    rust_os::hlt_loop();
}

/// This function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
