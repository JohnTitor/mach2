//! This module corresponds to `mach/arm/vm_param.h`.

#[cfg(target_arch = "arm")]
mod arm {
    use crate::vm_types::{mach_vm_offset_t, vm_address_t};

    pub const VM_MIN_ADDRESS_RAW: u32 = 0;
    pub const VM_MAX_ADDRESS_RAW: u32 = 0x8000_0000;
    pub const MACH_VM_MIN_ADDRESS_RAW: u32 = VM_MIN_ADDRESS_RAW;
    pub const MACH_VM_MAX_ADDRESS_RAW: u32 = VM_MAX_ADDRESS_RAW;
    pub const MACH_VM_ADDRESS_MASK: u32 = 0x7fff_ffff;

    pub const VM_MIN_ADDRESS: vm_address_t = VM_MIN_ADDRESS_RAW as vm_address_t;
    pub const VM_MAX_ADDRESS: vm_address_t = VM_MAX_ADDRESS_RAW as vm_address_t;
    pub const MACH_VM_MIN_ADDRESS: mach_vm_offset_t = MACH_VM_MIN_ADDRESS_RAW as mach_vm_offset_t;
    pub const MACH_VM_MAX_ADDRESS: mach_vm_offset_t = MACH_VM_MAX_ADDRESS_RAW as mach_vm_offset_t;
}

#[cfg(target_arch = "arm")]
pub use self::arm::*;

#[cfg(target_arch = "aarch64")]
mod aarch64 {
    use crate::vm_types::{mach_vm_offset_t, vm_address_t};

    pub const VM_MIN_ADDRESS_RAW: u64 = 0;
    pub const VM_MAX_ADDRESS_RAW: u64 = 0xf000_0000;
    pub const MACH_VM_MIN_ADDRESS_RAW: u64 = VM_MIN_ADDRESS_RAW;
    pub const MACH_VM_MAX_ADDRESS_RAW: u64 = 0x0000_7fff_fe00_0000;
    pub const MACH_VM_ADDRESS_MASK: u64 = 0x0000_7fff_ffff_ffff;
    pub const MACH_VM_NANO_BASE_ADDRESS: u64 = 0x0000_6000_0000_0000;
    pub const MACH_VM_MIN_GPU_CARVEOUT_ADDRESS_RAW: u64 = 0x0000_0010_0000_0000;
    pub const MACH_VM_MAX_GPU_CARVEOUT_ADDRESS_RAW: u64 = 0x0000_0070_0000_0000;

    pub const VM_MIN_ADDRESS: vm_address_t = VM_MIN_ADDRESS_RAW as vm_address_t;
    pub const VM_MAX_ADDRESS: vm_address_t = VM_MAX_ADDRESS_RAW as vm_address_t;
    pub const MACH_VM_MIN_ADDRESS: mach_vm_offset_t = MACH_VM_MIN_ADDRESS_RAW;
    pub const MACH_VM_MAX_ADDRESS: mach_vm_offset_t = MACH_VM_MAX_ADDRESS_RAW;
    pub const MACH_VM_MIN_GPU_CARVEOUT_ADDRESS: mach_vm_offset_t =
        MACH_VM_MIN_GPU_CARVEOUT_ADDRESS_RAW;
    pub const MACH_VM_MAX_GPU_CARVEOUT_ADDRESS: mach_vm_offset_t =
        MACH_VM_MAX_GPU_CARVEOUT_ADDRESS_RAW;
}

#[cfg(target_arch = "aarch64")]
pub use self::aarch64::*;
