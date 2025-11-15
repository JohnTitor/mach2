//! This module corresponds to `mach/thread_switch.h`.

pub const SWITCH_OPTION_NONE: libc::c_int = 0;
pub const SWITCH_OPTION_DEPRESS: libc::c_int = 1;
pub const SWITCH_OPTION_WAIT: libc::c_int = 2;
