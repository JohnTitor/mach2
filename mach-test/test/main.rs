#![allow(bad_style)]

#[allow(unused_imports)]
use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort};
use mach2::boolean::*;
use mach2::bootstrap::*;
use mach2::clock::*;
use mach2::clock_priv::*;
use mach2::clock_reply::*;
use mach2::clock_types::*;
use mach2::dyld::*;
use mach2::dyld_kernel::*;
use mach2::error::*;
use mach2::exc::*;
use mach2::exception_types::*;
use mach2::kern_return::*;
use mach2::loader::*;
use mach2::mach_debug::ipc_info::*;
use mach2::mach_debug::zone_info::*;
use mach2::mach_init::*;
use mach2::mach_port::*;
use mach2::mach_time::*;
use mach2::mach_types::*;
use mach2::mach_voucher_types::*;
use mach2::memory_object_types::*;
use mach2::message::*;
use mach2::mig::*;
use mach2::ndr::*;
use mach2::notify::*;
use mach2::policy::*;
use mach2::port::*;
// FIXME: semaphore is not used => not tested?
#[allow(unused_imports)]
use mach2::semaphore::*;
use mach2::structs::*;
use mach2::sync_policy::*;
use mach2::task::*;
use mach2::task_info::*;
use mach2::task_inspect::*;
use mach2::task_special_ports::*;
use mach2::thread_act::*;
use mach2::thread_policy::*;
use mach2::thread_special_ports::*;
use mach2::thread_state::*;
use mach2::thread_status::*;
use mach2::thread_switch::*;
use mach2::time_value::*;
use mach2::traps::*;
use mach2::vm::*;
use mach2::vm_attributes::*;
use mach2::vm_behavior::*;
use mach2::vm_inherit::*;
// FIXME: vm_page_size is not used => not tested?
#[allow(unused_imports)]
use mach2::vm_page_size::*;
use mach2::vm_prot::*;
use mach2::vm_purgable::*;
use mach2::vm_region::*;
use mach2::vm_statistics::*;
use mach2::vm_sync::*;
use mach2::vm_types::*;

// Imported by mach, but kept private:
unsafe extern "C" {
    static mach_task_self_: mach_port_t;
}

include!(concat!(env!("OUT_DIR"), "/all.rs"));
