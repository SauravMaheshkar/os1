pub mod gdt;
pub mod guard;
pub mod idt;

use x86_64::structures::idt::InterruptStackFrame;

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println_serial!("[INTERRUPT] Breakpoint\n{:#?}", stack_frame);
}
