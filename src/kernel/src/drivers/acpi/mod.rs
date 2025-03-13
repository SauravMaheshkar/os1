//! ACPI (Advanced Configuration and Power Interface) handler
use core::ptr::NonNull;

use acpi::{AcpiHandler, PhysicalMapping};
use x86_64::{PhysAddr, VirtAddr};

/// ACPI handler
#[derive(Clone)]
pub struct ACPI {
    /// Physical memory offset [`x86_64::VirtAddr`]
    pub physical_memory_offset: VirtAddr,
}

impl ACPI {
    /// Create a new ACPI handler
    pub fn new(physical_memory_offset: VirtAddr) -> Self {
        Self {
            physical_memory_offset,
        }
    }
}

// single threaded environment
unsafe impl Send for ACPI {}

impl AcpiHandler for ACPI {
    /// Map physical memory region to virtual memory
    ///
    /// # Safety
    /// returns a physical memory mapping proceed with caution
    ///
    /// # Arguments
    /// * `physical_address` - Physical address to map
    /// * `size` - Size of the region to map
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        let phys_addr = PhysAddr::new(physical_address as u64);
        let virt_addr = self.physical_memory_offset + phys_addr.as_u64();

        PhysicalMapping::new(
            physical_address,
            NonNull::new(virt_addr.as_mut_ptr())
                .expect("Failed to get virtual address"),
            size,
            size,
            self.clone(),
        )
    }

    /// Unmap physical memory region from virtual memory
    ///
    /// No unmapping necessary as we didn't create any new mappings
    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {
        // No unmapping necessary as we didn't create any new mappings
    }
}
