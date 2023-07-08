//! This module roughly corresponds to `mach/mach_error.h`.

use super::kern_return::kern_return_t;

pub type mach_error_t = kern_return_t;

extern "C" {
    pub fn mach_error_string(error_value: mach_error_t) -> *mut ::libc::c_char;
}
