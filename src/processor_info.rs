//! This module corresponds to `mach/processor_info.h`.

use crate::message::mach_msg_type_number_t;
use crate::vm_types::{integer_t, natural_t};
use core::ffi::c_int;
use core::mem;

pub type processor_flavor_t = c_int;
pub type processor_info_array_t = *mut integer_t;

pub const PROCESSOR_CPU_COUNTERS_INFO: processor_flavor_t = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct processor_cpu_counters_info {
    pub user_time_mach: u64,
    pub system_time_mach: u64,
    pub idle_time_mach: u64,
    pub cycles: u64,
    pub instructions: u64,
    pub energy_nj: u64,
}

pub type processor_cpu_counters_info_data_t = processor_cpu_counters_info;
pub type processor_cpu_counters_info_t = *mut processor_cpu_counters_info;
pub const PROCESSOR_CPU_COUNTERS_INFO_COUNT: mach_msg_type_number_t =
    (mem::size_of::<processor_cpu_counters_info_data_t>() / mem::size_of::<natural_t>())
        as mach_msg_type_number_t;
