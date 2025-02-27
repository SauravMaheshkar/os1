//! Real Time Clock (RTC) driver
//!
//! * https://wiki.osdev.org/CMOS
use thiserror_no_std::Error;

use crate::io::{Port, PortHandler};

fn nmi_mask(nmi_enable: bool) -> u8 {
    if nmi_enable {
        0
    } else {
        1 << 7
    }
}

fn read_cmos_reg(
    control_port: &mut Port,
    data_port: &mut Port,
    nmi_enable: bool,
    reg: u8,
) -> u8 {
    control_port.writeb(nmi_mask(nmi_enable) | reg);
    data_port.readb()
}

fn write_cmos_reg(
    control_port: &mut Port,
    data_port: &mut Port,
    nmi_enable: bool,
    reg: u8,
    val: u8,
) {
    control_port.writeb(nmi_mask(nmi_enable) | reg);
    data_port.writeb(val);
}

fn fmt(cmos_nmi_control_port: &mut Port, cmos_data_port: &mut Port) {
    let mut status_reg =
        read_cmos_reg(cmos_nmi_control_port, cmos_data_port, true, 0x0B);

    status_reg |= 1 << 1;
    status_reg |= 1 << 2;

    write_cmos_reg(
        cmos_nmi_control_port,
        cmos_data_port,
        true,
        0x0B,
        status_reg,
    );
}

fn rtc_busy(control_port: &mut Port, data_port: &mut Port) -> bool {
    control_port.writeb(nmi_mask(true) | 0x0a); // Status Register A
    const BUSY: u8 = 1 << 7;
    (data_port.readb() & BUSY) != BUSY
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DateTime {
    seconds: u8,
    minutes: u8,
    hours: u8,
    weekday: u8,
    day: u8,
    month: u8,
    year: u8,
    century: u8,
}

#[derive(Error, Debug)]
pub enum RtcInitError {
    #[error("Failed to connect to control port")]
    FailedToConnectToControlPort,
    #[error("Failed to connect to data port")]
    FailedToConnectToDataPort,
}

pub struct Rtc {
    cmos_nmi_control_port: Port,
    cmos_data_port: Port,
}

impl Rtc {
    pub fn new(port_handler: &mut PortHandler) -> Result<Self, RtcInitError> {
        let mut cmos_nmi_control_port = port_handler
            .add(0x70)
            .ok_or(RtcInitError::FailedToConnectToControlPort)?;
        let mut cmos_data_port = port_handler
            .add(0x71)
            .ok_or(RtcInitError::FailedToConnectToDataPort)?;

        // Enable 24 hr fmt + binary coded decimal
        fmt(&mut cmos_nmi_control_port, &mut cmos_data_port);

        Ok(Rtc {
            cmos_nmi_control_port,
            cmos_data_port,
        })
    }

    pub fn get(&mut self) -> DateTime {
        let mut ret;
        loop {
            while rtc_busy(
                &mut self.cmos_nmi_control_port,
                &mut self.cmos_data_port,
            ) {
                continue;
            }
            let seconds = read_cmos_reg(
                &mut self.cmos_nmi_control_port,
                &mut self.cmos_data_port,
                true,
                0x00,
            );
            let minutes = read_cmos_reg(
                &mut self.cmos_nmi_control_port,
                &mut self.cmos_data_port,
                true,
                0x02,
            );
            let hours = read_cmos_reg(
                &mut self.cmos_nmi_control_port,
                &mut self.cmos_data_port,
                true,
                0x04,
            );
            let weekday = read_cmos_reg(
                &mut self.cmos_nmi_control_port,
                &mut self.cmos_data_port,
                true,
                0x06,
            );
            let day = read_cmos_reg(
                &mut self.cmos_nmi_control_port,
                &mut self.cmos_data_port,
                true,
                0x07,
            );
            let month = read_cmos_reg(
                &mut self.cmos_nmi_control_port,
                &mut self.cmos_data_port,
                true,
                0x08,
            );
            let year = read_cmos_reg(
                &mut self.cmos_nmi_control_port,
                &mut self.cmos_data_port,
                true,
                0x09,
            );
            let century = read_cmos_reg(
                &mut self.cmos_nmi_control_port,
                &mut self.cmos_data_port,
                true,
                0x32,
            );

            ret = DateTime {
                seconds,
                minutes,
                hours,
                weekday,
                day,
                month,
                year,
                century,
            };

            if rtc_busy(
                &mut self.cmos_nmi_control_port,
                &mut self.cmos_data_port,
            ) {
                continue;
            }

            break;
        }

        ret
    }
}
