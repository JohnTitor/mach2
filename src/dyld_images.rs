//! This module roughly corresponds to `mach-o/dyld_images.h`.

#![allow(non_snake_case)]

use super::{mach_types::uuid_t, port::mach_port_t};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct mach_header {
    pub magic: ::libc::c_uint,
    pub cputype: ::libc::c_int,
    pub cpusubtype: ::libc::c_int,
    pub filetype: ::libc::c_uint,
    pub ncmds: ::libc::c_uint,
    pub sizeofcmds: ::libc::c_uint,
    pub flags: ::libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct mach_header_64 {
    pub magic: ::libc::c_uint,
    pub cputype: ::libc::c_int,
    pub cpusubtype: ::libc::c_int,
    pub filetype: ::libc::c_uint,
    pub ncmds: ::libc::c_uint,
    pub sizeofcmds: ::libc::c_uint,
    pub flags: ::libc::c_uint,
    pub reserved: ::libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct segment_command_64 {
    pub cmd: ::libc::c_uint,
    pub cmdsize: ::libc::c_uint,
    pub segname: [::libc::c_char; 16],
    pub vmaddr: ::libc::c_ulong,
    pub vmsize: ::libc::c_ulong,
    pub fileoff: ::libc::c_ulong,
    pub filesize: ::libc::c_ulong,
    pub maxprot: ::libc::c_int,
    pub initprot: ::libc::c_int,
    pub nsects: ::libc::c_uint,
    pub flags: ::libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct dyld_uuid_info {
    pub imageLoadAddress: *const mach_header,
    pub imageUUID: uuid_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct dyld_aot_image_info {
    pub x86LoadAddress: *const mach_header,
    pub aotLoadAddress: *const mach_header,
    pub aotImageSize: ::libc::c_ulong,
    pub aotImageKey: [::libc::c_uchar; 32],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct dyld_image_info {
    pub imageLoadAddress: *mut mach_header,
    pub imageFilePath: *const ::libc::c_char,
    pub imageFileModDate: ::libc::uintptr_t,
}

pub type dyld_image_mode = ::libc::c_uint;

pub type dyld_image_notifier = ::core::option::Option<
    unsafe extern "C" fn(mode: dyld_image_mode, infoCount: ::libc::c_uint, info: *const dyld_image_info),
>;

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct dyld_all_image_infos {
    pub version: ::libc::c_uint,
    pub infoArrayCount: ::libc::c_uint,
    pub infoArray: *const dyld_image_info,
    pub notification: dyld_image_notifier,
    pub processDetachedFromSharedRegion: bool,
    pub libSystemInitialized: bool,
    pub dyldImageLoadAddress: *const mach_header,
    pub jitInfo: *mut ::libc::c_void,
    pub dyldVersion: *const ::libc::c_char,
    pub errorMessage: *const ::libc::c_char,
    pub terminationFlags: ::libc::uintptr_t,
    pub coreSymbolicationShmPage: *mut ::libc::c_void,
    pub systemOrderFlag: ::libc::uintptr_t,
    pub uuidArrayCount: ::libc::uintptr_t,
    pub uuidArray: *const dyld_uuid_info,
    pub dyldAllImageInfosAddress: *mut dyld_all_image_infos,
    pub initialImageCount: ::libc::uintptr_t,
    pub errorKind: ::libc::uintptr_t,
    pub errorClientOfDylibPath: *const ::libc::c_char,
    pub errorTargetDylibPath: *const ::libc::c_char,
    pub errorSymbol: *const ::libc::c_char,
    pub sharedCacheSlide: ::libc::uintptr_t,
    pub sharedCacheUUID: [::libc::c_uchar; 16],
    pub sharedCacheBaseAddress: ::libc::uintptr_t,
    pub infoArrayChangeTimestamp: ::libc::c_ulong,
    pub dyldPath: *const ::libc::c_char,
    pub notifyPorts: [mach_port_t; 8],
    pub reserved: [::libc::uintptr_t; 7],
    pub sharedCacheFSID: ::libc::c_ulong,
    pub sharedCacheFSObjID: ::libc::c_ulong,
    pub compact_dyld_image_info_addr: ::libc::uintptr_t,
    pub compact_dyld_image_info_size: ::libc::uintptr_t,
    pub platform: ::libc::c_uint,
    pub aotInfoCount: ::libc::c_uint,
    pub aotInfoArray: *const dyld_aot_image_info,
    pub aotInfoArrayChangeTimestamp: ::libc::c_ulong,
    pub aotSharedCacheBaseAddress: ::libc::uintptr_t,
    pub aotSharedCacheUUID: [::libc::c_uchar; 16],
}
