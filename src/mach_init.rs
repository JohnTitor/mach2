//! This module corresponds to `mach/mach_init.h`.

use super::mach_types::thread_port_t;

extern "C" {
    pub fn mach_thread_self() -> thread_port_t;
}
