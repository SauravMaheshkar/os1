//! Keyboard Utilities

use core::{
    pin::Pin,
    task::{Context, Poll},
};

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::{
    stream::{Stream, StreamExt},
    task::AtomicWaker,
};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use x86_64::structures::idt::InterruptStackFrame;

use crate::drivers::apic::{self, registers::APICRegisters};

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

/// Initialize the keyboard
///
/// # Safety
/// This function directly writes to memory-mapped Local APIC registers.
///
/// # Arguments
/// * `local_apic_ptr` - A pointer to the Local APIC registers
pub unsafe fn init(local_apic_ptr: *mut u32) {
    let keyboard_register =
        local_apic_ptr.offset(APICRegisters::LvtLint1 as isize / 4);
    keyboard_register.write_volatile(
        crate::interrupts::InterruptIndex::Keyboard as u8 as u32,
    );
}

/// Called by the keyboard interrupt handler (must not block or allocate)
///
/// Adds a scancode to the scancode queue
///
/// # Arguments
/// * `scancode` - The scancode received from the keyboard
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if queue.push(scancode).is_err() {
            log::warn!("scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        log::warn!("WARNING: scancode queue uninitialized");
    }
}

/// A stream of scancodes
pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    /// Create a new ScancodeStream (should only be called once)
    ///
    /// initializes the scancode queue with [`ArrayQueue`](https://docs.rs/crossbeam/latest/crossbeam/queue/struct.ArrayQueue.html)
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    /// Poll for the next scancode
    ///
    /// If the scancode queue is empty, the current task is registered to be
    /// woken up when a scancode is added to the queue.
    ///
    /// # Arguments
    /// * `cx` - The current task's context
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        // try to get the scancode queue
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        // fast path
        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        // slow path
        WAKER.register(cx.waker());
        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

/// Print keypresses to the log
///
/// This function acts as an example of how to use the [`ScancodeStream`] and
/// `pc_keyboard::Keyboard`.
///
/// # Example
/// ```no_run
/// let mut executor = executor::Executor::new();
/// executor.spawn(Task::new(keyboard::print_keypresses()));
/// executor.run();
/// ```
pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        log::info!("{}", character)
                    }
                    DecodedKey::RawKey(key) => log::info!("{:?}", key),
                }
            }
        }
    }
}

/// Keyboard interrupt handler
///
/// reads the scancode from the keyboard port and adds it to the scancode queue
///
/// # Arguments
/// * `_stack_frame` - The interrupt stack frame
pub extern "x86-interrupt" fn keyboard_handler(
    _stack_frame: InterruptStackFrame,
) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    crate::devices::keyboard::add_scancode(scancode);

    apic::end_interrupt();
}
