//! This module corresponds to `mach/mach_error.h`.

use crate::error::mach_error_t;
use core::ffi::c_char;

unsafe extern "C" {
    pub fn mach_error_string(error_value: mach_error_t) -> *mut c_char;
    pub fn mach_error(str: *const c_char, error_value: mach_error_t);
    pub fn mach_error_type(error_value: mach_error_t) -> *mut c_char;
}
