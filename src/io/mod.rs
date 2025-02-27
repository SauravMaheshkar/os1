//! # I/O and Communication
pub mod rtc;
pub mod serial;
pub mod vga;
use core::arch::asm;

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
