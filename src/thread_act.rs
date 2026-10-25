//! This module corresponds to `mach/thread_act.defs`.

use crate::exception_types::{
    exception_behavior_array_t, exception_behavior_t, exception_flavor_array_t,
    exception_mask_array_t, exception_mask_t,
};
use crate::kern_return::kern_return_t;
use crate::mach_types::{
    exception_handler_array_t, thread_act_t, thread_inspect_t, thread_port_t, thread_read_t,
};
use crate::message::mach_msg_type_number_t;
use crate::port::mach_port_t;
use crate::thread_status::{thread_state_flavor_t, thread_state_t};
use core::ffi::c_int;

unsafe extern "C" {
    pub fn thread_terminate(target_act: thread_act_t) -> kern_return_t;

    pub fn thread_get_state(
        target_act: thread_act_t,
        flavor: thread_state_flavor_t,
        old_state: thread_state_t,
        old_state_count: *mut mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn thread_set_state(
        target_act: thread_port_t,
        flavor: thread_state_flavor_t,
        new_state: thread_state_t,
        new_stateCnt: mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn thread_set_exception_ports(
        thread: thread_port_t,
        exception_mask: exception_mask_t,
        new_port: mach_port_t,
        behavior: exception_behavior_t,
        new_flavor: thread_state_flavor_t,
    ) -> kern_return_t;

    pub fn thread_get_exception_ports(
        thread: thread_act_t,
        exception_mask: exception_mask_t,
        masks: exception_mask_array_t,
        masksCnt: *mut mach_msg_type_number_t,
        old_handlers: exception_handler_array_t,
        old_behaviors: exception_behavior_array_t,
        old_flavors: exception_flavor_array_t,
    ) -> kern_return_t;

    pub fn thread_swap_exception_ports(
        thread: thread_act_t,
        exception_mask: exception_mask_t,
        new_port: mach_port_t,
        behavior: exception_behavior_t,
        new_flavor: thread_state_flavor_t,
        masks: exception_mask_array_t,
        masksCnt: *mut mach_msg_type_number_t,
        old_handlers: exception_handler_array_t,
        old_behaviors: exception_behavior_array_t,
        old_flavors: exception_flavor_array_t,
    ) -> kern_return_t;

    pub fn thread_suspend(target_act: thread_act_t) -> kern_return_t;

    pub fn thread_resume(target_act: thread_act_t) -> kern_return_t;

    pub fn thread_abort(target_act: thread_act_t) -> kern_return_t;

    pub fn thread_abort_safely(target_act: thread_act_t) -> kern_return_t;

    pub fn thread_get_special_port(
        thr_act: thread_inspect_t,
        which_port: c_int,
        special_port: *mut mach_port_t,
    ) -> kern_return_t;

    pub fn thread_set_special_port(
        thr_act: thread_act_t,
        which_port: c_int,
        special_port: mach_port_t,
    ) -> kern_return_t;

    pub fn thread_suspend2(
        target_act: thread_read_t,
        suspend_token: *mut mach_port_t,
    ) -> kern_return_t;

    pub fn thread_resume2(suspend_token: mach_port_t) -> kern_return_t;
}
