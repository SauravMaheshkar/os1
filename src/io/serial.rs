//! # Serial Communication
//!
//! * Ref: https://wiki.osdev.org/Serial_Ports
use thiserror_no_std::Error;

use crate::io::{Port, PortHandler};

const SERIAL_PORT: u16 = 0x3f8;

#[derive(Debug, Error)]
pub enum SerialInitError {
    #[error("data port reserved")]
    DataReserved,
    #[error("enable interrupt port reserved")]
    EnableInterruptReserved,
    #[error("interrupt id port reserved")]
    InterruptIdReserved,
    #[error("line control port reserved")]
    LineControlReserved,
    #[error("modem control port reserved")]
    ModemControlReserved,
    #[error("line status port reserved")]
    LineStatusReserved,
    #[error("modem status port reserved")]
    ModemStatusReserved,
    #[error("scratch port reserved")]
    ScratchReserved,
    #[error("loopback test failed")]
    Loopback,
}

pub struct Serial {
    data: Port,
    _enable_interrupt: Port,
    _interrupt_id_fifo_control: Port,
    _line_control: Port,
    _modem_control: Port,
    line_status: Port,
    _modem_status: Port,
    _scratch: Port,
}

impl Serial {
    pub fn new(
        port_handler: &mut PortHandler,
    ) -> Result<Serial, SerialInitError> {
        let mut data = port_handler
            .add(SERIAL_PORT)
            .ok_or(SerialInitError::DataReserved)?;
        let mut enable_interrupt = port_handler
            .add(SERIAL_PORT + 1)
            .ok_or(SerialInitError::EnableInterruptReserved)?;
        let mut interrupt_id_fifo_control =
            port_handler
                .add(SERIAL_PORT + 2)
                .ok_or(SerialInitError::InterruptIdReserved)?;
        let mut line_control = port_handler
            .add(SERIAL_PORT + 3)
            .ok_or(SerialInitError::LineControlReserved)?;
        let mut modem_control = port_handler
            .add(SERIAL_PORT + 4)
            .ok_or(SerialInitError::ModemControlReserved)?;
        let line_status = port_handler
            .add(SERIAL_PORT + 5)
            .ok_or(SerialInitError::LineStatusReserved)?;
        let modem_status = port_handler
            .add(SERIAL_PORT + 6)
            .ok_or(SerialInitError::ModemStatusReserved)?;
        let scratch = port_handler
            .add(SERIAL_PORT + 7)
            .ok_or(SerialInitError::ScratchReserved)?;

        enable_interrupt.writeb(0x00);
        line_control.writeb(0x80);
        data.writeb(0x03);
        enable_interrupt.writeb(0x00);
        line_control.writeb(0x03);
        interrupt_id_fifo_control.writeb(0xC7);
        modem_control.writeb(0x0B);
        modem_control.writeb(0x1E);
        data.writeb(0xAE);

        if data.readb() != 0xAE {
            return Err(SerialInitError::Loopback);
        }

        modem_control.writeb(0x0F);

        Ok(Serial {
            data,
            _enable_interrupt: enable_interrupt,
            _interrupt_id_fifo_control: interrupt_id_fifo_control,
            _line_control: line_control,
            _modem_control: modem_control,
            line_status,
            _modem_status: modem_status,
            _scratch: scratch,
        })
    }

    fn is_transmit_empty(&mut self) -> u8 {
        self.line_status.readb() & 0x20
    }

    fn write_byte(&mut self, a: u8) {
        while self.is_transmit_empty() == 0 {}

        self.data.writeb(a);
    }
}

impl core::fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            self.write_byte(*b)
        }
        Ok(())
    }
}
