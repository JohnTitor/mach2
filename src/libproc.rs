//! This module roughly corresponds to `libproc.h`.

use super::{
    kern_return::kern_return_t,
    vm_types::{integer_t, mach_vm_address_t, mach_vm_size_t},
};

pub const PROC_PIDPATHINFO_MAXSIZE: ::libc::c_uint = 4096;

extern "C" {
    pub fn proc_pidpath(pid: ::libc::pid_t, buffer: *mut ::libc::c_void, buffersize: ::libc::c_uint) -> kern_return_t;
    pub fn proc_regionfilename(
        pid: ::libc::pid_t,
        address: ::libc::c_ulong,
        buffer: *mut ::libc::c_void,
        buffersize: ::libc::c_uint,
    ) -> kern_return_t;
    pub fn proc_pidinfo(
        pid: ::core::ffi::c_int,
        flavor: ::core::ffi::c_int,
        arg: u64,
        buffer: *mut ::core::ffi::c_void,
        buffersize: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct proc_regioninfo {
    pub pri_protection: u32,
    pub pri_max_protection: u32,
    pub pri_inheritance: u32,
    pub pri_flags: u32,
    pub pri_offset: u64,
    pub pri_behavior: u32,
    pub pri_user_wired_count: u32,
    pub pri_user_tag: u32,
    pub pri_pages_resident: u32,
    pub pri_pages_shared_now_private: u32,
    pub pri_pages_swapped_out: u32,
    pub pri_pages_dirtied: u32,
    pub pri_ref_count: u32,
    pub pri_shadow_depth: u32,
    pub pri_share_mode: u32,
    pub pri_private_pages_resident: u32,
    pub pri_shared_pages_resident: u32,
    pub pri_obj_id: u32,
    pub pri_depth: u32,
    pub pri_address: u64,
    pub pri_size: u64,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_vm_info {
    pub virtual_size: mach_vm_size_t,
    pub region_count: integer_t,
    pub page_size: integer_t,
    pub resident_size: mach_vm_size_t,
    pub resident_size_peak: mach_vm_size_t,
    pub device: mach_vm_size_t,
    pub device_peak: mach_vm_size_t,
    pub internal: mach_vm_size_t,
    pub internal_peak: mach_vm_size_t,
    pub external: mach_vm_size_t,
    pub external_peak: mach_vm_size_t,
    pub reusable: mach_vm_size_t,
    pub reusable_peak: mach_vm_size_t,
    pub purgeable_volatile_pmap: mach_vm_size_t,
    pub purgeable_volatile_resident: mach_vm_size_t,
    pub purgeable_volatile_virtual: mach_vm_size_t,
    pub compressed: mach_vm_size_t,
    pub compressed_peak: mach_vm_size_t,
    pub compressed_lifetime: mach_vm_size_t,
    pub phys_footprint: mach_vm_size_t,
    pub min_address: mach_vm_address_t,
    pub max_address: mach_vm_address_t,
    pub ledger_phys_footprint_peak: i64,
    pub ledger_purgeable_nonvolatile: i64,
    pub ledger_purgeable_novolatile_compressed: i64,
    pub ledger_purgeable_volatile: i64,
    pub ledger_purgeable_volatile_compressed: i64,
    pub ledger_tag_network_nonvolatile: i64,
    pub ledger_tag_network_nonvolatile_compressed: i64,
    pub ledger_tag_network_volatile: i64,
    pub ledger_tag_network_volatile_compressed: i64,
    pub ledger_tag_media_footprint: i64,
    pub ledger_tag_media_footprint_compressed: i64,
    pub ledger_tag_media_nofootprint: i64,
    pub ledger_tag_media_nofootprint_compressed: i64,
    pub ledger_tag_graphics_footprint: i64,
    pub ledger_tag_graphics_footprint_compressed: i64,
    pub ledger_tag_graphics_nofootprint: i64,
    pub ledger_tag_graphics_nofootprint_compressed: i64,
    pub ledger_tag_neural_footprint: i64,
    pub ledger_tag_neural_footprint_compressed: i64,
    pub ledger_tag_neural_nofootprint: i64,
    pub ledger_tag_neural_nofootprint_compressed: i64,
    pub limit_bytes_remaining: u64,
    pub decompressions: integer_t,
    pub ledger_swapins: i64,
}
