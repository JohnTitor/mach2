//! This module corresponds to `mach/machine.h`.

use crate::loader::cpu_subtype_t;

pub const CPU_SUBTYPE_MASK: cpu_subtype_t = 0xff00_0000u32 as cpu_subtype_t;
pub const CPU_SUBTYPE_ARM64E: cpu_subtype_t = 2;
pub const CPU_SUBTYPE_ARM64_X1: cpu_subtype_t = 3;
pub const CPU_SUBTYPE_ARM64E_X1: cpu_subtype_t = 12;
pub const CPU_SUBTYPE_ARM64_MAX: cpu_subtype_t = 20;

#[inline]
pub const fn cpu_subtype_has_arm64e(subtype: cpu_subtype_t) -> bool {
    matches!(
        subtype & !CPU_SUBTYPE_MASK,
        CPU_SUBTYPE_ARM64E | CPU_SUBTYPE_ARM64E_X1
    )
}

#[inline]
pub const fn cpu_subtype_has_arm64x1(subtype: cpu_subtype_t) -> bool {
    matches!(
        subtype & !CPU_SUBTYPE_MASK,
        CPU_SUBTYPE_ARM64_X1 | CPU_SUBTYPE_ARM64E_X1
    )
}

pub const CPUFAMILY_ARM_TAHITI: u32 = 0x75d4_acb9;
pub const CPUFAMILY_ARM_TUPAI: u32 = 0x2045_26d0;
pub const CPUFAMILY_ARM_SOTRA: u32 = 0xf76c_5b1a;
pub const CPUFAMILY_ARM_THERA: u32 = 0xab34_5f09;
pub const CPUFAMILY_ARM_TILOS: u32 = 0x01d7_a72b;
pub const CPUFAMILY_ARM_KOMODO: u32 = 0x6d0c_cb0c;
pub const CPUFAMILY_ARM_BORNEO: u32 = 0x7db5_6df1;
pub const CPUFAMILY_ARM_NEVIS: u32 = 0x3765_2b0c;
