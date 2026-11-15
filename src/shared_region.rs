//! This module corresponds to `mach/shared_region.h`.

#[cfg(target_pointer_width = "64")]
use crate::vm_types::mach_vm_address_t;

pub const SHARED_REGION_BASE_ARM64: u64 = 0x0001_8000_0000;
pub const SHARED_REGION_SIZE_ARM64: u64 = 0x000e_4000_0000;
pub const SHARED_REGION_NESTING_BASE_ARM64: u64 = SHARED_REGION_BASE_ARM64;
pub const SHARED_REGION_NESTING_SIZE_ARM64: u64 = SHARED_REGION_SIZE_ARM64;

/// The shared-region address, or packed placeholder size and base on `ENOMEM`.
///
/// The C union's bitfields share storage with `start_addr`.
#[cfg(target_pointer_width = "64")]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct src_start_address_t {
    pub start_addr: mach_vm_address_t,
}

#[cfg(target_pointer_width = "64")]
impl src_start_address_t {
    pub const fn size_gb(&self) -> u64 {
        self.start_addr & 0x1ffff
    }

    pub const fn base_addr(&self) -> u64 {
        self.start_addr >> 17
    }
}
