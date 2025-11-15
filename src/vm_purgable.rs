//! This module corresponds to `mach/vm_purgable.h`.

pub type vm_purgable_t = libc::c_int;

pub const VM_PURGABLE_SET_STATE: vm_purgable_t = 0;
pub const VM_PURGABLE_GET_STATE: vm_purgable_t = 1;
pub const VM_PURGABLE_PURGE_ALL: vm_purgable_t = 2;
pub const VM_PURGABLE_SET_STATE_FROM_KERNEL: vm_purgable_t = 3;

pub const VM_PURGABLE_NO_AGING_SHIFT: libc::c_int = 16;
pub const VM_PURGABLE_NO_AGING_MASK: libc::c_int = 1 << VM_PURGABLE_NO_AGING_SHIFT;
pub const VM_PURGABLE_NO_AGING: libc::c_int = 1 << VM_PURGABLE_NO_AGING_SHIFT;

pub const VM_PURGABLE_DEBUG_SHIFT: libc::c_int = 12;
pub const VM_PURGABLE_DEBUG_MASK: libc::c_int = 0x3 << VM_PURGABLE_DEBUG_SHIFT;
pub const VM_PURGABLE_DEBUG_EMPTY: libc::c_int = 0x1 << VM_PURGABLE_DEBUG_SHIFT;
pub const VM_PURGABLE_DEBUG_FAULT: libc::c_int = 0x2 << VM_PURGABLE_DEBUG_SHIFT;

pub const VM_VOLATILE_GROUP_SHIFT: libc::c_int = 8;
pub const VM_VOLATILE_GROUP_MASK: libc::c_int = 7 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_DEFAULT: libc::c_int = VM_VOLATILE_GROUP_0;

pub const VM_VOLATILE_GROUP_0: libc::c_int = 0 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_1: libc::c_int = 1 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_2: libc::c_int = 2 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_3: libc::c_int = 3 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_4: libc::c_int = 4 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_5: libc::c_int = 5 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_6: libc::c_int = 6 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_7: libc::c_int = 7 << VM_VOLATILE_GROUP_SHIFT;

pub const VM_PURGABLE_BEHAVIOR_SHIFT: libc::c_int = 6;
pub const VM_PURGABLE_BEHAVIOR_MASK: libc::c_int = 1 << VM_PURGABLE_BEHAVIOR_SHIFT;
pub const VM_PURGABLE_BEHAVIOR_FIFO: libc::c_int = 0 << VM_PURGABLE_BEHAVIOR_SHIFT;
pub const VM_PURGABLE_BEHAVIOR_LIFO: libc::c_int = 1 << VM_PURGABLE_BEHAVIOR_SHIFT;

pub const VM_PURGABLE_ORDERING_SHIFT: libc::c_int = 5;
pub const VM_PURGABLE_ORDERING_MASK: libc::c_int = 1 << VM_PURGABLE_ORDERING_SHIFT;
pub const VM_PURGABLE_ORDERING_OBSOLETE: libc::c_int = 1 << VM_PURGABLE_ORDERING_SHIFT;
pub const VM_PURGABLE_ORDERING_NORMAL: libc::c_int = 0 << VM_PURGABLE_ORDERING_SHIFT;

pub const VM_VOLATILE_ORDER_SHIFT: libc::c_int = 4;
pub const VM_VOLATILE_ORDER_MASK: libc::c_int = 1 << VM_VOLATILE_ORDER_SHIFT;
pub const VM_VOLATILE_MAKE_FIRST_IN_GROUP: libc::c_int = 1 << VM_VOLATILE_ORDER_SHIFT;
pub const VM_VOLATILE_MAKE_LAST_IN_GROUP: libc::c_int = 0 << VM_VOLATILE_ORDER_SHIFT;

pub const VM_PURGABLE_STATE_MIN: libc::c_int = 0;
pub const VM_PURGABLE_STATE_MAX: libc::c_int = 3;
pub const VM_PURGABLE_STATE_MASK: libc::c_int = 3;
pub const VM_PURGABLE_NONVOLATILE: libc::c_int = 0;
pub const VM_PURGABLE_VOLATILE: libc::c_int = 1;
pub const VM_PURGABLE_EMPTY: libc::c_int = 2;
pub const VM_PURGABLE_DENY: libc::c_int = 3;
pub const VM_PURGABLE_ALL_MASKS: libc::c_int = VM_PURGABLE_STATE_MASK
    | VM_VOLATILE_ORDER_MASK
    | VM_PURGABLE_ORDERING_MASK
    | VM_PURGABLE_BEHAVIOR_MASK
    | VM_VOLATILE_GROUP_MASK
    | VM_PURGABLE_DEBUG_MASK
    | VM_PURGABLE_NO_AGING_MASK;
