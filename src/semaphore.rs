//! This module corresponds to `mach/semaphore.h`

use crate::clock_types::mach_timespec_t;
use crate::kern_return::kern_return_t;
use crate::mach_types::{semaphore_t, task_t};
use crate::sync_policy::sync_policy_t;

extern "C" {
    pub fn semaphore_create(
        task: task_t,
        semaphore: semaphore_t,
        policy: sync_policy_t,
        value: libc::c_int,
    ) -> kern_return_t;
    pub fn semaphore_signal(semaphore: semaphore_t) -> kern_return_t;
    pub fn semaphore_wait(semaphore: semaphore_t) -> kern_return_t;
    pub fn semaphore_timedwait(semaphore: semaphore_t, timeout: mach_timespec_t) -> kern_return_t;
    pub fn semaphore_destroy(task: task_t, semaphore: semaphore_t) -> kern_return_t;
}
