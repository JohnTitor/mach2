//! This module roughly corresponds to `libproc.h`.

use super::kern_return::kern_return_t;

pub const PROC_PIDPATHINFO_MAXSIZE: ::libc::c_uint = 4096;

extern "C" {
    pub fn proc_pidpath(pid: ::libc::pid_t, buffer: *mut ::libc::c_void, buffersize: ::libc::c_uint) -> kern_return_t;
    pub fn proc_regionfilename(
        pid: ::libc::pid_t,
        address: ::libc::c_ulong,
        buffer: *mut ::libc::c_void,
        buffersize: ::libc::c_uint,
    ) -> kern_return_t;
}
