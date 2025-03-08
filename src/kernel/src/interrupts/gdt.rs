use core::ptr::addr_of;

use spin::Lazy;
use x86_64::{
    registers::segmentation::{SegmentSelector, SS},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable},
        tss::TaskStateSegment,
    },
    VirtAddr,
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

/// Task State Segment.
/// Structure on x86-based computers which holds information about a task
pub static TSS: Lazy<TaskStateSegment> = Lazy::new(|| {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(addr_of!(STACK));
        stack_start + STACK_SIZE as u64
    };
    tss
});

/// Global Descriptor Table.
/// Construct used by the x86 processor to configure segmented virtual memory
pub static GDT: Lazy<(GlobalDescriptorTable, Selectors)> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let code_selector = gdt.append(Descriptor::kernel_code_segment());
    let tss_selector = gdt.append(Descriptor::tss_segment(&TSS));
    (
        gdt,
        Selectors {
            code_selector,
            tss_selector,
        },
    )
});

pub struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::{
        segmentation::{Segment, CS},
        tables::load_tss,
    };

    GDT.0.load();

    unsafe {
        CS::set_reg(GDT.1.code_selector);
        SS::set_reg(SegmentSelector(0));
        load_tss(GDT.1.tss_selector);
    }
}
