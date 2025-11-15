//! This module corresponds to `mach/thread_state.h`.

use crate::kern_return::kern_return_t;
use crate::mach_types::thread_t;

unsafe extern "C" {
    pub fn thread_get_register_pointer_values(
        thread: thread_t,
        sp: *mut libc::uintptr_t,
        length: *mut libc::size_t,
        values: *mut libc::uintptr_t,
    ) -> kern_return_t;
}
