//! This module roughly corresponds to `mach/mach_vm.defs`.

use super::{
    boolean::boolean_t,
    kern_return::kern_return_t,
    mach_types::{mem_entry_name_port_t, vm_task_entry_t},
    memory_object_types::{memory_object_offset_t, memory_object_size_t},
    message::mach_msg_type_number_t,
    port::mach_port_t,
    vm_attributes::{vm_machine_attribute_t, vm_machine_attribute_val_t},
    vm_behavior::vm_behavior_t,
    vm_inherit::vm_inherit_t,
    vm_prot::vm_prot_t,
    vm_purgable::vm_purgable_t,
    vm_region::{
        mach_vm_read_entry_t, vm_page_info_flavor_t, vm_page_info_t, vm_region_flavor_t, vm_region_info_t,
        vm_region_recurse_info_t,
    },
    vm_sync::vm_sync_t,
    vm_types::{
        integer_t, mach_vm_address_t, mach_vm_offset_t, mach_vm_size_t, natural_t, vm_map_t, vm_offset_t, vm_size_t,
    },
};

extern "C" {
    pub fn mach_vm_allocate(
        target: vm_task_entry_t,
        address: *mut mach_vm_address_t,
        size: mach_vm_size_t,
        flags: ::libc::c_int,
    ) -> kern_return_t;

    pub fn mach_vm_deallocate(
        target: vm_task_entry_t,
        address: mach_vm_address_t,
        size: mach_vm_size_t,
    ) -> kern_return_t;

    pub fn mach_vm_protect(
        target_task: vm_task_entry_t,
        address: mach_vm_address_t,
        size: mach_vm_size_t,
        set_maximum: boolean_t,
        new_protection: vm_prot_t,
    ) -> kern_return_t;

    pub fn mach_vm_inherit(
        target_task: vm_task_entry_t,
        address: mach_vm_address_t,
        size: mach_vm_size_t,
        new_inheritance: vm_inherit_t,
    ) -> kern_return_t;

    pub fn mach_vm_read(
        target_task: vm_task_entry_t,
        address: mach_vm_address_t,
        size: mach_vm_size_t,
        data: *mut vm_offset_t,
        dataCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;

    #[allow(improper_ctypes)]
    pub fn mach_vm_read_list(
        target_task: vm_task_entry_t,
        data_list: mach_vm_read_entry_t,
        count: natural_t,
    ) -> kern_return_t;

    pub fn mach_vm_write(
        target_task: vm_map_t,
        address: mach_vm_address_t,
        data: vm_offset_t,
        dataCnt: mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn mach_vm_copy(
        target_task: vm_task_entry_t,
        source_address: mach_vm_address_t,
        size: mach_vm_size_t,
        dest_address: mach_vm_address_t,
    ) -> kern_return_t;

    pub fn mach_vm_read_overwrite(
        target_task: vm_task_entry_t,
        address: mach_vm_address_t,
        size: mach_vm_size_t,
        data: mach_vm_address_t,
        outsize: *mut mach_vm_size_t,
    ) -> kern_return_t;

    pub fn mach_vm_msync(
        target_task: vm_task_entry_t,
        address: mach_vm_address_t,
        size: mach_vm_size_t,
        sync_flags: vm_sync_t,
    ) -> kern_return_t;

    pub fn mach_vm_behavior_set(
        target_task: vm_task_entry_t,
        address: mach_vm_address_t,
        size: mach_vm_size_t,
        new_behavior: vm_behavior_t,
    ) -> kern_return_t;

    pub fn mach_vm_map(
        target_task: vm_task_entry_t,
        inout: *mut mach_vm_address_t,
        size: mach_vm_size_t,
        mask: mach_vm_offset_t,
        flags: ::libc::c_int,
        object: mem_entry_name_port_t,
        offset: memory_object_offset_t,
        copy: boolean_t,
        cur_protection: vm_prot_t,
        max_protection: vm_prot_t,
        inheritance: vm_inherit_t,
    ) -> kern_return_t;

    pub fn mach_vm_machine_attribute(
        target_task: vm_task_entry_t,
        address: mach_vm_address_t,
        size: mach_vm_size_t,
        attribute: vm_machine_attribute_t,
        value: *mut vm_machine_attribute_val_t,
    ) -> kern_return_t;

    pub fn mach_vm_remap(
        target_task: vm_task_entry_t,
        target_address: *mut mach_vm_address_t,
        size: mach_vm_size_t,
        mask: mach_vm_offset_t,
        flags: ::libc::c_int,
        src_task: vm_task_entry_t,
        src_address: mach_vm_address_t,
        copy: boolean_t,
        cur_protection: *mut vm_prot_t,
        out: *mut vm_prot_t,
        inheritance: vm_inherit_t,
    ) -> kern_return_t;

    pub fn mach_vm_page_query(
        target_map: vm_map_t,
        offset: mach_vm_offset_t,
        disposition: *mut integer_t,
        ref_count: *mut integer_t,
    ) -> kern_return_t;

    pub fn mach_vm_region_recurse(
        target_task: vm_task_entry_t,
        address: *mut mach_vm_address_t,
        size: *mut mach_vm_size_t,
        nesting_depth: *mut natural_t,
        info: vm_region_recurse_info_t,
        infoCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn mach_vm_region(
        target_task: vm_task_entry_t,
        address: *mut mach_vm_address_t,
        size: *mut mach_vm_size_t,
        flavor: vm_region_flavor_t,
        info: vm_region_info_t,
        infoCnt: *mut mach_msg_type_number_t,
        object_name: *mut mach_port_t,
    ) -> kern_return_t;

    pub fn mach_make_memory_entry(
        target_task: vm_map_t,
        size: *mut vm_size_t,
        offset: vm_offset_t,
        permission: vm_prot_t,
        object_handle: *mut mem_entry_name_port_t,
        parent_handle: mem_entry_name_port_t,
    ) -> kern_return_t;

    pub fn mach_make_memory_entry_64(
        target_task: vm_map_t,
        size: *mut memory_object_size_t,
        offset: memory_object_offset_t,
        permission: vm_prot_t,
        object_handle: *mut mach_port_t,
        parent_entry: mem_entry_name_port_t,
    ) -> kern_return_t;

    pub fn mach_vm_purgable_control(
        target_task: vm_task_entry_t,
        address: mach_vm_address_t,
        control: vm_purgable_t,
        state: *mut ::libc::c_int,
    ) -> kern_return_t;

    pub fn mach_vm_page_info(
        target_task: vm_task_entry_t,
        address: mach_vm_address_t,
        flavor: vm_page_info_flavor_t,
        info: vm_page_info_t,
        infoCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
}

pub type arm_thread_state32_t = __darwin_arm_thread_state;
pub type arm_thread_state64_t = __darwin_arm_thread_state64;
pub type arm_state_hdr_t = arm_state_hdr;
pub type arm_unified_thread_state_t = arm_unified_thread_state;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct arm_state_hdr {
    pub flavor: u32,
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_unified_thread_state {
    pub ash: arm_state_hdr_t,
    pub uts: arm_unified_thread_state__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union arm_unified_thread_state__bindgen_ty_1 {
    pub ts_32: arm_thread_state32_t,
    pub ts_64: arm_thread_state64_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_thread_state {
    pub __r: [u32; 13usize],
    pub __sp: u32,
    pub __lr: u32,
    pub __pc: u32,
    pub __cpsr: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_thread_state64 {
    pub __x: [u64; 29usize],
    pub __fp: u64,
    pub __lr: u64,
    pub __sp: u64,
    pub __pc: u64,
    pub __cpsr: u32,
    pub __pad: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{kern_return::KERN_SUCCESS, traps::mach_task_self, vm_statistics::VM_FLAGS_ANYWHERE};

    #[test]
    fn mach_vm_allocate_sanity() {
        unsafe {
            let size = 0x100;
            let task = mach_task_self();

            let mut address: mach_vm_address_t = 0;
            assert_eq!(mach_vm_allocate(task, &mut address, size, VM_FLAGS_ANYWHERE), KERN_SUCCESS);
            assert_eq!(mach_vm_deallocate(task, address, size), KERN_SUCCESS);
        }
    }

    #[test]
    fn mach_vm_region_sanity() {
        use core::mem;

        use crate::{
            vm_prot::{VM_PROT_EXECUTE, VM_PROT_READ},
            vm_region::{vm_region_basic_info_64, VM_REGION_BASIC_INFO_64},
        };
        unsafe {
            let mut size = 0x10;
            let mut object_name = 0;
            #[allow(clippy::fn_to_numeric_cast)]
            let mut address = mach_vm_region_sanity as mach_vm_address_t;
            let mut info: vm_region_basic_info_64 = mem::zeroed();
            let mut info_size = vm_region_basic_info_64::count();

            let result = mach_vm_region(
                mach_task_self(),
                &mut address,
                &mut size,
                VM_REGION_BASIC_INFO_64,
                (&mut info as *mut _) as vm_region_info_t,
                &mut info_size,
                &mut object_name,
            );
            assert_eq!(result, KERN_SUCCESS);
            assert_eq!(info.protection, VM_PROT_READ | VM_PROT_EXECUTE);
        }
    }
}
