//! # I/O and Communication
pub mod rtc;
pub mod serial;
pub mod vga;
use core::{arch::asm, cell::RefCell};

use hashbrown::HashSet;

pub struct Port {
    addr: u16,
}

impl Port {
    pub fn writeb(&mut self, val: u8) {
        unsafe {
            asm!(r#"
                .att_syntax
                out %al, %dx
                "#,
                in("dx") self.addr,
                in("al") val
            );
        }
    }

    pub fn readb(&mut self) -> u8 {
        unsafe {
            let mut ret;

            asm!(r#"
                .att_syntax
                in %dx, %al
                "#,
                in("dx") self.addr,
                out("al") ret
            );

            ret
        }
    }
}

pub struct PortHandler {
    ports: HashSet<u16>,
}

impl PortHandler {
    pub fn new() -> Self {
        PortHandler {
            ports: Default::default(),
        }
    }

    pub fn add(&mut self, port: u16) -> Option<Port> {
        if self.ports.contains(&port) {
            return None;
        }

        self.ports.insert(port);
        Some(Port { addr: port })
    }
}

pub struct Sinks {
    pub serial: Option<serial::Serial>,
    pub vga: Option<vga::TerminalWriter>,
}

pub struct SinksLock {
    data: RefCell<Sinks>,
}

impl SinksLock {
    pub const fn new() -> Self {
        SinksLock {
            data: RefCell::new(Sinks {
                serial: None,
                vga: None,
            }),
        }
    }
}

impl core::ops::Deref for SinksLock {
    type Target = RefCell<Sinks>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// Single Threaded
unsafe impl Sync for SinksLock {}

pub static SINKS: SinksLock = SinksLock::new();

pub struct SinkPort {
    port: RefCell<Option<Port>>,
}

impl SinkPort {
    pub const fn new() -> Self {
        SinkPort {
            port: RefCell::new(None),
        }
    }
}

// Single Threaded
unsafe impl Sync for SinkPort {}

pub static SINK_PORT: SinkPort = SinkPort::new();

pub fn init_io_ports(port_handler: &mut PortHandler) {
    let mut sinks = SINKS.data.borrow_mut();
    sinks.vga = Some(vga::TerminalWriter::new());
    sinks.serial = match serial::Serial::new(port_handler) {
        Ok(serial) => Some(serial),
        Err(e) => {
            println_serial!("Failed to initialise Serial Communication: {e}");
            None
        }
    };
}

pub fn init_io_exit(port_handler: &mut PortHandler) {
    const ISA_DEBUG_EXIT_PORT_NUM: u16 = 0xf4;
    let mut port = SINK_PORT.port.borrow_mut();
    *port = Some(
        port_handler
            .add(ISA_DEBUG_EXIT_PORT_NUM)
            .expect("Failed to initialise ISA Debug Exit Port"),
    )
}

pub unsafe fn exit(code: u8) {
    let mut port = SINK_PORT.port.borrow_mut();
    port.as_mut()
        .expect("io exit port not initialised")
        .writeb(code);
}
