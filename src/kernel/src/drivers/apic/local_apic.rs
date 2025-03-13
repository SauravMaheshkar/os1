//! Local APIC (Advanced Programmable Interrupt Controller) module
use spin::{Lazy, Mutex};
use x86_64::structures::paging::{FrameAllocator, Mapper, Size4KiB};

use super::map_apic;

pub static LAPIC_ADDR: Lazy<Mutex<LocalAPICAddress>> =
    Lazy::new(|| Mutex::new(LocalAPICAddress::new()));

/// Local APIC address
#[repr(C, packed)]
pub struct LocalAPICAddress {
    pub address: *mut u32,
}

impl LocalAPICAddress {
    /// Create a new LocalAPICAddress
    ///
    /// sets the address to null pointer
    pub fn new() -> Self {
        Self {
            address: core::ptr::null_mut(),
        }
    }
}

// single threaded environment
unsafe impl Send for LocalAPICAddress {}

/// Initialize the local APIC
///
/// # Arguments
/// * `local_apic_addr` - The physical address of the Local APIC
/// * `mapper` - The mapper to use for mapping
///   ([`x86_64::structures::paging::Mapper`])
/// * `frame_allocator` - The frame allocator to use for allocating frames
///  ([`x86_64::structures::paging::FrameAllocator`])
pub unsafe fn init_local_apic(
    local_apic_addr: usize,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let virtual_address =
        map_apic(local_apic_addr as u64, mapper, frame_allocator);

    let local_apic_ptr = virtual_address.as_mut_ptr::<u32>();
    LAPIC_ADDR.lock().address = local_apic_ptr;

    crate::devices::timer::init(local_apic_ptr);
    crate::devices::keyboard::init(local_apic_ptr);
}
