//! This module corresponds to `mach/i386/_structs.h` and `mach/arm/_structs.h`.

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
#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Ord)]
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
            fpu_reserved1: 0
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
#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct i386_avx_state_t {
    fpu_reserved: [i32; 2],
    fpu_fcw: fp_control_t,
    fpu_fsw: fp_status_t,
   	fpu_ftw: u8,
   	fpu_rsrv1: u8,
   	fpu_fop: u16,
   	fpu_ip: u32,
   	fpu_cs: u16,
   	fpu_rsrv2: u16,
   	fpu_dp: u32,
   	fpu_ds: u16,
   	fpu_rsrv3: u16,
   	fpu_mxcsr: u32,
   	fpu_mxcsrmask: u32,
    fpu_stmm0: mmst_reg_t,
    fpu_stmm1: mmst_reg_t,
    fpu_stmm2: mmst_reg_t,
    fpu_stmm3: mmst_reg_t,
    fpu_stmm4: mmst_reg_t,
    fpu_stmm5: mmst_reg_t,
    fpu_stmm6: mmst_reg_t,
    fpu_stmm7: mmst_reg_t,
    fpu_xmm0: xmm_reg_t,
    fpu_xmm1: xmm_reg_t,
    fpu_xmm2: xmm_reg_t,
    fpu_xmm3: xmm_reg_t,
    fpu_xmm4: xmm_reg_t,
    fpu_xmm5: xmm_reg_t,
    fpu_xmm6: xmm_reg_t,
    fpu_xmm7: xmm_reg_t,
    fpu_rsrv4: [i8; 14 * 16],
    fpu_reserved1: i32,
    avx_reserved1: [i8; 64],
    fpu_ymmh0: xmm_reg_t,
    fpu_ymmh1: xmm_reg_t,
    fpu_ymmh2: xmm_reg_t,
    fpu_ymmh3: xmm_reg_t,
    fpu_ymmh4: xmm_reg_t,
    fpu_ymmh5: xmm_reg_t,
    fpu_ymmh6: xmm_reg_t,
    fpu_ymmh7: xmm_reg_t,
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
            fpu_ymmh7: Default::default()
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
impl x86_thread_state64_t {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn count() -> mach_msg_type_number_t {
        (mem::size_of::<Self>() / mem::size_of::<::libc::c_int>()) as mach_msg_type_number_t
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Ord)]
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
    pub fpu_rsrv4: [i8; 6*16],
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
            fpu_reserved1: 0
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

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct x86_exception_state64_t {
    pub trapno: u16,
    pub cpu: u16,
    pub err: u32,
    pub faultvaddr: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct x86_avx_state64_t {
    fpu_reserved: [i32; 2],
    fpu_fcw: fp_control_t,
    fpu_fsw: fp_status_t,
    fpu_ftw: u8,
    fpu_rsrv1: u8,
    fpu_fop: u16,
    fpu_ip: u32,
    fpu_cs: u16,
    fpu_rsrv2: u16,
    fpu_dp: u32,
    fpu_ds: u16,
    fpu_rsrv3: u16,
    fpu_mxcsr: u32,
    fpu_mxcsrmask: u32,
    fpu_stmm0: mmst_reg_t,
    fpu_stmm1: mmst_reg_t,
    fpu_stmm2: mmst_reg_t,
    fpu_stmm3: mmst_reg_t,
    fpu_stmm4: mmst_reg_t,
    fpu_stmm5: mmst_reg_t,
    fpu_stmm6: mmst_reg_t,
    fpu_stmm7: mmst_reg_t,
    fpu_xmm0: xmm_reg_t,
    fpu_xmm1: xmm_reg_t,
    fpu_xmm2: xmm_reg_t,
    fpu_xmm3: xmm_reg_t,
    fpu_xmm4: xmm_reg_t,
    fpu_xmm5: xmm_reg_t,
    fpu_xmm6: xmm_reg_t,
    fpu_xmm7: xmm_reg_t,
    fpu_xmm8: xmm_reg_t,
    fpu_xmm9: xmm_reg_t,
    fpu_xmm10: xmm_reg_t,
    fpu_xmm11: xmm_reg_t,
    fpu_xmm12: xmm_reg_t,
    fpu_xmm13: xmm_reg_t,
    fpu_xmm14: xmm_reg_t,
    fpu_xmm15: xmm_reg_t,
    fpu_rsrv4: [i8; 6 * 16],
    fpu_reserved1: i32,
    avx_reserved1: [i8; 64],
    fpu_ymmh0: xmm_reg_t,
    fpu_ymmh1: xmm_reg_t,
    fpu_ymmh2: xmm_reg_t,
    fpu_ymmh3: xmm_reg_t,
    fpu_ymmh4: xmm_reg_t,
    fpu_ymmh5: xmm_reg_t,
    fpu_ymmh6: xmm_reg_t,
    fpu_ymmh7: xmm_reg_t,
    fpu_ymmh8: xmm_reg_t,
    fpu_ymmh9: xmm_reg_t,
    fpu_ymmh10: xmm_reg_t,
    fpu_ymmh11: xmm_reg_t,
    fpu_ymmh12: xmm_reg_t,
    fpu_ymmh13: xmm_reg_t,
    fpu_ymmh14: xmm_reg_t,
    fpu_ymmh15: xmm_reg_t,
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
            fpu_ymmh15: Default::default()
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
