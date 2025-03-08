use spin::Lazy;
use x86_64::structures::idt::InterruptDescriptorTable;

pub static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    idt.breakpoint
        .set_handler_fn(crate::interrupts::breakpoint_handler);

    unsafe {
        idt.double_fault
            .set_handler_fn(crate::interrupts::double_fault_handler)
            .set_stack_index(crate::interrupts::gdt::DOUBLE_FAULT_IST_INDEX);
    }

    idt.page_fault
        .set_handler_fn(crate::interrupts::page_fault_handler);

    idt[crate::interrupts::InterruptIndex::Timer as u8]
        .set_handler_fn(crate::devices::timer::timer_handler);

    idt[crate::interrupts::InterruptIndex::Keyboard as u8]
        .set_handler_fn(crate::devices::keyboard::keyboard_handler);

    idt
});

pub fn init() {
    IDT.load();
}
