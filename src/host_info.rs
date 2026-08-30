//! This module corresponds to `mach/host_info.h`.

use crate::message::mach_msg_type_number_t;
use crate::vm_statistics::vm_statistics64_data_t;
use crate::vm_types::integer_t;
use core::mem;

pub const HOST_VM_INFO64_COUNT: mach_msg_type_number_t = (mem::size_of::<vm_statistics64_data_t>()
    / mem::size_of::<integer_t>())
    as mach_msg_type_number_t;
pub const HOST_VM_INFO64_LATEST_COUNT: mach_msg_type_number_t = HOST_VM_INFO64_COUNT;
pub const HOST_VM_INFO64_REV3_COUNT: mach_msg_type_number_t = HOST_VM_INFO64_COUNT;
pub const HOST_VM_INFO64_REV2_COUNT: mach_msg_type_number_t = (mem::offset_of!(
    vm_statistics64_data_t,
    total_tag_storage_pages
) / mem::size_of::<integer_t>())
    as mach_msg_type_number_t;
