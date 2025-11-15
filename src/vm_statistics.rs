//! This module roughly corresponds to `mach/vm_statistics.h`

use crate::vm_types::{integer_t, natural_t};

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
pub const VM_PAGE_QUERY_PAGE_PAGED_OUT: integer_t = 1 << 4;
pub const VM_PAGE_QUERY_PAGE_COPIED: integer_t = 1 << 5;
pub const VM_PAGE_QUERY_PAGE_SPECULATIVE: integer_t = 1 << 6;
pub const VM_PAGE_QUERY_PAGE_EXTERNAL: integer_t = 1 << 7;
pub const VM_PAGE_QUERY_PAGE_CS_VALIDATED: integer_t = 1 << 8;
pub const VM_PAGE_QUERY_PAGE_CS_TAINTED: integer_t = 1 << 9;
pub const VM_PAGE_QUERY_PAGE_CS_NX: integer_t = 1 << 10;
pub const VM_PAGE_QUERY_PAGE_REUSABLE: integer_t = 1 << 11;

pub const VM_MEMORY_MALLOC: libc::c_uint = 1;
pub const VM_MEMORY_MALLOC_SMALL: libc::c_uint = 2;
pub const VM_MEMORY_MALLOC_LARGE: libc::c_uint = 3;
pub const VM_MEMORY_MALLOC_HUGE: libc::c_uint = 4;
pub const VM_MEMORY_SBRK: libc::c_uint = 5;
pub const VM_MEMORY_REALLOC: libc::c_uint = 6;
pub const VM_MEMORY_MALLOC_TINY: libc::c_uint = 7;
pub const VM_MEMORY_MALLOC_LARGE_REUSABLE: libc::c_uint = 8;
pub const VM_MEMORY_MALLOC_LARGE_REUSED: libc::c_uint = 9;
pub const VM_MEMORY_ANALYSIS_TOOL: libc::c_uint = 10;
pub const VM_MEMORY_MALLOC_NANO: libc::c_uint = 11;
pub const VM_MEMORY_MALLOC_MEDIUM: libc::c_uint = 12;
pub const VM_MEMORY_MALLOC_PROB_GUARD: libc::c_uint = 13;
pub const VM_MEMORY_MACH_MSG: libc::c_uint = 20;
pub const VM_MEMORY_IOKIT: libc::c_uint = 21;
pub const VM_MEMORY_STACK: libc::c_uint = 30;
pub const VM_MEMORY_GUARD: libc::c_uint = 31;
pub const VM_MEMORY_SHARED_PMAP: libc::c_uint = 32;
pub const VM_MEMORY_DYLIB: libc::c_uint = 33;
pub const VM_MEMORY_OBJC_DISPATCHERS: libc::c_uint = 34;
pub const VM_MEMORY_UNSHARED_PMAP: libc::c_uint = 35;
pub const VM_MEMORY_LIBCHANNEL: libc::c_uint = 36;
pub const VM_MEMORY_APPKIT: libc::c_uint = 40;
pub const VM_MEMORY_FOUNDATION: libc::c_uint = 41;
pub const VM_MEMORY_COREGRAPHICS: libc::c_uint = 42;
pub const VM_MEMORY_CORESERVICES: libc::c_uint = 43;
pub const VM_MEMORY_CARBON: libc::c_uint = VM_MEMORY_CORESERVICES;
pub const VM_MEMORY_JAVA: libc::c_uint = 44;
pub const VM_MEMORY_COREDATA: libc::c_uint = 45;
pub const VM_MEMORY_COREDATA_OBJECTIDS: libc::c_uint = 46;
pub const VM_MEMORY_ATS: libc::c_uint = 50;
pub const VM_MEMORY_LAYERKIT: libc::c_uint = 51;
pub const VM_MEMORY_CGIMAGE: libc::c_uint = 52;
pub const VM_MEMORY_TCMALLOC: libc::c_uint = 53;
pub const VM_MEMORY_COREGRAPHICS_DATA: libc::c_uint = 54;
pub const VM_MEMORY_COREGRAPHICS_SHARED: libc::c_uint = 55;
pub const VM_MEMORY_COREGRAPHICS_FRAMEBUFFERS: libc::c_uint = 56;
pub const VM_MEMORY_COREGRAPHICS_BACKINGSTORES: libc::c_uint = 57;
pub const VM_MEMORY_COREGRAPHICS_XALLOC: libc::c_uint = 58;
pub const VM_MEMORY_COREGRAPHICS_MISC: libc::c_uint = VM_MEMORY_COREGRAPHICS;
pub const VM_MEMORY_DYLD: libc::c_uint = 60;
pub const VM_MEMORY_DYLD_MALLOC: libc::c_uint = 61;
pub const VM_MEMORY_SQLITE: libc::c_uint = 62;
pub const VM_MEMORY_JAVASCRIPT_CORE: libc::c_uint = 63;
pub const VM_MEMORY_WEBASSEMBLY: libc::c_uint = VM_MEMORY_JAVASCRIPT_CORE;
pub const VM_MEMORY_JAVASCRIPT_JIT_EXECUTABLE_ALLOCATOR: libc::c_uint = 64;
pub const VM_MEMORY_JAVASCRIPT_JIT_REGISTER_FILE: libc::c_uint = 65;
pub const VM_MEMORY_GLSL: libc::c_uint = 66;
pub const VM_MEMORY_OPENCL: libc::c_uint = 67;
pub const VM_MEMORY_COREIMAGE: libc::c_uint = 68;
pub const VM_MEMORY_WEBCORE_PURGEABLE_BUFFERS: libc::c_uint = 69;
pub const VM_MEMORY_IMAGEIO: libc::c_uint = 70;
pub const VM_MEMORY_COREPROFILE: libc::c_uint = 71;
pub const VM_MEMORY_ASSETSD: libc::c_uint = 72;
pub const VM_MEMORY_OS_ALLOC_ONCE: libc::c_uint = 73;
pub const VM_MEMORY_LIBDISPATCH: libc::c_uint = 74;
pub const VM_MEMORY_ACCELERATE: libc::c_uint = 75;
pub const VM_MEMORY_COREUI: libc::c_uint = 76;
pub const VM_MEMORY_COREUIFILE: libc::c_uint = 77;
pub const VM_MEMORY_GENEALOGY: libc::c_uint = 78;
pub const VM_MEMORY_RAWCAMERA: libc::c_uint = 79;
pub const VM_MEMORY_CORPSEINFO: libc::c_uint = 80;
pub const VM_MEMORY_ASL: libc::c_uint = 81;
pub const VM_MEMORY_SWIFT_RUNTIME: libc::c_uint = 82;
pub const VM_MEMORY_SWIFT_METADATA: libc::c_uint = 83;
pub const VM_MEMORY_DHMM: libc::c_uint = 84;
pub const VM_MEMORY_DFR: libc::c_uint = 85;
pub const VM_MEMORY_SCENEKIT: libc::c_uint = 86;
pub const VM_MEMORY_SKYWALK: libc::c_uint = 87;
pub const VM_MEMORY_IOSURFACE: libc::c_uint = 88;
pub const VM_MEMORY_LIBNETWORK: libc::c_uint = 89;
pub const VM_MEMORY_AUDIO: libc::c_uint = 90;
pub const VM_MEMORY_VIDEOBITSTREAM: libc::c_uint = 91;
pub const VM_MEMORY_CM_XPC: libc::c_uint = 92;
pub const VM_MEMORY_CM_RPC: libc::c_uint = 93;
pub const VM_MEMORY_CM_MEMORYPOOL: libc::c_uint = 94;
pub const VM_MEMORY_CM_READCACHE: libc::c_uint = 95;
pub const VM_MEMORY_CM_CRABS: libc::c_uint = 96;
pub const VM_MEMORY_QUICKLOOK_THUMBNAILS: libc::c_uint = 97;
pub const VM_MEMORY_ACCOUNTS: libc::c_uint = 98;
pub const VM_MEMORY_SANITIZER: libc::c_uint = 99;
pub const VM_MEMORY_IOACCELERATOR: libc::c_uint = 100;
pub const VM_MEMORY_CM_REGWARP: libc::c_uint = 101;
pub const VM_MEMORY_EAR_DECODER: libc::c_uint = 102;
pub const VM_MEMORY_COREUI_CACHED_IMAGE_DATA: libc::c_uint = 103;
pub const VM_MEMORY_COLORSYNC: libc::c_uint = 104;
pub const VM_MEMORY_BTINFO: libc::c_uint = 105;
pub const VM_MEMORY_CM_HLS: libc::c_uint = 106;
pub const VM_MEMORY_COMPOSITOR_SERVICES: libc::c_uint = 107;
pub const VM_MEMORY_ROSETTA: libc::c_uint = 230;
pub const VM_MEMORY_ROSETTA_THREAD_CONTEXT: libc::c_uint = 231;
pub const VM_MEMORY_ROSETTA_INDIRECT_BRANCH_MAP: libc::c_uint = 232;
pub const VM_MEMORY_ROSETTA_RETURN_STACK: libc::c_uint = 233;
pub const VM_MEMORY_ROSETTA_EXECUTABLE_HEAP: libc::c_uint = 234;
pub const VM_MEMORY_ROSETTA_USER_LDT: libc::c_uint = 235;
pub const VM_MEMORY_ROSETTA_ARENA: libc::c_uint = 236;
pub const VM_MEMORY_ROSETTA_10: libc::c_uint = 239;
pub const VM_MEMORY_APPLICATION_SPECIFIC_1: libc::c_uint = 240;
pub const VM_MEMORY_APPLICATION_SPECIFIC_2: libc::c_uint = 241;
pub const VM_MEMORY_APPLICATION_SPECIFIC_3: libc::c_uint = 242;
pub const VM_MEMORY_APPLICATION_SPECIFIC_4: libc::c_uint = 243;
pub const VM_MEMORY_APPLICATION_SPECIFIC_5: libc::c_uint = 244;
pub const VM_MEMORY_APPLICATION_SPECIFIC_6: libc::c_uint = 245;
pub const VM_MEMORY_APPLICATION_SPECIFIC_7: libc::c_uint = 246;
pub const VM_MEMORY_APPLICATION_SPECIFIC_8: libc::c_uint = 247;
pub const VM_MEMORY_APPLICATION_SPECIFIC_9: libc::c_uint = 248;
pub const VM_MEMORY_APPLICATION_SPECIFIC_10: libc::c_uint = 249;
pub const VM_MEMORY_APPLICATION_SPECIFIC_11: libc::c_uint = 250;
pub const VM_MEMORY_APPLICATION_SPECIFIC_12: libc::c_uint = 251;
pub const VM_MEMORY_APPLICATION_SPECIFIC_13: libc::c_uint = 252;
pub const VM_MEMORY_APPLICATION_SPECIFIC_14: libc::c_uint = 253;
pub const VM_MEMORY_APPLICATION_SPECIFIC_15: libc::c_uint = 254;
pub const VM_MEMORY_APPLICATION_SPECIFIC_16: libc::c_uint = 255;
pub const VM_MEMORY_COUNT: libc::c_uint = 256;

#[inline]
pub const fn vm_make_tag(tag: libc::c_uint) -> libc::c_int {
    (tag as libc::c_int) << 24
}

pub const VM_FLAGS_FIXED: libc::c_int = 0x0;
pub const VM_FLAGS_ANYWHERE: libc::c_int = 0x1;
pub const VM_FLAGS_PURGABLE: libc::c_int = 0x2;
pub const VM_FLAGS_4GB_CHUNK: libc::c_int = 0x4;
pub const VM_FLAGS_RANDOM_ADDR: libc::c_int = 0x8;
pub const VM_FLAGS_NO_CACHE: libc::c_int = 0x10;
pub const VM_FLAGS_RESILIENT_CODESIGN: libc::c_int = 0x20;
pub const VM_FLAGS_RESILIENT_MEDIA: libc::c_int = 0x40;
pub const VM_FLAGS_PERMANENT: libc::c_int = 0x80;
pub const VM_FLAGS_TPRO: libc::c_int = 0x1000;
pub const VM_FLAGS_MTE: libc::c_int = 0x2000;
pub const VM_FLAGS_OVERWRITE: libc::c_int = 0x4000;
pub const VM_FLAGS_SUPERPAGE_MASK: libc::c_int = 0x0007_0000;
pub const VM_FLAGS_RETURN_DATA_ADDR: libc::c_int = 0x0010_0000;
pub const VM_FLAGS_RETURN_4K_DATA_ADDR: libc::c_int = 0x0080_0000;
pub const VM_FLAGS_ALIAS_MASK: libc::c_int = -16_777_216; // 0xFF000000

pub const VM_FLAGS_HW: libc::c_int = VM_FLAGS_TPRO | VM_FLAGS_MTE;

pub const VM_FLAGS_USER_ALLOCATE: libc::c_int = VM_FLAGS_FIXED
    | VM_FLAGS_ANYWHERE
    | VM_FLAGS_PURGABLE
    | VM_FLAGS_4GB_CHUNK
    | VM_FLAGS_RANDOM_ADDR
    | VM_FLAGS_NO_CACHE
    | VM_FLAGS_PERMANENT
    | VM_FLAGS_OVERWRITE
    | VM_FLAGS_SUPERPAGE_MASK
    | VM_FLAGS_HW
    | VM_FLAGS_ALIAS_MASK;
pub const VM_FLAGS_USER_MAP: libc::c_int =
    VM_FLAGS_USER_ALLOCATE | VM_FLAGS_RETURN_4K_DATA_ADDR | VM_FLAGS_RETURN_DATA_ADDR;
pub const VM_FLAGS_USER_REMAP: libc::c_int = VM_FLAGS_FIXED
    | VM_FLAGS_ANYWHERE
    | VM_FLAGS_RANDOM_ADDR
    | VM_FLAGS_OVERWRITE
    | VM_FLAGS_RETURN_DATA_ADDR
    | VM_FLAGS_RESILIENT_CODESIGN
    | VM_FLAGS_RESILIENT_MEDIA;

pub const VM_FLAGS_SUPERPAGE_SHIFT: libc::c_int = 16;
pub const SUPERPAGE_NONE: libc::c_int = 0;
pub const SUPERPAGE_SIZE_ANY: libc::c_int = 1;
pub const VM_FLAGS_SUPERPAGE_NONE: libc::c_int = SUPERPAGE_NONE << VM_FLAGS_SUPERPAGE_SHIFT;
pub const VM_FLAGS_SUPERPAGE_SIZE_ANY: libc::c_int = SUPERPAGE_SIZE_ANY << VM_FLAGS_SUPERPAGE_SHIFT;
#[cfg(target_arch = "x86_64")]
pub const SUPERPAGE_SIZE_2MB: libc::c_int = 2;
#[cfg(target_arch = "x86_64")]
pub const VM_FLAGS_SUPERPAGE_SIZE_2MB: libc::c_int = SUPERPAGE_SIZE_2MB << VM_FLAGS_SUPERPAGE_SHIFT;

pub const GUARD_TYPE_VIRT_MEMORY: u32 = 5;

pub type virtual_memory_guard_exception_code_t = u32;
pub const kGUARD_EXC_DEALLOC_GAP: u32 = 1;
pub const kGUARD_EXC_RECLAIM_COPYIO_FAILURE: u32 = 2;
pub const kGUARD_EXC_RECLAIM_INDEX_FAILURE: u32 = 4;
pub const kGUARD_EXC_RECLAIM_DEALLOCATE_FAILURE: u32 = 8;
pub const kGUARD_EXC_RECLAIM_ACCOUNTING_FAILURE: u32 = 9;
pub const kGUARD_EXC_SEC_IOPL_ON_EXEC_PAGE: u32 = 10;
pub const kGUARD_EXC_SEC_EXEC_ON_IOPL_PAGE: u32 = 11;
pub const kGUARD_EXC_SEC_UPL_WRITE_ON_EXEC_REGION: u32 = 12;

pub const kGUARD_EXC_SEC_ACCESS_FAULT: u32 = 98;
pub const kGUARD_EXC_SEC_ASYNC_ACCESS_FAULT: u32 = 99;
pub const kGUARD_EXC_SEC_COPY_DENIED: u32 = 100;
pub const kGUARD_EXC_SEC_SHARING_DENIED: u32 = 101;
pub const kGUARD_EXC_MTE_SYNC_FAULT: u32 = 200;
pub const kGUARD_EXC_MTE_ASYNC_USER_FAULT: u32 = 201;
pub const kGUARD_EXC_MTE_ASYNC_KERN_FAULT: u32 = 202;

pub const kGUARD_EXC_MTE_SOFT_MODE: u32 = 0x100000;

pub const __VM_LEDGER_ACCOUNTING_POSTMARK: u64 = 2_019_032_600;
pub const VM_LEDGER_TAG_NONE: libc::c_int = 0x0000_0000;
pub const VM_LEDGER_TAG_DEFAULT: libc::c_int = 0x0000_0001;
pub const VM_LEDGER_TAG_NETWORK: libc::c_int = 0x0000_0002;
pub const VM_LEDGER_TAG_MEDIA: libc::c_int = 0x0000_0003;
pub const VM_LEDGER_TAG_GRAPHICS: libc::c_int = 0x0000_0004;
pub const VM_LEDGER_TAG_NEURAL: libc::c_int = 0x0000_0005;
pub const VM_LEDGER_TAG_MAX: libc::c_int = VM_LEDGER_TAG_NEURAL;
pub const VM_LEDGER_TAG_UNCHANGED: libc::c_int = -1;
pub const VM_LEDGER_FLAG_NO_FOOTPRINT: libc::c_int = 1 << 0;
pub const VM_LEDGER_FLAG_NO_FOOTPRINT_FOR_DEBUG: libc::c_int = 1 << 1;
pub const VM_LEDGER_FLAG_FROM_KERNEL: libc::c_int = 1 << 2;
pub const VM_LEDGER_FLAGS_USER: libc::c_int =
    VM_LEDGER_FLAG_NO_FOOTPRINT | VM_LEDGER_FLAG_NO_FOOTPRINT_FOR_DEBUG;
pub const VM_LEDGER_FLAGS_ALL: libc::c_int = VM_LEDGER_FLAGS_USER | VM_LEDGER_FLAG_FROM_KERNEL;

#[inline]
pub fn vm_statistics_truncate_to_32_bit(value: u64) -> u32 {
    if value > u32::MAX as u64 {
        u32::MAX
    } else {
        value as u32
    }
}

#[inline]
pub fn vm_get_flags_alias(flags: libc::c_int) -> u8 {
    ((flags >> 24) & 0xff) as u8
}

#[inline]
pub fn vm_set_flags_alias(flags: &mut libc::c_int, alias: u8) {
    *flags = (*flags & !VM_FLAGS_ALIAS_MASK) | ((alias as libc::c_int) << 24);
}

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
    pub swapped_count: u64,
}

#[repr(C, packed(8))]
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
