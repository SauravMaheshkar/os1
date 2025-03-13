//! Interrupt handling module
pub mod gdt;
pub mod idt;

use x86_64::{
    registers::control::Cr2,
    structures::idt::{InterruptStackFrame, PageFaultErrorCode},
};

pub const PIC_1_OFFSET: u8 = 0x20;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

/// Breakpoint exception handler
///
/// This function is called when a breakpoint exception occurs, panics and
/// prints the stack frame
///
/// # Arguments
/// * `stack_frame` - The stack frame of the interrupt
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/// Double fault exception handler
///
/// This function is called when a double fault exception occurs, panics and
/// prints the stack frame
///
/// # Arguments
/// * `stack_frame` - The stack frame of the interrupt
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

/// Page fault exception handler
///
/// This function is called when a page fault exception occurs, logs the
/// accessed address, error code and the stack frame
///
/// # Arguments
/// * `stack_frame` - The stack frame of the interrupt
/// * `error_code` - The error code of the interrupt
pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    log::info!("Exception : Page Fault");
    log::info!("Accessed address : {:?}", Cr2::read());
    log::info!("ErrorCode : {:?}", error_code);
    log::info!("{:#?}", stack_frame);
}
