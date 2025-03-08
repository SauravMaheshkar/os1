use core::ptr::NonNull;

use acpi::{AcpiHandler, PhysicalMapping};
use x86_64::{PhysAddr, VirtAddr};

#[derive(Clone)]
pub struct ACPI {
    pub physical_memory_offset: VirtAddr,
}

impl ACPI {
    pub fn new(physical_memory_offset: VirtAddr) -> Self {
        Self {
            physical_memory_offset,
        }
    }
}

// impl Clone for ACPI {
//     fn clone(&self) -> Self {
//         Self {
//             physical_memory_offset: self.physical_memory_offset,
//         }
//     }
// }

unsafe impl Send for ACPI {}

impl AcpiHandler for ACPI {
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

    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {
        // No unmapping necessary as we didn't create any new mappings
    }
}
