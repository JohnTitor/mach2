//! This module corresponds to `mach/mach_host.defs`.

use crate::host_info::{host_flavor_t, host_info64_t};
use crate::kern_return::kern_return_t;
use crate::mach_debug::zone_info::{
    mach_memory_info_array_t, mach_zone_info_array_t, mach_zone_name_array_t,
};
use crate::mach_types::host_t;
use crate::message::mach_msg_type_number_t;
use crate::port::mach_port_t;
use crate::processor_info::{processor_flavor_t, processor_info_array_t};
use crate::vm_types::natural_t;

unsafe extern "C" {
    pub fn host_statistics64(
        host_priv: host_t,
        flavor: host_flavor_t,
        host_info64_out: host_info64_t,
        host_info64_outCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn host_processor_info(
        host: host_t,
        flavor: processor_flavor_t,
        out_processor_count: *mut natural_t,
        out_processor_info: *mut processor_info_array_t,
        out_processor_infoCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn mach_memory_info_redacted(
        host: mach_port_t,
        names: *mut mach_zone_name_array_t,
        namesCnt: *mut mach_msg_type_number_t,
        info: *mut mach_zone_info_array_t,
        infoCnt: *mut mach_msg_type_number_t,
        memory_info: *mut mach_memory_info_array_t,
        memory_infoCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
}
