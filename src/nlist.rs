//! This module roughly corresponds to `mach-o/nlist.h`.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlist_64 {
    pub n_un: nlist_64__bindgen_ty_1,
    pub n_type: u8,
    pub n_sect: u8,
    pub n_desc: u16,
    pub n_value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nlist_64__bindgen_ty_1 {
    pub n_strx: u32,
}
