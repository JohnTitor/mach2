//! This module corresponds to `mach/host_info.h`.

use crate::message::mach_msg_type_number_t;
use crate::vm_statistics::vm_statistics64_data_t;
use crate::vm_types::integer_t;
use core::mem;

pub type host_flavor_t = integer_t;
pub type host_info64_t = *mut integer_t;

pub const HOST_CPU_COUNTERS_INFO: host_flavor_t = 8;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct host_cpu_counters_info {
    pub user_time_mach: u64,
    pub system_time_mach: u64,
    pub idle_time_mach: u64,
    pub cycles: u64,
    pub instructions: u64,
    pub energy_nj: u64,
}

pub type host_cpu_counters_info_data_t = host_cpu_counters_info;
pub type host_cpu_counters_info_t = *mut host_cpu_counters_info;
pub const HOST_CPU_COUNTERS_INFO_COUNT: mach_msg_type_number_t =
    (mem::size_of::<host_cpu_counters_info_data_t>() / mem::size_of::<integer_t>())
        as mach_msg_type_number_t;

pub const HOST_VM_INFO64_COUNT: mach_msg_type_number_t = (mem::size_of::<vm_statistics64_data_t>()
    / mem::size_of::<integer_t>())
    as mach_msg_type_number_t;
pub const HOST_VM_INFO64_LATEST_COUNT: mach_msg_type_number_t = HOST_VM_INFO64_COUNT;
pub const HOST_VM_INFO64_REV6_COUNT: mach_msg_type_number_t = HOST_VM_INFO64_COUNT;
pub const HOST_VM_INFO64_REV5_COUNT: mach_msg_type_number_t =
    (mem::offset_of!(vm_statistics64_data_t, executable_count) / mem::size_of::<integer_t>())
        as mach_msg_type_number_t;
pub const HOST_VM_INFO64_REV4_COUNT: mach_msg_type_number_t =
    (mem::offset_of!(vm_statistics64_data_t, swap_count) / mem::size_of::<integer_t>())
        as mach_msg_type_number_t;
pub const HOST_VM_INFO64_REV3_COUNT: mach_msg_type_number_t =
    (mem::offset_of!(vm_statistics64_data_t, speculative_pages_created)
        / mem::size_of::<integer_t>()) as mach_msg_type_number_t;
pub const HOST_VM_INFO64_REV2_COUNT: mach_msg_type_number_t = (mem::offset_of!(
    vm_statistics64_data_t,
    total_tag_storage_pages
) / mem::size_of::<integer_t>())
    as mach_msg_type_number_t;
