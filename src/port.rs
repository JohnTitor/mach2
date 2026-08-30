//! This module corresponds to `mach/port.h`

use crate::vm_types::{integer_t, natural_t};
use core::ffi::{c_char, c_int, c_uint};
use core::mem;

pub type mach_port_name_t = natural_t;

pub type mach_port_t = c_uint;
pub type mach_port_array_t = *mut mach_port_t;

pub const MACH_PORT_NULL: mach_port_t = 0;
pub const MACH_PORT_DEAD: mach_port_t = !0;

pub type mach_port_right_t = natural_t;

pub const MACH_PORT_RIGHT_SEND: mach_port_right_t = 0;
pub const MACH_PORT_RIGHT_RECEIVE: mach_port_right_t = 1;
pub const MACH_PORT_RIGHT_SEND_ONCE: mach_port_right_t = 2;
pub const MACH_PORT_RIGHT_PORT_SET: mach_port_right_t = 3;
pub const MACH_PORT_RIGHT_DEAD_NAME: mach_port_right_t = 4;
pub const MACH_PORT_RIGHT_LABELH: mach_port_right_t = 5;
pub const MACH_PORT_RIGHT_NUMBER: mach_port_right_t = 6;

pub type mach_port_type_t = natural_t;

pub type mach_port_urefs_t = natural_t;
pub type mach_port_delta_t = integer_t;

pub type mach_port_seqno_t = natural_t;
pub type mach_port_mscount_t = natural_t;
pub type mach_port_msgcount_t = natural_t;
pub type mach_port_rights_t = natural_t;

pub const MACH_PORT_QLIMIT_ZERO: mach_port_msgcount_t = 0;
pub const MACH_PORT_QLIMIT_BASIC: mach_port_msgcount_t = 5;
pub const MACH_PORT_QLIMIT_SMALL: mach_port_msgcount_t = 16;
pub const MACH_PORT_QLIMIT_LARGE: mach_port_msgcount_t = 1024;
pub const MACH_PORT_QLIMIT_KERNEL: mach_port_msgcount_t = 65534;
pub const MACH_PORT_QLIMIT_MIN: mach_port_msgcount_t = MACH_PORT_QLIMIT_ZERO;
pub const MACH_PORT_QLIMIT_DEFAULT: mach_port_msgcount_t = MACH_PORT_QLIMIT_BASIC;
pub const MACH_PORT_QLIMIT_MAX: mach_port_msgcount_t = MACH_PORT_QLIMIT_LARGE;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct mach_port_limits_t {
    pub mpl_qlimit: mach_port_msgcount_t,
}

pub type mach_port_info_t = *mut integer_t;

/* Flavors for mach_port_get/set/assert_attributes() */
pub type mach_port_flavor_t = c_int;

pub const MACH_PORT_LIMITS_INFO: mach_port_flavor_t = 1;
pub const MACH_PORT_RECEIVE_STATUS: mach_port_flavor_t = 2;
pub const MACH_PORT_DNREQUESTS_SIZE: mach_port_flavor_t = 3;
pub const MACH_PORT_TEMPOWNER: mach_port_flavor_t = 4;
pub const MACH_PORT_IMPORTANCE_RECEIVER: mach_port_flavor_t = 5;
pub const MACH_PORT_DENAP_RECEIVER: mach_port_flavor_t = 6;
pub const MACH_PORT_INFO_EXT: mach_port_flavor_t = 7;
pub const MACH_PORT_GUARD_INFO: mach_port_flavor_t = 8;
pub const MACH_PORT_SERVICE_THROTTLED: mach_port_flavor_t = 9;

pub const MACH_PORT_LIMITS_INFO_COUNT: natural_t =
    (mem::size_of::<mach_port_limits_t>() / mem::size_of::<natural_t>()) as natural_t;
pub const MACH_PORT_DNREQUESTS_SIZE_COUNT: natural_t = 1;
pub const MACH_PORT_SERVICE_THROTTLED_COUNT: natural_t = 1;

pub const MPO_CONTEXT_AS_GUARD: u32 = 1;
pub const MPO_QLIMIT: u32 = 2;
pub const MPO_TEMPOWNER: u32 = 4;
pub const MPO_IMPORTANCE_RECEIVER: u32 = 8;
pub const MPO_INSERT_SEND_RIGHT: u32 = 0x10;
pub const MPO_STRICT: u32 = 0x20;
pub const MPO_DENAP_RECEIVER: u32 = 0x40;
pub const MPO_IMMOVABLE_RECEIVE: u32 = 0x80;

pub const MACH_PORT_WEAK_REPLY_ENTITLEMENT: *const c_char =
    c"com.apple.private.allow-weak-reply-port".as_ptr();

pub type mpo_flags_t = u32;
pub const MPO_WEAK_REPLY_PORT: mpo_flags_t = 0x4000;
pub const MPO_NOTIFICATION_PORT: mpo_flags_t = 0x4400;

pub const kGUARD_EXC_WEAK_REPLY_PORT: u32 = 0x0010_0002;
pub const kGUARD_EXC_MOVE_WEAK_REPLY_PORT: u32 = 0x0010_0004;
pub const kGUARD_EXC_INVALID_NOTIFICATION_PORT: u32 = 0x0010_0006;
pub const kGUARD_EXC_MACH_EXC_THREAD_SET_STATE: u32 = 0x0010_0007;
pub const kGUARD_EXC_CV_NOTIFICATION_PORT_REQ: u32 = 0x0010_0008;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct mach_port_options_t {
    pub flags: u32,
    pub mpl: mach_port_limits_t,
    pub reserved: [u64; 2],
}
