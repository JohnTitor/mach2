//! This module corresponds to `mach/i386/_structs.h` and `mach/arm/_structs.h`.

use core::cmp::Ordering;
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::hash::{Hash, Hasher};
use mem;
use message::mach_msg_type_number_t;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct fp_control_t(u16);

impl fp_control_t {
    pub fn new() -> fp_control_t {
        fp_control_t(0)
    }

    pub fn invalid(self) -> bool {
        self.0 & 0x1 > 0
    }

    pub fn set_invalid(&mut self, val: bool) {
        self.0 = (self.0 & !0x1) | (val as u16);
    }

    pub fn denorm(self) -> bool {
        self.0 & 0x2 > 0
    }

    pub fn set_denorm(&mut self, val: bool) {
        self.0 = (self.0 & !0x2) | ((val as u16) << 1);
    }

    pub fn zdiv(self) -> bool {
        self.0 & 0x4 > 0
    }

    pub fn set_zdiv(&mut self, val: bool) {
        self.0 = (self.0 & !0x4) | ((val as u16) << 2);
    }

    pub fn ovrfl(self) -> bool {
        self.0 & 0x8 > 0
    }

    pub fn set_ovrfl(&mut self, val: bool) {
        self.0 = (self.0 & !0x8) | ((val as u16) << 3);
    }

    pub fn undfl(self) -> bool {
        self.0 & 0x10 > 0
    }

    pub fn set_undfl(&mut self, val: bool) {
        self.0 = (self.0 & !0x10) | ((val as u16) << 4);
    }

    pub fn precis(self) -> bool {
        self.0 & 0x20 > 0
    }

    pub fn set_precis(&mut self, val: bool) {
        self.0 = (self.0 & !0x20) | ((val as u16) << 5);
    }

    // Skip 0x40, 0x80

    pub fn pc(self) -> u8 {
        (self.0 & 0x300 >> 8) as u8
    }

    pub fn set_pc(&mut self, val: u8) {
        if val > 3 {
            panic!("Invalid value for pc, must be <= 3")
        }
        self.0 = (self.0 & !0x300) | ((val as u16) << 8);
    }

    pub fn rc(self) -> u8 {
        (self.0 & 0xC00 >> 10) as u8
    }

    pub fn set_rc(&mut self, val: u8) {
        if val > 3 {
            panic!("Invalid value for rc, must be <= 3")
        }
        self.0 = (self.0 & !0xC00) | ((val as u16) << 10);
    }

    pub fn inf(self) -> bool {
        self.0 & 0x1000 > 0
    }

    pub fn set_info(&mut self, val: bool) {
        self.0 = (self.0 & !0x1000) | ((val as u16) << 12);
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct fp_status_t(u16);

impl fp_status_t {
    pub fn new() -> fp_status_t {
        fp_status_t(0)
    }

    pub fn invalid(self) -> bool {
        self.0 & 0x1 > 0
    }

    pub fn set_invalid(&mut self, val: bool) {
        self.0 = (self.0 & !0x1) | (val as u16);
    }

    pub fn denorm(self) -> bool {
        self.0 & 0x2 > 0
    }

    pub fn set_denorm(&mut self, val: bool) {
        self.0 = (self.0 & !0x2) | ((val as u16) << 1);
    }

    pub fn zdiv(self) -> bool {
        self.0 & 0x4 > 0
    }

    pub fn set_zdiv(&mut self, val: bool) {
        self.0 = (self.0 & !0x4) | ((val as u16) << 2);
    }

    pub fn ovrfl(self) -> bool {
        self.0 & 0x8 > 0
    }

    pub fn set_ovrfl(&mut self, val: bool) {
        self.0 = (self.0 & !0x8) | ((val as u16) << 3);
    }

    pub fn undfl(self) -> bool {
        self.0 & 0x10 > 0
    }

    pub fn set_undfl(&mut self, val: bool) {
        self.0 = (self.0 & !0x10) | ((val as u16) << 4);
    }

    pub fn precis(self) -> bool {
        self.0 & 0x20 > 0
    }

    pub fn set_precis(&mut self, val: bool) {
        self.0 = (self.0 & !0x20) | ((val as u16) << 5);
    }

    pub fn stkflt(self) -> bool {
        self.0 & 0x40 > 0
    }

    pub fn set_stkflt(&mut self, val: bool) {
        self.0 = (self.0 & !0x40) | ((val as u16) << 6);
    }

    pub fn errsumm(self) -> bool {
        self.0 & 0x80 > 0
    }

    pub fn set_errsumm(&mut self, val: bool) {
        self.0 = (self.0 & !0x80) | ((val as u16) << 7);
    }

    pub fn c0(self) -> bool {
        self.0 & 0x100 > 0
    }

    pub fn set_c0(&mut self, val: bool) {
        self.0 = (self.0 & !0x100) | ((val as u16) << 8);
    }

    pub fn c1(self) -> bool {
        self.0 & 0x200 > 0
    }

    pub fn set_c1(&mut self, val: bool) {
        self.0 = (self.0 & !0x200) | ((val as u16) << 9);
    }

    pub fn c2(self) -> bool {
        self.0 & 0x400 > 0
    }

    pub fn set_c2(&mut self, val: bool) {
        self.0 = (self.0 & !0x400) | ((val as u16) << 10);
    }

    pub fn tos(self) -> u8 {
        (self.0 & 0x3800 >> 11) as u8
    }

    pub fn set_tos(&mut self, val: u8) {
        if val > 7 {
            panic!("Invalid value for tos, must be <= 7")
        }
        self.0 = (self.0 & !0x3800) | ((val as u16) << 11);
    }

    pub fn c3(self) -> bool {
        self.0 & 0x4000 > 0
    }

    pub fn set_c3(&mut self, val: bool) {
        self.0 = (self.0 & !0x4000) | ((val as u16) << 14);
    }

    pub fn busy(self) -> bool {
        self.0 & 0x8000 > 0
    }

    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !0x8000) | ((val as u16) << 15);
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct mmst_reg_t {
    pub mmst_reg: [i8; 10],
    pub mmst_rsrv: [i8; 6],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct xmm_reg_t {
    pub xmm_reg: [i8; 16],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct i386_thread_state32_t {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub esp: u32,
    pub ss: u32,
    pub eflags: u32,
    pub eip: u32,
    pub cs: u32,
    pub ds: u32,
    pub es: u32,
    pub fs: u32,
    pub gs: u32,
}

impl i386_thread_state32_t {
    pub fn count() -> mach_msg_type_number_t {
        (mem::size_of::<Self>() / mem::size_of::<::libc::c_int>()) as mach_msg_type_number_t
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i386_float_state_t {
    pub fpu_reserved: [i32; 2],
    pub fpu_fcw: fp_control_t,
    pub fpu_fsw: fp_status_t,
    pub fpu_ftw: u8,
    pub fpu_rsrv1: u8,
    pub fpu_fop: u16,
    pub fpu_ip: u32,
    pub fpu_cs: u16,
    pub fpu_rsrv2: u16,
    pub fpu_dp: u32,
    pub fpu_ds: u16,
    pub fpu_rsrv3: u16,
    pub fpu_mxcsr: u32,
    pub fpu_mxcsrmask: u32,
    pub fpu_stmm0: mmst_reg_t,
    pub fpu_stmm1: mmst_reg_t,
    pub fpu_stmm2: mmst_reg_t,
    pub fpu_stmm3: mmst_reg_t,
    pub fpu_stmm4: mmst_reg_t,
    pub fpu_stmm5: mmst_reg_t,
    pub fpu_stmm6: mmst_reg_t,
    pub fpu_stmm7: mmst_reg_t,
    pub fpu_xmm0: xmm_reg_t,
    pub fpu_xmm1: xmm_reg_t,
    pub fpu_xmm2: xmm_reg_t,
    pub fpu_xmm3: xmm_reg_t,
    pub fpu_xmm4: xmm_reg_t,
    pub fpu_xmm5: xmm_reg_t,
    pub fpu_xmm6: xmm_reg_t,
    pub fpu_xmm7: xmm_reg_t,
    pub fpu_rsrv4: [i8; 14 * 16],
    pub fpu_reserved1: i32,
}

impl i386_float_state_t {
    pub fn new() -> Self {
        Self {
            fpu_reserved: [0; 2],
            fpu_fcw: Default::default(),
            fpu_fsw: Default::default(),
            fpu_ftw: 0,
            fpu_rsrv1: 0,
            fpu_fop: 0,
            fpu_ip: 0,
            fpu_cs: 0,
            fpu_rsrv2: 0,
            fpu_dp: 0,
            fpu_ds: 0,
            fpu_rsrv3: 0,
            fpu_mxcsr: 0,
            fpu_mxcsrmask: 0,
            fpu_stmm0: Default::default(),
            fpu_stmm1: Default::default(),
            fpu_stmm2: Default::default(),
            fpu_stmm3: Default::default(),
            fpu_stmm4: Default::default(),
            fpu_stmm5: Default::default(),
            fpu_stmm6: Default::default(),
            fpu_stmm7: Default::default(),
            fpu_xmm0: Default::default(),
            fpu_xmm1: Default::default(),
            fpu_xmm2: Default::default(),
            fpu_xmm3: Default::default(),
            fpu_xmm4: Default::default(),
            fpu_xmm5: Default::default(),
            fpu_xmm6: Default::default(),
            fpu_xmm7: Default::default(),
            fpu_rsrv4: [0; 14 * 16],
            fpu_reserved1: 0,
        }
    }

    pub fn count() -> mach_msg_type_number_t {
        (mem::size_of::<Self>() / mem::size_of::<::libc::c_int>()) as mach_msg_type_number_t
    }
}

impl Default for i386_float_state_t {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for i386_float_state_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("i386_float_state_t");
        ds.field("fpu_reserved", &self.fpu_reserved);
        ds.field("fpu_fcw", &self.fpu_fcw);
        ds.field("fpu_fsw", &self.fpu_fsw);
        ds.field("fpu_ftw", &self.fpu_ftw);
        ds.field("fpu_rsrv1", &self.fpu_rsrv1);
        ds.field("fpu_fop", &self.fpu_fop);
        ds.field("fpu_ip", &self.fpu_ip);
        ds.field("fpu_cs", &self.fpu_cs);
        ds.field("fpu_rsrv2", &self.fpu_rsrv2);
        ds.field("fpu_dp", &self.fpu_dp);
        ds.field("fpu_ds", &self.fpu_ds);
        ds.field("fpu_rsrv3", &self.fpu_rsrv3);
        ds.field("fpu_mxcsr", &self.fpu_mxcsr);
        ds.field("fpu_mxcsrmask", &self.fpu_mxcsrmask);
        ds.field("fpu_stmm0", &self.fpu_stmm0);
        ds.field("fpu_stmm1", &self.fpu_stmm1);
        ds.field("fpu_stmm2", &self.fpu_stmm2);
        ds.field("fpu_stmm3", &self.fpu_stmm3);
        ds.field("fpu_stmm4", &self.fpu_stmm4);
        ds.field("fpu_stmm5", &self.fpu_stmm5);
        ds.field("fpu_stmm6", &self.fpu_stmm6);
        ds.field("fpu_stmm7", &self.fpu_stmm7);
        ds.field("fpu_xmm0", &self.fpu_xmm0);
        ds.field("fpu_xmm1", &self.fpu_xmm1);
        ds.field("fpu_xmm2", &self.fpu_xmm2);
        ds.field("fpu_xmm3", &self.fpu_xmm3);
        ds.field("fpu_xmm4", &self.fpu_xmm4);
        ds.field("fpu_xmm5", &self.fpu_xmm5);
        ds.field("fpu_xmm6", &self.fpu_xmm6);
        ds.field("fpu_xmm7", &self.fpu_xmm7);
        ds.field("fpu_rsrv4", &(&self.fpu_rsrv4 as &[i8]));
        ds.field("fpu_reserved1", &self.fpu_reserved1);
        ds.finish()
    }
}

impl PartialEq for i386_float_state_t {
    fn eq(&self, other: &Self) -> bool {
        self.fpu_reserved == other.fpu_reserved
            && self.fpu_fcw == other.fpu_fcw
            && self.fpu_fsw == other.fpu_fsw
            && self.fpu_ftw == other.fpu_ftw
            && self.fpu_rsrv1 == other.fpu_rsrv1
            && self.fpu_fop == other.fpu_fop
            && self.fpu_ip == other.fpu_ip
            && self.fpu_cs == other.fpu_cs
            && self.fpu_rsrv2 == other.fpu_rsrv2
            && self.fpu_dp == other.fpu_dp
            && self.fpu_ds == other.fpu_ds
            && self.fpu_rsrv3 == other.fpu_rsrv3
            && self.fpu_mxcsr == other.fpu_mxcsr
            && self.fpu_mxcsrmask == other.fpu_mxcsrmask
            && self.fpu_stmm0 == other.fpu_stmm0
            && self.fpu_stmm1 == other.fpu_stmm1
            && self.fpu_stmm2 == other.fpu_stmm2
            && self.fpu_stmm3 == other.fpu_stmm3
            && self.fpu_stmm4 == other.fpu_stmm4
            && self.fpu_stmm5 == other.fpu_stmm5
            && self.fpu_stmm6 == other.fpu_stmm6
            && self.fpu_stmm7 == other.fpu_stmm7
            && self.fpu_xmm0 == other.fpu_xmm0
            && self.fpu_xmm1 == other.fpu_xmm1
            && self.fpu_xmm2 == other.fpu_xmm2
            && self.fpu_xmm3 == other.fpu_xmm3
            && self.fpu_xmm4 == other.fpu_xmm4
            && self.fpu_xmm5 == other.fpu_xmm5
            && self.fpu_xmm6 == other.fpu_xmm6
            && self.fpu_xmm7 == other.fpu_xmm7
            && <[i8] as PartialEq<[i8]>>::eq(&self.fpu_rsrv4, &other.fpu_rsrv4)
            && self.fpu_reserved1 == other.fpu_reserved1
    }
}

impl Eq for i386_float_state_t {}

impl Hash for i386_float_state_t {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fpu_reserved.hash(state);
        self.fpu_fcw.hash(state);
        self.fpu_fsw.hash(state);
        self.fpu_ftw.hash(state);
        self.fpu_rsrv1.hash(state);
        self.fpu_fop.hash(state);
        self.fpu_ip.hash(state);
        self.fpu_cs.hash(state);
        self.fpu_rsrv2.hash(state);
        self.fpu_dp.hash(state);
        self.fpu_ds.hash(state);
        self.fpu_rsrv3.hash(state);
        self.fpu_mxcsr.hash(state);
        self.fpu_mxcsrmask.hash(state);
        self.fpu_stmm0.hash(state);
        self.fpu_stmm1.hash(state);
        self.fpu_stmm2.hash(state);
        self.fpu_stmm3.hash(state);
        self.fpu_stmm4.hash(state);
        self.fpu_stmm5.hash(state);
        self.fpu_stmm6.hash(state);
        self.fpu_stmm7.hash(state);
        self.fpu_xmm0.hash(state);
        self.fpu_xmm1.hash(state);
        self.fpu_xmm2.hash(state);
        self.fpu_xmm3.hash(state);
        self.fpu_xmm4.hash(state);
        self.fpu_xmm5.hash(state);
        self.fpu_xmm6.hash(state);
        self.fpu_xmm7.hash(state);
        self.fpu_rsrv4.len().hash(state);
        (&self.fpu_rsrv4 as &[i8]).hash(state);
        self.fpu_reserved1.hash(state);
    }
}

impl PartialOrd for i386_float_state_t {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for i386_float_state_t {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.fpu_reserved.cmp(&other.fpu_reserved) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_fcw.cmp(&other.fpu_fcw) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_fsw.cmp(&other.fpu_fsw) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ftw.cmp(&other.fpu_ftw) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv1.cmp(&other.fpu_rsrv1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_fop.cmp(&other.fpu_fop) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ip.cmp(&other.fpu_ip) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_cs.cmp(&other.fpu_cs) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv2.cmp(&other.fpu_rsrv2) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_dp.cmp(&other.fpu_dp) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ds.cmp(&other.fpu_ds) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv3.cmp(&other.fpu_rsrv3) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_mxcsr.cmp(&other.fpu_mxcsr) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_mxcsrmask.cmp(&other.fpu_mxcsrmask) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm0.cmp(&other.fpu_stmm0) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm1.cmp(&other.fpu_stmm1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm2.cmp(&other.fpu_stmm2) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm3.cmp(&other.fpu_stmm3) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm4.cmp(&other.fpu_stmm4) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm5.cmp(&other.fpu_stmm5) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm6.cmp(&other.fpu_stmm6) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm7.cmp(&other.fpu_stmm7) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm0.cmp(&other.fpu_xmm0) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm1.cmp(&other.fpu_xmm1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm2.cmp(&other.fpu_xmm2) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm3.cmp(&other.fpu_xmm3) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm4.cmp(&other.fpu_xmm4) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm5.cmp(&other.fpu_xmm5) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm6.cmp(&other.fpu_xmm6) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm7.cmp(&other.fpu_xmm7) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv4.cmp(&other.fpu_rsrv4) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_reserved1.cmp(&other.fpu_reserved1) {
            Ordering::Equal => (),
            ord => return ord,
        }

        Ordering::Equal
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct i386_exception_state_t {
    pub trapno: u16,
    pub cpu: u16,
    pub err: u32,
    pub faultvaddr: u32,
}

impl i386_exception_state_t {
    pub fn count() -> mach_msg_type_number_t {
        (mem::size_of::<Self>() / mem::size_of::<::libc::c_int>()) as mach_msg_type_number_t
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i386_avx_state_t {
    pub fpu_reserved: [i32; 2],
    pub fpu_fcw: fp_control_t,
    pub fpu_fsw: fp_status_t,
    pub fpu_ftw: u8,
    pub fpu_rsrv1: u8,
    pub fpu_fop: u16,
    pub fpu_ip: u32,
    pub fpu_cs: u16,
    pub fpu_rsrv2: u16,
    pub fpu_dp: u32,
    pub fpu_ds: u16,
    pub fpu_rsrv3: u16,
    pub fpu_mxcsr: u32,
    pub fpu_mxcsrmask: u32,
    pub fpu_stmm0: mmst_reg_t,
    pub fpu_stmm1: mmst_reg_t,
    pub fpu_stmm2: mmst_reg_t,
    pub fpu_stmm3: mmst_reg_t,
    pub fpu_stmm4: mmst_reg_t,
    pub fpu_stmm5: mmst_reg_t,
    pub fpu_stmm6: mmst_reg_t,
    pub fpu_stmm7: mmst_reg_t,
    pub fpu_xmm0: xmm_reg_t,
    pub fpu_xmm1: xmm_reg_t,
    pub fpu_xmm2: xmm_reg_t,
    pub fpu_xmm3: xmm_reg_t,
    pub fpu_xmm4: xmm_reg_t,
    pub fpu_xmm5: xmm_reg_t,
    pub fpu_xmm6: xmm_reg_t,
    pub fpu_xmm7: xmm_reg_t,
    pub fpu_rsrv4: [i8; 14 * 16],
    pub fpu_reserved1: i32,
    pub avx_reserved1: [i8; 64],
    pub fpu_ymmh0: xmm_reg_t,
    pub fpu_ymmh1: xmm_reg_t,
    pub fpu_ymmh2: xmm_reg_t,
    pub fpu_ymmh3: xmm_reg_t,
    pub fpu_ymmh4: xmm_reg_t,
    pub fpu_ymmh5: xmm_reg_t,
    pub fpu_ymmh6: xmm_reg_t,
    pub fpu_ymmh7: xmm_reg_t,
}

impl i386_avx_state_t {
    pub fn new() -> Self {
        Self {
            fpu_reserved: [0; 2],
            fpu_fcw: Default::default(),
            fpu_fsw: Default::default(),
            fpu_ftw: 0,
            fpu_rsrv1: 0,
            fpu_fop: 0,
            fpu_ip: 0,
            fpu_cs: 0,
            fpu_rsrv2: 0,
            fpu_dp: 0,
            fpu_ds: 0,
            fpu_rsrv3: 0,
            fpu_mxcsr: 0,
            fpu_mxcsrmask: 0,
            fpu_stmm0: Default::default(),
            fpu_stmm1: Default::default(),
            fpu_stmm2: Default::default(),
            fpu_stmm3: Default::default(),
            fpu_stmm4: Default::default(),
            fpu_stmm5: Default::default(),
            fpu_stmm6: Default::default(),
            fpu_stmm7: Default::default(),
            fpu_xmm0: Default::default(),
            fpu_xmm1: Default::default(),
            fpu_xmm2: Default::default(),
            fpu_xmm3: Default::default(),
            fpu_xmm4: Default::default(),
            fpu_xmm5: Default::default(),
            fpu_xmm6: Default::default(),
            fpu_xmm7: Default::default(),
            fpu_rsrv4: [0; 14 * 16],
            fpu_reserved1: 0,
            avx_reserved1: [0; 64],
            fpu_ymmh0: Default::default(),
            fpu_ymmh1: Default::default(),
            fpu_ymmh2: Default::default(),
            fpu_ymmh3: Default::default(),
            fpu_ymmh4: Default::default(),
            fpu_ymmh5: Default::default(),
            fpu_ymmh6: Default::default(),
            fpu_ymmh7: Default::default(),
        }
    }

    pub fn count() -> mach_msg_type_number_t {
        (mem::size_of::<Self>() / mem::size_of::<::libc::c_int>()) as mach_msg_type_number_t
    }
}

impl Default for i386_avx_state_t {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for i386_avx_state_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("i386_avx_state_t");
        ds.field("fpu_reserved", &self.fpu_reserved);
        ds.field("fpu_fcw", &self.fpu_fcw);
        ds.field("fpu_fsw", &self.fpu_fsw);
        ds.field("fpu_ftw", &self.fpu_ftw);
        ds.field("fpu_rsrv1", &self.fpu_rsrv1);
        ds.field("fpu_fop", &self.fpu_fop);
        ds.field("fpu_ip", &self.fpu_ip);
        ds.field("fpu_cs", &self.fpu_cs);
        ds.field("fpu_rsrv2", &self.fpu_rsrv2);
        ds.field("fpu_dp", &self.fpu_dp);
        ds.field("fpu_ds", &self.fpu_ds);
        ds.field("fpu_rsrv3", &self.fpu_rsrv3);
        ds.field("fpu_mxcsr", &self.fpu_mxcsr);
        ds.field("fpu_mxcsrmask", &self.fpu_mxcsrmask);
        ds.field("fpu_stmm0", &self.fpu_stmm0);
        ds.field("fpu_stmm1", &self.fpu_stmm1);
        ds.field("fpu_stmm2", &self.fpu_stmm2);
        ds.field("fpu_stmm3", &self.fpu_stmm3);
        ds.field("fpu_stmm4", &self.fpu_stmm4);
        ds.field("fpu_stmm5", &self.fpu_stmm5);
        ds.field("fpu_stmm6", &self.fpu_stmm6);
        ds.field("fpu_stmm7", &self.fpu_stmm7);
        ds.field("fpu_xmm0", &self.fpu_xmm0);
        ds.field("fpu_xmm1", &self.fpu_xmm1);
        ds.field("fpu_xmm2", &self.fpu_xmm2);
        ds.field("fpu_xmm3", &self.fpu_xmm3);
        ds.field("fpu_xmm4", &self.fpu_xmm4);
        ds.field("fpu_xmm5", &self.fpu_xmm5);
        ds.field("fpu_xmm6", &self.fpu_xmm6);
        ds.field("fpu_xmm7", &self.fpu_xmm7);
        ds.field("fpu_rsrv4", &(&self.fpu_rsrv4 as &[_]));
        ds.field("fpu_reserved1", &self.fpu_reserved1);
        ds.field("avx_reserved1", &(&self.avx_reserved1 as &[_]));
        ds.field("fpu_ymmh0", &self.fpu_ymmh0);
        ds.field("fpu_ymmh1", &self.fpu_ymmh1);
        ds.field("fpu_ymmh2", &self.fpu_ymmh2);
        ds.field("fpu_ymmh3", &self.fpu_ymmh3);
        ds.field("fpu_ymmh4", &self.fpu_ymmh4);
        ds.field("fpu_ymmh5", &self.fpu_ymmh5);
        ds.field("fpu_ymmh6", &self.fpu_ymmh6);
        ds.field("fpu_ymmh7", &self.fpu_ymmh7);
        ds.finish()
    }
}

impl PartialEq for i386_avx_state_t {
    fn eq(&self, other: &Self) -> bool {
        self.fpu_reserved == other.fpu_reserved
            && self.fpu_fcw == other.fpu_fcw
            && self.fpu_fsw == other.fpu_fsw
            && self.fpu_ftw == other.fpu_ftw
            && self.fpu_rsrv1 == other.fpu_rsrv1
            && self.fpu_fop == other.fpu_fop
            && self.fpu_ip == other.fpu_ip
            && self.fpu_cs == other.fpu_cs
            && self.fpu_rsrv2 == other.fpu_rsrv2
            && self.fpu_dp == other.fpu_dp
            && self.fpu_ds == other.fpu_ds
            && self.fpu_rsrv3 == other.fpu_rsrv3
            && self.fpu_mxcsr == other.fpu_mxcsr
            && self.fpu_mxcsrmask == other.fpu_mxcsrmask
            && self.fpu_stmm0 == other.fpu_stmm0
            && self.fpu_stmm1 == other.fpu_stmm1
            && self.fpu_stmm2 == other.fpu_stmm2
            && self.fpu_stmm3 == other.fpu_stmm3
            && self.fpu_stmm4 == other.fpu_stmm4
            && self.fpu_stmm5 == other.fpu_stmm5
            && self.fpu_stmm6 == other.fpu_stmm6
            && self.fpu_stmm7 == other.fpu_stmm7
            && self.fpu_xmm0 == other.fpu_xmm0
            && self.fpu_xmm1 == other.fpu_xmm1
            && self.fpu_xmm2 == other.fpu_xmm2
            && self.fpu_xmm3 == other.fpu_xmm3
            && self.fpu_xmm4 == other.fpu_xmm4
            && self.fpu_xmm5 == other.fpu_xmm5
            && self.fpu_xmm6 == other.fpu_xmm6
            && self.fpu_xmm7 == other.fpu_xmm7
            && <[i8] as PartialEq<[i8]>>::eq(&self.fpu_rsrv4, &other.fpu_rsrv4)
            && self.fpu_reserved1 == other.fpu_reserved1
            && <[i8] as PartialEq<[i8]>>::eq(&self.avx_reserved1, &other.avx_reserved1)
            && self.fpu_ymmh0 == other.fpu_ymmh0
            && self.fpu_ymmh1 == other.fpu_ymmh1
            && self.fpu_ymmh2 == other.fpu_ymmh2
            && self.fpu_ymmh3 == other.fpu_ymmh3
            && self.fpu_ymmh4 == other.fpu_ymmh4
            && self.fpu_ymmh5 == other.fpu_ymmh5
            && self.fpu_ymmh6 == other.fpu_ymmh6
            && self.fpu_ymmh7 == other.fpu_ymmh7
    }
}

impl Eq for i386_avx_state_t {}

impl Hash for i386_avx_state_t {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fpu_reserved.hash(state);
        self.fpu_fcw.hash(state);
        self.fpu_fsw.hash(state);
        self.fpu_ftw.hash(state);
        self.fpu_rsrv1.hash(state);
        self.fpu_fop.hash(state);
        self.fpu_ip.hash(state);
        self.fpu_cs.hash(state);
        self.fpu_rsrv2.hash(state);
        self.fpu_dp.hash(state);
        self.fpu_ds.hash(state);
        self.fpu_rsrv3.hash(state);
        self.fpu_mxcsr.hash(state);
        self.fpu_mxcsrmask.hash(state);
        self.fpu_stmm0.hash(state);
        self.fpu_stmm1.hash(state);
        self.fpu_stmm2.hash(state);
        self.fpu_stmm3.hash(state);
        self.fpu_stmm4.hash(state);
        self.fpu_stmm5.hash(state);
        self.fpu_stmm6.hash(state);
        self.fpu_stmm7.hash(state);
        self.fpu_xmm0.hash(state);
        self.fpu_xmm1.hash(state);
        self.fpu_xmm2.hash(state);
        self.fpu_xmm3.hash(state);
        self.fpu_xmm4.hash(state);
        self.fpu_xmm5.hash(state);
        self.fpu_xmm6.hash(state);
        self.fpu_xmm7.hash(state);
        (&self.fpu_rsrv4 as &[i8]).hash(state);
        self.fpu_reserved1.hash(state);
        (&self.avx_reserved1 as &[i8]).hash(state);
        self.fpu_ymmh0.hash(state);
        self.fpu_ymmh1.hash(state);
        self.fpu_ymmh2.hash(state);
        self.fpu_ymmh3.hash(state);
        self.fpu_ymmh4.hash(state);
        self.fpu_ymmh5.hash(state);
        self.fpu_ymmh6.hash(state);
        self.fpu_ymmh7.hash(state);
    }
}

impl PartialOrd for i386_avx_state_t {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for i386_avx_state_t {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.fpu_reserved.cmp(&other.fpu_reserved) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_fcw.cmp(&other.fpu_fcw) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_fsw.cmp(&other.fpu_fsw) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ftw.cmp(&other.fpu_ftw) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv1.cmp(&other.fpu_rsrv1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_fop.cmp(&other.fpu_fop) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ip.cmp(&other.fpu_ip) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_cs.cmp(&other.fpu_cs) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv2.cmp(&other.fpu_rsrv2) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_dp.cmp(&other.fpu_dp) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ds.cmp(&other.fpu_ds) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv3.cmp(&other.fpu_rsrv3) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_mxcsr.cmp(&other.fpu_mxcsr) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_mxcsrmask.cmp(&other.fpu_mxcsrmask) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm0.cmp(&other.fpu_stmm0) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm1.cmp(&other.fpu_stmm1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm2.cmp(&other.fpu_stmm2) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm3.cmp(&other.fpu_stmm3) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm4.cmp(&other.fpu_stmm4) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm5.cmp(&other.fpu_stmm5) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm6.cmp(&other.fpu_stmm6) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm7.cmp(&other.fpu_stmm7) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm0.cmp(&other.fpu_xmm0) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm1.cmp(&other.fpu_xmm1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm2.cmp(&other.fpu_xmm2) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm3.cmp(&other.fpu_xmm3) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm4.cmp(&other.fpu_xmm4) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm5.cmp(&other.fpu_xmm5) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm6.cmp(&other.fpu_xmm6) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm7.cmp(&other.fpu_xmm7) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv4.cmp(&other.fpu_rsrv4) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_reserved1.cmp(&other.fpu_reserved1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.avx_reserved1.cmp(&other.avx_reserved1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh0.cmp(&other.fpu_ymmh0) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh1.cmp(&other.fpu_ymmh1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh2.cmp(&other.fpu_ymmh2) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh3.cmp(&other.fpu_ymmh3) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh4.cmp(&other.fpu_ymmh4) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh5.cmp(&other.fpu_ymmh5) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh6.cmp(&other.fpu_ymmh6) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh7.cmp(&other.fpu_ymmh7) {
            Ordering::Equal => (),
            ord => return ord,
        }

        Ordering::Equal
    }
}

#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct arm_thread_state64_t {
    pub __x: [u64; 29],
    pub __fp: u64,
    pub __lr: u64,
    pub __sp: u64,
    pub __pc: u64,
    pub __cpsr: u32,
    pub __flags: u32,
}

#[cfg(target_arch = "aarch64")]
impl arm_thread_state64_t {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn count() -> mach_msg_type_number_t {
        (mem::size_of::<Self>() / mem::size_of::<::libc::c_int>()) as mach_msg_type_number_t
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
#[cfg(feature = "unstable")]
pub struct x86_thread_state64_t {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
    pub cs: u64,
    pub fs: u64,
    pub gs: u64,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
#[cfg(not(feature = "unstable"))]
pub struct x86_thread_state64_t {
    pub __rax: u64,
    pub __rbx: u64,
    pub __rcx: u64,
    pub __rdx: u64,
    pub __rdi: u64,
    pub __rsi: u64,
    pub __rbp: u64,
    pub __rsp: u64,
    pub __r8: u64,
    pub __r9: u64,
    pub __r10: u64,
    pub __r11: u64,
    pub __r12: u64,
    pub __r13: u64,
    pub __r14: u64,
    pub __r15: u64,
    pub __rip: u64,
    pub __rflags: u64,
    pub __cs: u64,
    pub __fs: u64,
    pub __gs: u64,
}

impl x86_thread_state64_t {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn count() -> mach_msg_type_number_t {
        (mem::size_of::<Self>() / mem::size_of::<::libc::c_int>()) as mach_msg_type_number_t
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_float_state64_t {
    pub fpu_reserved: [i32; 2],
    pub fpu_fcw: fp_control_t,
    pub fpu_fsw: fp_status_t,
    pub fpu_ftw: u8,
    pub fpu_rsrv1: u8,
    pub fpu_fop: u16,
    pub fpu_ip: u32,
    pub fpu_cs: u16,
    pub fpu_rsrv2: u16,
    pub fpu_dp: u32,
    pub fpu_ds: u16,
    pub fpu_rsrv3: u16,
    pub fpu_mxcsr: u32,
    pub fpu_mxcsrmask: u32,
    pub fpu_stmm0: mmst_reg_t,
    pub fpu_stmm1: mmst_reg_t,
    pub fpu_stmm2: mmst_reg_t,
    pub fpu_stmm3: mmst_reg_t,
    pub fpu_stmm4: mmst_reg_t,
    pub fpu_stmm5: mmst_reg_t,
    pub fpu_stmm6: mmst_reg_t,
    pub fpu_stmm7: mmst_reg_t,
    pub fpu_xmm0: xmm_reg_t,
    pub fpu_xmm1: xmm_reg_t,
    pub fpu_xmm2: xmm_reg_t,
    pub fpu_xmm3: xmm_reg_t,
    pub fpu_xmm4: xmm_reg_t,
    pub fpu_xmm5: xmm_reg_t,
    pub fpu_xmm6: xmm_reg_t,
    pub fpu_xmm7: xmm_reg_t,
    pub fpu_xmm8: xmm_reg_t,
    pub fpu_xmm9: xmm_reg_t,
    pub fpu_xmm10: xmm_reg_t,
    pub fpu_xmm11: xmm_reg_t,
    pub fpu_xmm12: xmm_reg_t,
    pub fpu_xmm13: xmm_reg_t,
    pub fpu_xmm14: xmm_reg_t,
    pub fpu_xmm15: xmm_reg_t,
    pub fpu_rsrv4: [i8; 6 * 16],
    pub fpu_reserved1: i32,
}

impl x86_float_state64_t {
    pub fn new() -> Self {
        Self {
            fpu_reserved: [0; 2],
            fpu_fcw: Default::default(),
            fpu_fsw: Default::default(),
            fpu_ftw: 0,
            fpu_rsrv1: 0,
            fpu_fop: 0,
            fpu_ip: 0,
            fpu_cs: 0,
            fpu_rsrv2: 0,
            fpu_dp: 0,
            fpu_ds: 0,
            fpu_rsrv3: 0,
            fpu_mxcsr: 0,
            fpu_mxcsrmask: 0,
            fpu_stmm0: Default::default(),
            fpu_stmm1: Default::default(),
            fpu_stmm2: Default::default(),
            fpu_stmm3: Default::default(),
            fpu_stmm4: Default::default(),
            fpu_stmm5: Default::default(),
            fpu_stmm6: Default::default(),
            fpu_stmm7: Default::default(),
            fpu_xmm0: Default::default(),
            fpu_xmm1: Default::default(),
            fpu_xmm2: Default::default(),
            fpu_xmm3: Default::default(),
            fpu_xmm4: Default::default(),
            fpu_xmm5: Default::default(),
            fpu_xmm6: Default::default(),
            fpu_xmm7: Default::default(),
            fpu_xmm8: Default::default(),
            fpu_xmm9: Default::default(),
            fpu_xmm10: Default::default(),
            fpu_xmm11: Default::default(),
            fpu_xmm12: Default::default(),
            fpu_xmm13: Default::default(),
            fpu_xmm14: Default::default(),
            fpu_xmm15: Default::default(),
            fpu_rsrv4: [0; 6 * 16],
            fpu_reserved1: 0,
        }
    }

    pub fn count() -> mach_msg_type_number_t {
        (mem::size_of::<Self>() / mem::size_of::<::libc::c_int>()) as mach_msg_type_number_t
    }
}

impl Default for x86_float_state64_t {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for x86_float_state64_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("x86_float_state64_t");
        ds.field("fpu_reserved", &self.fpu_reserved);
        ds.field("fpu_fcw", &self.fpu_fcw);
        ds.field("fpu_fsw", &self.fpu_fsw);
        ds.field("fpu_ftw", &self.fpu_ftw);
        ds.field("fpu_rsrv1", &self.fpu_rsrv1);
        ds.field("fpu_fop", &self.fpu_fop);
        ds.field("fpu_ip", &self.fpu_ip);
        ds.field("fpu_cs", &self.fpu_cs);
        ds.field("fpu_rsrv2", &self.fpu_rsrv2);
        ds.field("fpu_dp", &self.fpu_dp);
        ds.field("fpu_ds", &self.fpu_ds);
        ds.field("fpu_rsrv3", &self.fpu_rsrv3);
        ds.field("fpu_mxcsr", &self.fpu_mxcsr);
        ds.field("fpu_mxcsrmask", &self.fpu_mxcsrmask);
        ds.field("fpu_stmm0", &self.fpu_stmm0);
        ds.field("fpu_stmm1", &self.fpu_stmm1);
        ds.field("fpu_stmm2", &self.fpu_stmm2);
        ds.field("fpu_stmm3", &self.fpu_stmm3);
        ds.field("fpu_stmm4", &self.fpu_stmm4);
        ds.field("fpu_stmm5", &self.fpu_stmm5);
        ds.field("fpu_stmm6", &self.fpu_stmm6);
        ds.field("fpu_stmm7", &self.fpu_stmm7);
        ds.field("fpu_xmm0", &self.fpu_xmm0);
        ds.field("fpu_xmm1", &self.fpu_xmm1);
        ds.field("fpu_xmm2", &self.fpu_xmm2);
        ds.field("fpu_xmm3", &self.fpu_xmm3);
        ds.field("fpu_xmm4", &self.fpu_xmm4);
        ds.field("fpu_xmm5", &self.fpu_xmm5);
        ds.field("fpu_xmm6", &self.fpu_xmm6);
        ds.field("fpu_xmm7", &self.fpu_xmm7);
        ds.field("fpu_xmm8", &self.fpu_xmm8);
        ds.field("fpu_xmm9", &self.fpu_xmm9);
        ds.field("fpu_xmm10", &self.fpu_xmm10);
        ds.field("fpu_xmm11", &self.fpu_xmm11);
        ds.field("fpu_xmm12", &self.fpu_xmm12);
        ds.field("fpu_xmm13", &self.fpu_xmm13);
        ds.field("fpu_xmm14", &self.fpu_xmm14);
        ds.field("fpu_xmm15", &self.fpu_xmm15);
        ds.field("fpu_rsrv4", &(&self.fpu_rsrv4 as &[i8]));
        ds.field("fpu_reserved1", &self.fpu_reserved1);
        ds.finish()
    }
}

impl PartialEq for x86_float_state64_t {
    fn eq(&self, other: &Self) -> bool {
        self.fpu_reserved == other.fpu_reserved
            && self.fpu_fcw == other.fpu_fcw
            && self.fpu_fsw == other.fpu_fsw
            && self.fpu_ftw == other.fpu_ftw
            && self.fpu_rsrv1 == other.fpu_rsrv1
            && self.fpu_fop == other.fpu_fop
            && self.fpu_ip == other.fpu_ip
            && self.fpu_cs == other.fpu_cs
            && self.fpu_rsrv2 == other.fpu_rsrv2
            && self.fpu_dp == other.fpu_dp
            && self.fpu_ds == other.fpu_ds
            && self.fpu_rsrv3 == other.fpu_rsrv3
            && self.fpu_mxcsr == other.fpu_mxcsr
            && self.fpu_mxcsrmask == other.fpu_mxcsrmask
            && self.fpu_stmm0 == other.fpu_stmm0
            && self.fpu_stmm1 == other.fpu_stmm1
            && self.fpu_stmm2 == other.fpu_stmm2
            && self.fpu_stmm3 == other.fpu_stmm3
            && self.fpu_stmm4 == other.fpu_stmm4
            && self.fpu_stmm5 == other.fpu_stmm5
            && self.fpu_stmm6 == other.fpu_stmm6
            && self.fpu_stmm7 == other.fpu_stmm7
            && self.fpu_xmm0 == other.fpu_xmm0
            && self.fpu_xmm1 == other.fpu_xmm1
            && self.fpu_xmm2 == other.fpu_xmm2
            && self.fpu_xmm3 == other.fpu_xmm3
            && self.fpu_xmm4 == other.fpu_xmm4
            && self.fpu_xmm5 == other.fpu_xmm5
            && self.fpu_xmm6 == other.fpu_xmm6
            && self.fpu_xmm7 == other.fpu_xmm7
            && self.fpu_xmm8 == other.fpu_xmm8
            && self.fpu_xmm9 == other.fpu_xmm9
            && self.fpu_xmm10 == other.fpu_xmm10
            && self.fpu_xmm11 == other.fpu_xmm11
            && self.fpu_xmm12 == other.fpu_xmm12
            && self.fpu_xmm13 == other.fpu_xmm13
            && self.fpu_xmm14 == other.fpu_xmm14
            && self.fpu_xmm15 == other.fpu_xmm15
            && <[i8] as PartialEq<[i8]>>::eq(&self.fpu_rsrv4, &other.fpu_rsrv4)
            && self.fpu_reserved1 == other.fpu_reserved1
    }
}

impl Eq for x86_float_state64_t {}

impl Hash for x86_float_state64_t {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fpu_reserved.hash(state);
        self.fpu_fcw.hash(state);
        self.fpu_fsw.hash(state);
        self.fpu_ftw.hash(state);
        self.fpu_rsrv1.hash(state);
        self.fpu_fop.hash(state);
        self.fpu_ip.hash(state);
        self.fpu_cs.hash(state);
        self.fpu_rsrv2.hash(state);
        self.fpu_dp.hash(state);
        self.fpu_ds.hash(state);
        self.fpu_rsrv3.hash(state);
        self.fpu_mxcsr.hash(state);
        self.fpu_mxcsrmask.hash(state);
        self.fpu_stmm0.hash(state);
        self.fpu_stmm1.hash(state);
        self.fpu_stmm2.hash(state);
        self.fpu_stmm3.hash(state);
        self.fpu_stmm4.hash(state);
        self.fpu_stmm5.hash(state);
        self.fpu_stmm6.hash(state);
        self.fpu_stmm7.hash(state);
        self.fpu_xmm0.hash(state);
        self.fpu_xmm1.hash(state);
        self.fpu_xmm2.hash(state);
        self.fpu_xmm3.hash(state);
        self.fpu_xmm4.hash(state);
        self.fpu_xmm5.hash(state);
        self.fpu_xmm6.hash(state);
        self.fpu_xmm7.hash(state);
        self.fpu_xmm8.hash(state);
        self.fpu_xmm9.hash(state);
        self.fpu_xmm10.hash(state);
        self.fpu_xmm11.hash(state);
        self.fpu_xmm12.hash(state);
        self.fpu_xmm13.hash(state);
        self.fpu_xmm14.hash(state);
        self.fpu_xmm15.hash(state);
        (&self.fpu_rsrv4 as &[i8]).hash(state);
        self.fpu_reserved1.hash(state);
    }
}

impl PartialOrd for x86_float_state64_t {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for x86_float_state64_t {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.fpu_reserved.cmp(&other.fpu_reserved) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_fcw.cmp(&other.fpu_fcw) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_fsw.cmp(&other.fpu_fsw) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ftw.cmp(&other.fpu_ftw) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv1.cmp(&other.fpu_rsrv1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_fop.cmp(&other.fpu_fop) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ip.cmp(&other.fpu_ip) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_cs.cmp(&other.fpu_cs) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv2.cmp(&other.fpu_rsrv2) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_dp.cmp(&other.fpu_dp) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ds.cmp(&other.fpu_ds) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv3.cmp(&other.fpu_rsrv3) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_mxcsr.cmp(&other.fpu_mxcsr) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_mxcsrmask.cmp(&other.fpu_mxcsrmask) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm0.cmp(&other.fpu_stmm0) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm1.cmp(&other.fpu_stmm1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm2.cmp(&other.fpu_stmm2) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm3.cmp(&other.fpu_stmm3) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm4.cmp(&other.fpu_stmm4) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm5.cmp(&other.fpu_stmm5) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm6.cmp(&other.fpu_stmm6) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm7.cmp(&other.fpu_stmm7) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm0.cmp(&other.fpu_xmm0) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm1.cmp(&other.fpu_xmm1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm2.cmp(&other.fpu_xmm2) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm3.cmp(&other.fpu_xmm3) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm4.cmp(&other.fpu_xmm4) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm5.cmp(&other.fpu_xmm5) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm6.cmp(&other.fpu_xmm6) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm7.cmp(&other.fpu_xmm7) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm8.cmp(&other.fpu_xmm8) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm9.cmp(&other.fpu_xmm9) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm10.cmp(&other.fpu_xmm10) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm11.cmp(&other.fpu_xmm11) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm12.cmp(&other.fpu_xmm12) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm13.cmp(&other.fpu_xmm13) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm14.cmp(&other.fpu_xmm14) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm15.cmp(&other.fpu_xmm15) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv4.cmp(&other.fpu_rsrv4) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_reserved1.cmp(&other.fpu_reserved1) {
            Ordering::Equal => (),
            ord => return ord,
        }

        Ordering::Equal
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct x86_exception_state64_t {
    pub trapno: u16,
    pub cpu: u16,
    pub err: u32,
    pub faultvaddr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_avx_state64_t {
    pub fpu_reserved: [i32; 2],
    pub fpu_fcw: fp_control_t,
    pub fpu_fsw: fp_status_t,
    pub fpu_ftw: u8,
    pub fpu_rsrv1: u8,
    pub fpu_fop: u16,
    pub fpu_ip: u32,
    pub fpu_cs: u16,
    pub fpu_rsrv2: u16,
    pub fpu_dp: u32,
    pub fpu_ds: u16,
    pub fpu_rsrv3: u16,
    pub fpu_mxcsr: u32,
    pub fpu_mxcsrmask: u32,
    pub fpu_stmm0: mmst_reg_t,
    pub fpu_stmm1: mmst_reg_t,
    pub fpu_stmm2: mmst_reg_t,
    pub fpu_stmm3: mmst_reg_t,
    pub fpu_stmm4: mmst_reg_t,
    pub fpu_stmm5: mmst_reg_t,
    pub fpu_stmm6: mmst_reg_t,
    pub fpu_stmm7: mmst_reg_t,
    pub fpu_xmm0: xmm_reg_t,
    pub fpu_xmm1: xmm_reg_t,
    pub fpu_xmm2: xmm_reg_t,
    pub fpu_xmm3: xmm_reg_t,
    pub fpu_xmm4: xmm_reg_t,
    pub fpu_xmm5: xmm_reg_t,
    pub fpu_xmm6: xmm_reg_t,
    pub fpu_xmm7: xmm_reg_t,
    pub fpu_xmm8: xmm_reg_t,
    pub fpu_xmm9: xmm_reg_t,
    pub fpu_xmm10: xmm_reg_t,
    pub fpu_xmm11: xmm_reg_t,
    pub fpu_xmm12: xmm_reg_t,
    pub fpu_xmm13: xmm_reg_t,
    pub fpu_xmm14: xmm_reg_t,
    pub fpu_xmm15: xmm_reg_t,
    pub fpu_rsrv4: [i8; 6 * 16],
    pub fpu_reserved1: i32,
    pub avx_reserved1: [i8; 64],
    pub fpu_ymmh0: xmm_reg_t,
    pub fpu_ymmh1: xmm_reg_t,
    pub fpu_ymmh2: xmm_reg_t,
    pub fpu_ymmh3: xmm_reg_t,
    pub fpu_ymmh4: xmm_reg_t,
    pub fpu_ymmh5: xmm_reg_t,
    pub fpu_ymmh6: xmm_reg_t,
    pub fpu_ymmh7: xmm_reg_t,
    pub fpu_ymmh8: xmm_reg_t,
    pub fpu_ymmh9: xmm_reg_t,
    pub fpu_ymmh10: xmm_reg_t,
    pub fpu_ymmh11: xmm_reg_t,
    pub fpu_ymmh12: xmm_reg_t,
    pub fpu_ymmh13: xmm_reg_t,
    pub fpu_ymmh14: xmm_reg_t,
    pub fpu_ymmh15: xmm_reg_t,
}

impl x86_avx_state64_t {
    pub fn new() -> Self {
        Self {
            fpu_reserved: [0; 2],
            fpu_fcw: Default::default(),
            fpu_fsw: Default::default(),
            fpu_ftw: 0,
            fpu_rsrv1: 0,
            fpu_fop: 0,
            fpu_ip: 0,
            fpu_cs: 0,
            fpu_rsrv2: 0,
            fpu_dp: 0,
            fpu_ds: 0,
            fpu_rsrv3: 0,
            fpu_mxcsr: 0,
            fpu_mxcsrmask: 0,
            fpu_stmm0: Default::default(),
            fpu_stmm1: Default::default(),
            fpu_stmm2: Default::default(),
            fpu_stmm3: Default::default(),
            fpu_stmm4: Default::default(),
            fpu_stmm5: Default::default(),
            fpu_stmm6: Default::default(),
            fpu_stmm7: Default::default(),
            fpu_xmm0: Default::default(),
            fpu_xmm1: Default::default(),
            fpu_xmm2: Default::default(),
            fpu_xmm3: Default::default(),
            fpu_xmm4: Default::default(),
            fpu_xmm5: Default::default(),
            fpu_xmm6: Default::default(),
            fpu_xmm7: Default::default(),
            fpu_xmm8: Default::default(),
            fpu_xmm9: Default::default(),
            fpu_xmm10: Default::default(),
            fpu_xmm11: Default::default(),
            fpu_xmm12: Default::default(),
            fpu_xmm13: Default::default(),
            fpu_xmm14: Default::default(),
            fpu_xmm15: Default::default(),
            fpu_rsrv4: [0; 6 * 16],
            fpu_reserved1: 0,
            avx_reserved1: [0; 64],
            fpu_ymmh0: Default::default(),
            fpu_ymmh1: Default::default(),
            fpu_ymmh2: Default::default(),
            fpu_ymmh3: Default::default(),
            fpu_ymmh4: Default::default(),
            fpu_ymmh5: Default::default(),
            fpu_ymmh6: Default::default(),
            fpu_ymmh7: Default::default(),
            fpu_ymmh8: Default::default(),
            fpu_ymmh9: Default::default(),
            fpu_ymmh10: Default::default(),
            fpu_ymmh11: Default::default(),
            fpu_ymmh12: Default::default(),
            fpu_ymmh13: Default::default(),
            fpu_ymmh14: Default::default(),
            fpu_ymmh15: Default::default(),
        }
    }

    pub fn count() -> mach_msg_type_number_t {
        (mem::size_of::<Self>() / mem::size_of::<::libc::c_int>()) as mach_msg_type_number_t
    }
}

impl Default for x86_avx_state64_t {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for x86_avx_state64_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("x86_avx_state64_t");
        ds.field("fpu_reserved", &self.fpu_reserved);
        ds.field("fpu_fcw", &self.fpu_fcw);
        ds.field("fpu_fsw", &self.fpu_fsw);
        ds.field("fpu_ftw", &self.fpu_ftw);
        ds.field("fpu_rsrv1", &self.fpu_rsrv1);
        ds.field("fpu_fop", &self.fpu_fop);
        ds.field("fpu_ip", &self.fpu_ip);
        ds.field("fpu_cs", &self.fpu_cs);
        ds.field("fpu_rsrv2", &self.fpu_rsrv2);
        ds.field("fpu_dp", &self.fpu_dp);
        ds.field("fpu_ds", &self.fpu_ds);
        ds.field("fpu_rsrv3", &self.fpu_rsrv3);
        ds.field("fpu_mxcsr", &self.fpu_mxcsr);
        ds.field("fpu_mxcsrmask", &self.fpu_mxcsrmask);
        ds.field("fpu_stmm0", &self.fpu_stmm0);
        ds.field("fpu_stmm1", &self.fpu_stmm1);
        ds.field("fpu_stmm2", &self.fpu_stmm2);
        ds.field("fpu_stmm3", &self.fpu_stmm3);
        ds.field("fpu_stmm4", &self.fpu_stmm4);
        ds.field("fpu_stmm5", &self.fpu_stmm5);
        ds.field("fpu_stmm6", &self.fpu_stmm6);
        ds.field("fpu_stmm7", &self.fpu_stmm7);
        ds.field("fpu_xmm0", &self.fpu_xmm0);
        ds.field("fpu_xmm1", &self.fpu_xmm1);
        ds.field("fpu_xmm2", &self.fpu_xmm2);
        ds.field("fpu_xmm3", &self.fpu_xmm3);
        ds.field("fpu_xmm4", &self.fpu_xmm4);
        ds.field("fpu_xmm5", &self.fpu_xmm5);
        ds.field("fpu_xmm6", &self.fpu_xmm6);
        ds.field("fpu_xmm7", &self.fpu_xmm7);
        ds.field("fpu_xmm8", &self.fpu_xmm8);
        ds.field("fpu_xmm9", &self.fpu_xmm9);
        ds.field("fpu_xmm10", &self.fpu_xmm10);
        ds.field("fpu_xmm11", &self.fpu_xmm11);
        ds.field("fpu_xmm12", &self.fpu_xmm12);
        ds.field("fpu_xmm13", &self.fpu_xmm13);
        ds.field("fpu_xmm14", &self.fpu_xmm14);
        ds.field("fpu_xmm15", &self.fpu_xmm15);
        ds.field("fpu_rsrv4", &(&self.fpu_rsrv4 as &[i8]));
        ds.field("fpu_reserved1", &self.fpu_reserved1);
        ds.field("avx_reserved1", &(&self.avx_reserved1 as &[i8]));
        ds.field("fpu_ymmh0", &self.fpu_ymmh0);
        ds.field("fpu_ymmh1", &self.fpu_ymmh1);
        ds.field("fpu_ymmh2", &self.fpu_ymmh2);
        ds.field("fpu_ymmh3", &self.fpu_ymmh3);
        ds.field("fpu_ymmh4", &self.fpu_ymmh4);
        ds.field("fpu_ymmh5", &self.fpu_ymmh5);
        ds.field("fpu_ymmh6", &self.fpu_ymmh6);
        ds.field("fpu_ymmh7", &self.fpu_ymmh7);
        ds.field("fpu_ymmh8", &self.fpu_ymmh8);
        ds.field("fpu_ymmh9", &self.fpu_ymmh9);
        ds.field("fpu_ymmh10", &self.fpu_ymmh10);
        ds.field("fpu_ymmh11", &self.fpu_ymmh11);
        ds.field("fpu_ymmh12", &self.fpu_ymmh12);
        ds.field("fpu_ymmh13", &self.fpu_ymmh13);
        ds.field("fpu_ymmh14", &self.fpu_ymmh14);
        ds.field("fpu_ymmh15", &self.fpu_ymmh15);
        ds.finish()
    }
}

impl PartialEq for x86_avx_state64_t {
    fn eq(&self, other: &Self) -> bool {
        self.fpu_reserved == other.fpu_reserved
            && self.fpu_fcw == other.fpu_fcw
            && self.fpu_fsw == other.fpu_fsw
            && self.fpu_ftw == other.fpu_ftw
            && self.fpu_rsrv1 == other.fpu_rsrv1
            && self.fpu_fop == other.fpu_fop
            && self.fpu_ip == other.fpu_ip
            && self.fpu_cs == other.fpu_cs
            && self.fpu_rsrv2 == other.fpu_rsrv2
            && self.fpu_dp == other.fpu_dp
            && self.fpu_ds == other.fpu_ds
            && self.fpu_rsrv3 == other.fpu_rsrv3
            && self.fpu_mxcsr == other.fpu_mxcsr
            && self.fpu_mxcsrmask == other.fpu_mxcsrmask
            && self.fpu_stmm0 == other.fpu_stmm0
            && self.fpu_stmm1 == other.fpu_stmm1
            && self.fpu_stmm2 == other.fpu_stmm2
            && self.fpu_stmm3 == other.fpu_stmm3
            && self.fpu_stmm4 == other.fpu_stmm4
            && self.fpu_stmm5 == other.fpu_stmm5
            && self.fpu_stmm6 == other.fpu_stmm6
            && self.fpu_stmm7 == other.fpu_stmm7
            && self.fpu_xmm0 == other.fpu_xmm0
            && self.fpu_xmm1 == other.fpu_xmm1
            && self.fpu_xmm2 == other.fpu_xmm2
            && self.fpu_xmm3 == other.fpu_xmm3
            && self.fpu_xmm4 == other.fpu_xmm4
            && self.fpu_xmm5 == other.fpu_xmm5
            && self.fpu_xmm6 == other.fpu_xmm6
            && self.fpu_xmm7 == other.fpu_xmm7
            && self.fpu_xmm8 == other.fpu_xmm8
            && self.fpu_xmm9 == other.fpu_xmm9
            && self.fpu_xmm10 == other.fpu_xmm10
            && self.fpu_xmm11 == other.fpu_xmm11
            && self.fpu_xmm12 == other.fpu_xmm12
            && self.fpu_xmm13 == other.fpu_xmm13
            && self.fpu_xmm14 == other.fpu_xmm14
            && self.fpu_xmm15 == other.fpu_xmm15
            && <[i8] as PartialEq<[i8]>>::eq(&self.fpu_rsrv4, &other.fpu_rsrv4)
            && self.fpu_reserved1 == other.fpu_reserved1
            && <[i8] as PartialEq<[i8]>>::eq(&self.avx_reserved1, &other.avx_reserved1)
            && self.fpu_ymmh0 == other.fpu_ymmh0
            && self.fpu_ymmh1 == other.fpu_ymmh1
            && self.fpu_ymmh2 == other.fpu_ymmh2
            && self.fpu_ymmh3 == other.fpu_ymmh3
            && self.fpu_ymmh4 == other.fpu_ymmh4
            && self.fpu_ymmh5 == other.fpu_ymmh5
            && self.fpu_ymmh6 == other.fpu_ymmh6
            && self.fpu_ymmh7 == other.fpu_ymmh7
            && self.fpu_ymmh8 == other.fpu_ymmh8
            && self.fpu_ymmh9 == other.fpu_ymmh9
            && self.fpu_ymmh10 == other.fpu_ymmh10
            && self.fpu_ymmh11 == other.fpu_ymmh11
            && self.fpu_ymmh12 == other.fpu_ymmh12
            && self.fpu_ymmh13 == other.fpu_ymmh13
            && self.fpu_ymmh14 == other.fpu_ymmh14
            && self.fpu_ymmh15 == other.fpu_ymmh15
    }
}

impl Eq for x86_avx_state64_t {}

impl Hash for x86_avx_state64_t {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fpu_reserved.hash(state);
        self.fpu_fcw.hash(state);
        self.fpu_fsw.hash(state);
        self.fpu_ftw.hash(state);
        self.fpu_rsrv1.hash(state);
        self.fpu_fop.hash(state);
        self.fpu_ip.hash(state);
        self.fpu_cs.hash(state);
        self.fpu_rsrv2.hash(state);
        self.fpu_dp.hash(state);
        self.fpu_ds.hash(state);
        self.fpu_rsrv3.hash(state);
        self.fpu_mxcsr.hash(state);
        self.fpu_mxcsrmask.hash(state);
        self.fpu_stmm0.hash(state);
        self.fpu_stmm1.hash(state);
        self.fpu_stmm2.hash(state);
        self.fpu_stmm3.hash(state);
        self.fpu_stmm4.hash(state);
        self.fpu_stmm5.hash(state);
        self.fpu_stmm6.hash(state);
        self.fpu_stmm7.hash(state);
        self.fpu_xmm0.hash(state);
        self.fpu_xmm1.hash(state);
        self.fpu_xmm2.hash(state);
        self.fpu_xmm3.hash(state);
        self.fpu_xmm4.hash(state);
        self.fpu_xmm5.hash(state);
        self.fpu_xmm6.hash(state);
        self.fpu_xmm7.hash(state);
        self.fpu_xmm8.hash(state);
        self.fpu_xmm9.hash(state);
        self.fpu_xmm10.hash(state);
        self.fpu_xmm11.hash(state);
        self.fpu_xmm12.hash(state);
        self.fpu_xmm13.hash(state);
        self.fpu_xmm14.hash(state);
        self.fpu_xmm15.hash(state);
        (&self.fpu_rsrv4 as &[i8]).hash(state);
        self.fpu_reserved1.hash(state);
        (&self.avx_reserved1 as &[i8]).hash(state);
        self.fpu_ymmh0.hash(state);
        self.fpu_ymmh1.hash(state);
        self.fpu_ymmh2.hash(state);
        self.fpu_ymmh3.hash(state);
        self.fpu_ymmh4.hash(state);
        self.fpu_ymmh5.hash(state);
        self.fpu_ymmh6.hash(state);
        self.fpu_ymmh7.hash(state);
        self.fpu_ymmh8.hash(state);
        self.fpu_ymmh9.hash(state);
        self.fpu_ymmh10.hash(state);
        self.fpu_ymmh11.hash(state);
        self.fpu_ymmh12.hash(state);
        self.fpu_ymmh13.hash(state);
        self.fpu_ymmh14.hash(state);
        self.fpu_ymmh15.hash(state);
    }
}

impl PartialOrd for x86_avx_state64_t {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for x86_avx_state64_t {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.fpu_reserved.cmp(&other.fpu_reserved) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_fcw.cmp(&other.fpu_fcw) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_fsw.cmp(&other.fpu_fsw) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ftw.cmp(&other.fpu_ftw) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv1.cmp(&other.fpu_rsrv1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_fop.cmp(&other.fpu_fop) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ip.cmp(&other.fpu_ip) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_cs.cmp(&other.fpu_cs) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv2.cmp(&other.fpu_rsrv2) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_dp.cmp(&other.fpu_dp) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ds.cmp(&other.fpu_ds) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv3.cmp(&other.fpu_rsrv3) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_mxcsr.cmp(&other.fpu_mxcsr) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_mxcsrmask.cmp(&other.fpu_mxcsrmask) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm0.cmp(&other.fpu_stmm0) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm1.cmp(&other.fpu_stmm1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm2.cmp(&other.fpu_stmm2) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm3.cmp(&other.fpu_stmm3) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm4.cmp(&other.fpu_stmm4) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm5.cmp(&other.fpu_stmm5) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm6.cmp(&other.fpu_stmm6) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_stmm7.cmp(&other.fpu_stmm7) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm0.cmp(&other.fpu_xmm0) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm1.cmp(&other.fpu_xmm1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm2.cmp(&other.fpu_xmm2) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm3.cmp(&other.fpu_xmm3) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm4.cmp(&other.fpu_xmm4) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm5.cmp(&other.fpu_xmm5) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm6.cmp(&other.fpu_xmm6) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm7.cmp(&other.fpu_xmm7) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm8.cmp(&other.fpu_xmm8) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm9.cmp(&other.fpu_xmm9) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm10.cmp(&other.fpu_xmm10) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm11.cmp(&other.fpu_xmm11) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm12.cmp(&other.fpu_xmm12) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm13.cmp(&other.fpu_xmm13) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm14.cmp(&other.fpu_xmm14) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_xmm15.cmp(&other.fpu_xmm15) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_rsrv4.cmp(&other.fpu_rsrv4) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_reserved1.cmp(&other.fpu_reserved1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.avx_reserved1.cmp(&other.avx_reserved1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh0.cmp(&other.fpu_ymmh0) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh1.cmp(&other.fpu_ymmh1) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh2.cmp(&other.fpu_ymmh2) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh3.cmp(&other.fpu_ymmh3) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh4.cmp(&other.fpu_ymmh4) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh5.cmp(&other.fpu_ymmh5) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh6.cmp(&other.fpu_ymmh6) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh7.cmp(&other.fpu_ymmh7) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh8.cmp(&other.fpu_ymmh8) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh9.cmp(&other.fpu_ymmh9) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh10.cmp(&other.fpu_ymmh10) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh11.cmp(&other.fpu_ymmh11) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh12.cmp(&other.fpu_ymmh12) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh13.cmp(&other.fpu_ymmh13) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh14.cmp(&other.fpu_ymmh14) {
            Ordering::Equal => (),
            ord => return ord,
        }
        match self.fpu_ymmh15.cmp(&other.fpu_ymmh15) {
            Ordering::Equal => (),
            ord => return ord,
        }

        Ordering::Equal
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct x86_debug_state32_t {
    pub dr0: u32,
    pub dr1: u32,
    pub dr2: u32,
    pub dr3: u32,
    pub dr4: u32,
    pub dr5: u32,
    pub dr6: u32,
    pub dr7: u32,
}

impl x86_debug_state32_t {
    pub fn count() -> mach_msg_type_number_t {
        (mem::size_of::<Self>() / mem::size_of::<::libc::c_int>()) as mach_msg_type_number_t
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct x86_debug_state64_t {
    pub dr0: u64,
    pub dr1: u64,
    pub dr2: u64,
    pub dr3: u64,
    pub dr4: u64,
    pub dr5: u64,
    pub dr6: u64,
    pub dr7: u64,
}

impl x86_debug_state64_t {
    pub fn count() -> mach_msg_type_number_t {
        (mem::size_of::<Self>() / mem::size_of::<::libc::c_int>()) as mach_msg_type_number_t
    }
}
