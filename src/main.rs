#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os1::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use os1::{mem, println};

entry_point!(kernel);

#[no_mangle]
fn kernel(info: &'static BootInfo) -> ! {
    use os1::mem::paging::BootInfoFrameAllocator;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");

    os1::init();

    let phys_mem_offset = VirtAddr::new(info.physical_memory_offset);
    let mut mapper = unsafe { os1::mem::paging::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { BootInfoFrameAllocator::init(&info.memory_map) };

    mem::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    #[cfg(test)]
    test_main();

    os1::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC]: {}\n", info);
    os1::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os1::test_panic_handler(info)
}
