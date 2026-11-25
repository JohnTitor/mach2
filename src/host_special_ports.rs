//! This module corresponds to `mach/host_special_ports.h`.

use crate::kern_return::kern_return_t;
use crate::mach_types::host_priv_t;
use crate::port::mach_port_t;
use core::ffi::c_int;

pub const HOST_MAX_SPECIAL_KERNEL_PORT: c_int = 7;
pub const HOST_DISKIMAGESIOD_PORT: c_int = 29 + HOST_MAX_SPECIAL_KERNEL_PORT;
pub const HOST_MAX_SPECIAL_PORT: c_int = HOST_DISKIMAGESIOD_PORT;
pub const HOST_LOCAL_NODE: c_int = -1;

unsafe extern "C" {
    pub fn host_get_special_port(
        host_priv: host_priv_t,
        node: c_int,
        which: c_int,
        port: *mut mach_port_t,
    ) -> kern_return_t;

    pub fn host_set_special_port(
        host_priv: host_priv_t,
        which: c_int,
        port: mach_port_t,
    ) -> kern_return_t;
}

/// # Safety
/// `port` must point to writable storage for a Mach port.
#[inline]
pub unsafe fn host_get_diskimagesiod_port(
    host: host_priv_t,
    port: *mut mach_port_t,
) -> kern_return_t {
    unsafe { host_get_special_port(host, HOST_LOCAL_NODE, HOST_DISKIMAGESIOD_PORT, port) }
}

/// # Safety
/// The caller must own valid rights for `host` and `port`.
#[inline]
pub unsafe fn host_set_diskimagesiod_port(host: host_priv_t, port: mach_port_t) -> kern_return_t {
    unsafe { host_set_special_port(host, HOST_DISKIMAGESIOD_PORT, port) }
}
