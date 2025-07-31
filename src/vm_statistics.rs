//! This module roughly corresponds to `mach/vm_statistics.h`

use vm_types::{integer_t, natural_t};

pub type vm_statistics_t = *mut vm_statistics;
pub type vm_statistics_data_t = vm_statistics;
pub type vm_statistics64_t = *mut vm_statistics64;
pub type vm_statistics64_data_t = vm_statistics64;
pub type vm_extmod_statistics_t = *mut vm_extmod_statistics;
pub type vm_extmod_statistics_data_t = vm_extmod_statistics;
pub type vm_purgeable_stat_t = vm_purgeable_stat;
pub type vm_purgeable_info_t = *mut vm_purgeable_info;

pub const VM_PAGE_QUERY_PAGE_PRESENT: integer_t = 1;
pub const VM_PAGE_QUERY_PAGE_FICTITIOUS: integer_t = 1 << 1;
pub const VM_PAGE_QUERY_PAGE_REF: integer_t = 1 << 2;
pub const VM_PAGE_QUERY_PAGE_DIRTY: integer_t = 1 << 3;

pub const VM_MEMORY_MALLOC: ::libc::c_uint = 1;
pub const VM_MEMORY_MALLOC_SMALL: ::libc::c_uint = 2;
pub const VM_MEMORY_MALLOC_LARGE: ::libc::c_uint = 3;
pub const VM_MEMORY_MALLOC_HUGE: ::libc::c_uint = 4;
pub const VM_MEMORY_SBRK: ::libc::c_uint = 5;
pub const VM_MEMORY_ANALYSIS_TOOL: ::libc::c_uint = 10;
pub const VM_MEMORY_MACH_MSG: ::libc::c_uint = 20;
pub const VM_MEMORY_IOKIT: ::libc::c_uint = 21;
pub const VM_MEMORY_STACK: ::libc::c_uint = 30;
pub const VM_MEMORY_GUARD: ::libc::c_uint = 31;
pub const VM_MEMORY_SHARED_PMAP: ::libc::c_uint = 32;
pub const VM_MEMORY_DYLIB: ::libc::c_uint = 33;
pub const VM_MEMORY_APPKIT: ::libc::c_uint = 40;
pub const VM_MEMORY_FOUNDATION: ::libc::c_uint = 41;
pub const VM_MEMORY_COREGRAPHICS: ::libc::c_uint = 42;
pub const VM_MEMORY_CARBON: ::libc::c_uint = 43;
pub const VM_MEMORY_JAVA: ::libc::c_uint = 44;
pub const VM_MEMORY_ATS: ::libc::c_uint = 50;
pub const VM_MEMORY_DYLD: ::libc::c_uint = 60;
pub const VM_MEMORY_DYLD_MALLOC: ::libc::c_uint = 61;
pub const VM_MEMORY_APPLICATION_SPECIFIC_1: ::libc::c_uint = 240;
pub const VM_MEMORY_APPLICATION_SPECIFIC_16: ::libc::c_uint = 255;

pub const VM_FLAGS_FIXED: ::libc::c_int = 0x0;
pub const VM_FLAGS_ANYWHERE: ::libc::c_int = 0x1;
pub const VM_FLAGS_OVERWRITE: ::libc::c_int = 0x4000;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct vm_statistics {
    pub free_count: natural_t,
    pub active_count: natural_t,
    pub inactive_count: natural_t,
    pub wire_count: natural_t,
    pub zero_fill_count: natural_t,
    pub reactivations: natural_t,
    pub pageins: natural_t,
    pub pageouts: natural_t,
    pub faults: natural_t,
    pub cow_faults: natural_t,
    pub lookups: natural_t,
    pub hits: natural_t,
    pub purgeable_count: natural_t,
    pub purges: natural_t,
    pub speculative_count: natural_t,
}

#[repr(C, packed(8))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct vm_statistics64 {
    pub free_count: natural_t,
    pub active_count: natural_t,
    pub inactive_count: natural_t,
    pub wire_count: natural_t,
    pub zero_fill_count: u64,
    pub reactivations: u64,
    pub pageins: u64,
    pub pageouts: u64,
    pub faults: u64,
    pub cow_faults: u64,
    pub lookups: u64,
    pub hits: u64,
    pub purges: u64,
    pub purgeable_count: natural_t,
    pub speculative_count: natural_t,
    pub decompressions: u64,
    pub compressions: u64,
    pub swapins: u64,
    pub swapouts: u64,
    pub compressor_page_count: natural_t,
    pub throttled_count: natural_t,
    pub external_page_count: natural_t,
    pub internal_page_count: natural_t,
    pub total_uncompressed_pages_in_compressor: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct vm_extmod_statistics {
    pub task_for_pid_count: i64,
    pub task_for_pid_caller_count: i64,
    pub thread_creation_count: i64,
    pub thread_creation_caller_count: i64,
    pub thread_set_state_count: i64,
    pub thread_set_state_caller_count: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct vm_purgeable_stat {
    pub count: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct vm_purgeable_info {
    pub fifo_data: [vm_purgeable_stat_t; 8usize],
    pub obsolete_data: vm_purgeable_stat_t,
    pub lifo_data: [vm_purgeable_stat_t; 8usize],
}
