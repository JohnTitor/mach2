//! This module roughly corresponds to `mach-o/loader.h`.

pub const LC_SEGMENT: u32 = 1;
pub const LC_SEGMENT_64: u32 = 25;
pub const SEG_TEXT: &[u8; 7usize] = b"__TEXT\0";
pub const SEG_LINKEDIT: &[u8; 11usize] = b"__LINKEDIT\0";
pub const LC_SYMTAB: u32 = 2;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct load_command {
    pub cmd: u32,
    pub cmdsize: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct symtab_command {
    pub cmd: u32,
    pub cmdsize: u32,
    pub symoff: u32,
    pub nsyms: u32,
    pub stroff: u32,
    pub strsize: u32,
}
