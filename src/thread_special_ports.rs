//! This module corresponds to `mach/thread_special_ports.h`.

pub const THREAD_KERNEL_PORT: libc::c_int = 1;
pub const THREAD_INSPECT_PORT: libc::c_int = 2;
pub const THREAD_READ_PORT: libc::c_int = 3;
pub const THREAD_MAX_SPECIAL_PORT: libc::c_int = THREAD_READ_PORT;
