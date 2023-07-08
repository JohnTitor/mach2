//! This module roughly corresponds to `mach/vm_page_size.h`

use super::vm_types::{mach_vm_offset_t, mach_vm_size_t, vm_size_t};

extern "C" {
    pub static vm_page_size: vm_size_t;
    pub static vm_page_mask: vm_size_t;
    pub static vm_page_shift: ::libc::c_int;
}

#[allow(clippy::missing_safety_doc)] // FIXME
pub unsafe fn mach_vm_trunc_page(x: mach_vm_offset_t) -> mach_vm_offset_t {
    x & !(vm_page_mask as mach_vm_size_t)
}

#[allow(clippy::missing_safety_doc)] // FIXME
pub unsafe fn mach_vm_round_page(x: mach_vm_offset_t) -> mach_vm_offset_t {
    (x + vm_page_mask as mach_vm_size_t) & !(vm_page_mask as mach_vm_size_t)
}
