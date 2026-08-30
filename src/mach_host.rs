//! This module corresponds to `mach/mach_host.defs`.

use crate::kern_return::kern_return_t;
use crate::mach_debug::zone_info::{
    mach_memory_info_array_t, mach_zone_info_array_t, mach_zone_name_array_t,
};
use crate::message::mach_msg_type_number_t;
use crate::port::mach_port_t;

unsafe extern "C" {
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
