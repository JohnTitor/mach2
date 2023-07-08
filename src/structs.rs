//! This module corresponds to `mach/i386/_structs.h` and `mach/arm/_structs.h`.

use core::mem;

use super::message::mach_msg_type_number_t;

#[cfg(target_arch = "aarch64")]
pub type arm_thread_state32_t = __darwin_arm_thread_state;
#[cfg(target_arch = "aarch64")]
pub type arm_thread_state64_t = __darwin_arm_thread_state64;
#[cfg(target_arch = "aarch64")]
pub type arm_state_hdr_t = arm_state_hdr;
#[cfg(target_arch = "aarch64")]
pub type arm_unified_thread_state_t = arm_unified_thread_state;

#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct arm_state_hdr {
    pub flavor: u32,
    pub count: u32,
}

#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_unified_thread_state {
    pub ash: arm_state_hdr_t,
    pub uts: arm_unified_thread_state__bindgen_ty_1,
}

#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Copy, Clone)]
pub union arm_unified_thread_state__bindgen_ty_1 {
    pub ts_32: arm_thread_state32_t,
    pub ts_64: arm_thread_state64_t,
}

#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_thread_state {
    pub __r: [u32; 13usize],
    pub __sp: u32,
    pub __lr: u32,
    pub __pc: u32,
    pub __cpsr: u32,
}

#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct __darwin_arm_thread_state64 {
    pub __x: [u64; 29usize],
    pub __fp: u64,
    pub __lr: u64,
    pub __sp: u64,
    pub __pc: u64,
    pub __cpsr: u32,
    pub __pad: u32,
}

#[cfg(target_arch = "aarch64")]
impl __darwin_arm_thread_state64 {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn count() -> mach_msg_type_number_t {
        (mem::size_of::<Self>() / mem::size_of::<::libc::c_int>()) as mach_msg_type_number_t
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct x86_thread_state64_t {
    pub __rax: u64,
    pub __rbx: u64,
    pub __rcx: u64,
    pub __rdx: u64,
    pub __rdi: u64,
    pub __rsi: u64,
    pub __rbp: u64,
    pub __rsp: u64,
    pub __r8: u64,
    pub __r9: u64,
    pub __r10: u64,
    pub __r11: u64,
    pub __r12: u64,
    pub __r13: u64,
    pub __r14: u64,
    pub __r15: u64,
    pub __rip: u64,
    pub __rflags: u64,
    pub __cs: u64,
    pub __fs: u64,
    pub __gs: u64,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl x86_thread_state64_t {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn count() -> mach_msg_type_number_t {
        (mem::size_of::<Self>() / mem::size_of::<::libc::c_int>()) as mach_msg_type_number_t
    }
}
