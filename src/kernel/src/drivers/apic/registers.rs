/// Local APIC Registers
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[repr(isize)]
#[allow(dead_code)]
pub enum APICRegisters {
    /// RESERVED = 0x00
    R0x00 = 0x0,
    /// RESERVED = 0x10
    R0x10 = 0x10,
    /// ID Register
    Ir = 0x20,
    /// Version Register
    Vr = 0x30,
    /// RESERVED = 0x40
    R0x40 = 0x40,
    /// RESERVED = 0x50
    R0x50 = 0x50,
    /// RESERVED = 0x60
    R0x60 = 0x60,
    /// RESERVED = 0x70
    R0x70 = 0x70,
    /// Task Priority Register
    Tpr = 0x80,
    /// Arbitration Priority Register
    Apr = 0x90,
    /// Processor Priority Register
    Ppr = 0xA0,
    /// EOI Register
    Eoi = 0xB0,
    /// Remote Read Register
    Rrd = 0xC0,
    /// Logical Destination Register
    Ldr = 0xD0,
    /// Destination Format Register
    Dfr = 0xE0,
    /// Spurious (Interrupt) Vector Register
    Svr = 0xF0,
    /// In-Service Register 1
    Isr1 = 0x100,
    /// In-Service Register 2
    Isr2 = 0x110,
    /// In-Service Register 3
    Isr3 = 0x120,
    /// In-Service Register 4
    Isr4 = 0x130,
    /// In-Service Register 5
    Isr5 = 0x140,
    /// In-Service Register 6
    Isr6 = 0x150,
    /// In-Service Register 7
    Isr7 = 0x160,
    /// In-Service Register 8
    Isr8 = 0x170,
    /// Trigger Mode Register 1
    Tmr1 = 0x180,
    /// Trigger Mode Register 2
    Tmr2 = 0x190,
    /// Trigger Mode Register 3
    Tmr3 = 0x1A0,
    /// Trigger Mode Register 4
    Tmr4 = 0x1B0,
    /// Trigger Mode Register 5
    Tmr5 = 0x1C0,
    /// Trigger Mode Register 6
    Tmr6 = 0x1D0,
    /// Trigger Mode Register 7
    Tmr7 = 0x1E0,
    /// Trigger Mode Register 8
    Tmr8 = 0x1F0,
    /// Interrupt Request Register 1
    Irr1 = 0x200,
    /// Interrupt Request Register 2
    Irr2 = 0x210,
    /// Interrupt Request Register 3
    Irr3 = 0x220,
    /// Interrupt Request Register 4
    Irr4 = 0x230,
    /// Interrupt Request Register 5
    Irr5 = 0x240,
    /// Interrupt Request Register 6
    Irr6 = 0x250,
    /// Interrupt Request Register 7
    Irr7 = 0x260,
    /// Interrupt Request Register 8
    Irr8 = 0x270,
    /// Error Status Register
    Esr = 0x280,
    /// RESERVED = 0x290
    R0x290 = 0x290,
    /// RESERVED = 0x2A0
    R0x2A0 = 0x2A0,
    /// RESERVED = 0x2B0
    R0x2B0 = 0x2B0,
    /// RESERVED = 0x2C0
    R0x2C0 = 0x2C0,
    /// RESERVED = 0x2D0
    R0x2D0 = 0x2D0,
    /// RESERVED = 0x2E0
    R0x2E0 = 0x2E0,
    /// LVT Corrected Machine Check Interrupt (CMCI) Register
    LvtCmci = 0x2F0,
    /// Interrupt Command Register 1
    Icr1 = 0x300,
    /// Interrupt Command Register 2
    Icr2 = 0x310,
    /// LVT Timer Register
    LvtT = 0x320,
    /// LVT Thermal Sensor Register
    LvtTsr = 0x330,
    /// LVT Performance Monitoring Counters Register
    LvtPmcr = 0x340,
    /// LVT LINT0 Register
    LvtLint0 = 0x350,
    /// LVT LINT1 Register
    LvtLint1 = 0x360,
    /// LVT Error Register
    LvtE = 0x370,
    /// Initial Count Register (for Timer)
    Ticr = 0x380,
    /// Current Count Register (for Timer)
    Tccr = 0x390,
    /// RESERVED = 0x3A0
    R0x3A0 = 0x3A0,
    /// RESERVED = 0x3B0
    R0x3B0 = 0x3B0,
    /// RESERVED = 0x3C0
    R0x3C0 = 0x3C0,
    /// RESERVED = 0x3D0
    R0x3D0 = 0x3D0,
    /// Divide Configuration Register (for Timer)
    Tdcr = 0x3E0,
    /// RESERVED = 0x3F0
    R0x3F0 = 0x3F0,
}
