#[repr(u32)]
pub enum Mode {
    M0_2D = nds_sys::VideoMode_MODE_0_2D,
    M1_2D = nds_sys::VideoMode_MODE_1_2D,
    M2_2D = nds_sys::VideoMode_MODE_2_2D,
}

#[inline(always)]
pub fn set_mode(mode: Mode) {
    const DISPCNT: *mut u32 = 0x0400_0000 as *mut u32;
    unsafe { DISPCNT.write_volatile(mode as u32) };
}

#[inline(always)]
pub fn set_mode_sub(mode: Mode) {
    const DISPCNT_SUB: *mut u32 = 0x0400_1000 as *mut u32;
    unsafe { DISPCNT_SUB.write_volatile(mode as u32) }
}

#[repr(transparent)]
pub struct Bank(*mut u8);

impl Bank {
    pub const A: Self = Self(0x0400_0240 as *mut u8);
    pub const B: Self = Self(0x0400_0241 as *mut u8);
    pub const C: Self = Self(0x0400_0242 as *mut u8);
    pub const D: Self = Self(0x0400_0243 as *mut u8);
    pub const E: Self = Self(0x0400_0244 as *mut u8);
    pub const F: Self = Self(0x0400_0245 as *mut u8);
    pub const G: Self = Self(0x0400_0246 as *mut u8);
    pub const H: Self = Self(0x0400_0248 as *mut u8);
    pub const I: Self = Self(0x0400_0249 as *mut u8);
}
