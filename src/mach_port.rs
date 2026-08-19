//! This module corresponds to `mach/mach_port.h`

use crate::kern_return::kern_return_t;
use crate::mach_types::ipc_space_t;
use crate::message::{mach_msg_id_t, mach_msg_type_name_t, mach_msg_type_number_t};
use crate::port::{
    mach_port_delta_t, mach_port_flavor_t, mach_port_info_t, mach_port_mscount_t, mach_port_name_t,
    mach_port_options_t, mach_port_right_t, mach_port_t,
};
use crate::vm_types::mach_port_context_t;

unsafe extern "C" {
    pub fn mach_port_allocate(
        task: ipc_space_t,
        right: mach_port_right_t,
        name: *mut mach_port_name_t,
    ) -> kern_return_t;
    pub fn mach_port_destroy(task: ipc_space_t, name: mach_port_name_t) -> kern_return_t;
    pub fn mach_port_deallocate(task: ipc_space_t, name: mach_port_name_t) -> kern_return_t;
    pub fn mach_port_insert_right(
        task: ipc_space_t,
        name: mach_port_name_t,
        poly: mach_port_t,
        polyPoly: mach_msg_type_name_t,
    ) -> kern_return_t;
    pub fn mach_port_extract_right(
        task: ipc_space_t,
        name: mach_port_name_t,
        msgt_name: mach_msg_type_name_t,
        poly: *mut mach_port_t,
        polyPoly: *mut mach_msg_type_name_t,
    ) -> kern_return_t;
    pub fn mach_port_mod_refs(
        task: ipc_space_t,
        name: mach_port_name_t,
        right: mach_port_right_t,
        delta: mach_port_delta_t,
    ) -> kern_return_t;
    pub fn mach_port_get_attributes(
        task: ipc_space_t,
        name: mach_port_name_t,
        flavor: mach_port_flavor_t,
        port_info_out: mach_port_info_t,
        port_info_outCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn mach_port_set_attributes(
        task: ipc_space_t,
        name: mach_port_name_t,
        flavor: mach_port_flavor_t,
        port_info: mach_port_info_t,
        port_infoCnt: mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn mach_port_request_notification(
        task: ipc_space_t,
        name: mach_port_name_t,
        msgid: mach_msg_id_t,
        sync: mach_port_mscount_t,
        notify: mach_port_t,
        notifyPoly: mach_msg_type_name_t,
        previous: *mut mach_port_t,
    ) -> kern_return_t;
    pub fn mach_port_construct(
        task: ipc_space_t,
        options: *mut mach_port_options_t,
        context: mach_port_context_t,
        name: *mut mach_port_name_t,
    ) -> kern_return_t;
    pub fn mach_port_destruct(
        task: ipc_space_t,
        name: mach_port_name_t,
        srdelta: mach_port_delta_t,
        guard: mach_port_context_t,
    ) -> kern_return_t;
}
