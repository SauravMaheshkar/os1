pub mod gdt;
pub mod guard;
pub mod idt;

extern "x86-interrupt" fn general_protection_fault() {
    println_serial!("General Protection Fault");
}
