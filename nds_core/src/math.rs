/// Division control register.
const DIV_CONTROL: *mut u16 = 0x0400_0280 as _;
/// Division numerator register.
const DIV_NUMERATOR: *mut i64 = 0x0400_0290 as _;
/// Division denominator register.
const DIV_DENOMINATOR: *mut i64 = 0x0400_0298 as _;
/// Division result register.
const DIV_RESULT: *const i64 = 0x0400_02A0 as _;
/// Division remainder register.
const DIV_REMAINDER: *const i64 = 0x0400_02A8 as _;

/// Square root control register.
const SQRT_CONTROL: *mut u16 = 0x0400_02B0 as _;
/// Square root input register.
const SQRT_PARAM: *mut u64 = 0x0400_02B8 as _;
/// Square root result register.
const SQRT_RESULT: *const u32 = 0x0400_02B4 as _;

// #[inline(always)]
pub fn div_f32(numerator: i32, denominator: i32) -> i32 {
    unsafe { DIV_NUMERATOR.write_volatile((numerator as i64) << 12) };
    unsafe { (DIV_DENOMINATOR as *mut i32).write_volatile(denominator) };

    if unsafe { DIV_CONTROL.read_volatile() } & 3 != 1 {
        unsafe { DIV_CONTROL.write_volatile(1) };
    }

    while unsafe { DIV_CONTROL.read_volatile() } & 1 << 15 != 0 {}

    unsafe { (DIV_RESULT as *mut i32).read_volatile() }
}

// #[inline(always)]
// pub fn div_f32(numerator: i32, denominator: i32) -> i32 {
//     unsafe { nds_sys::divf32(numerator, denominator) }
// }

// #[inline(always)]
// pub fn mul_f32(lhs: i32, rhs: i32) -> i32 {
//     unsafe { nds_sys::mulf32(lhs, rhs) }
// }

// #[inline(always)]
// pub fn sqrt_f32(number: u32) -> u32 {
//     unsafe { nds_sys::sqrt32(number) }
// }

// #[inline(always)]
// pub fn div_i32(numerator: i32, denominator: i32) -> i32 {
//     unsafe { nds_sys::div32(numerator, denominator) }
// }

// #[inline(always)]
// pub fn mod_i32(numerator: i32, denominator: i32) -> i32 {
//     unsafe { nds_sys::mod32(numerator, denominator) }
// }
