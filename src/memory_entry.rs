//! This module corresponds to `mach/memory_entry.h`.

use crate::kern_return::kern_return_t;
use crate::mach_types::mem_entry_name_port_t;
use crate::message::{mach_msg_header_t, mach_msg_type_number_t};
use crate::ndr::NDR_record_t;
use crate::vm_region::vm_region_recurse_info_t;
use crate::vm_types::{mach_vm_offset_t, mach_vm_size_t};
use core::ffi::c_int;

pub const memory_entry_MSG_COUNT: u32 = 5;

unsafe extern "C" {
    pub fn mach_memory_entry_region_info(
        mem_entry: mem_entry_name_port_t,
        offset: *mut mach_vm_offset_t,
        size: *mut mach_vm_size_t,
        info: vm_region_recurse_info_t,
        infoCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
#[allow(non_snake_case)]
pub struct __Request__mach_memory_entry_region_info_t {
    pub Head: mach_msg_header_t,
    pub NDR: NDR_record_t,
    pub offset: mach_vm_offset_t,
    pub infoCnt: mach_msg_type_number_t,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
#[allow(non_snake_case)]
pub struct __Reply__mach_memory_entry_region_info_t {
    pub Head: mach_msg_header_t,
    pub NDR: NDR_record_t,
    pub RetCode: kern_return_t,
    pub offset: mach_vm_offset_t,
    pub size: mach_vm_size_t,
    pub infoCnt: mach_msg_type_number_t,
    pub info: [c_int; 21],
}
