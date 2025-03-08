use x86_64::structures::idt::InterruptStackFrame;

use crate::drivers::apic::{self, registers::APICRegisters};

pub unsafe fn init(local_apic_ptr: *mut u32) {
    let keyboard_register =
        local_apic_ptr.offset(APICRegisters::LvtLint1 as isize / 4);
    keyboard_register.write_volatile(
        crate::interrupts::InterruptIndex::Keyboard as u8 as u32,
    );
}

pub extern "x86-interrupt" fn keyboard_handler(
    _stack_frame: InterruptStackFrame,
) {
    use pc_keyboard::{
        layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1,
    };
    use spin::{Lazy, Mutex};
    use x86_64::instructions::port::Port;

    static KEYBOARD: Lazy<Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>>> =
        Lazy::new(|| {
            Mutex::new(Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore,
            ))
        });

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => log::info!("{}", character),
                DecodedKey::RawKey(key) => log::info!("{:?}", key),
            }
        }
    }

    apic::end_interrupt();
}
