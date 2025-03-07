#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use os1::{
    mem, println,
    task::{executor::Executor, keyboard, Task},
};

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

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    // os1::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC]: {}\n", info);
    os1::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
