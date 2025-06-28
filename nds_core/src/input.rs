//! Physical input related APIs.

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};
use nds_sys::KEYPAD_BITS;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keys(KEYPAD_BITS);

impl Keys {
    /// Keypad `A` button.
    pub const A: Self = Self(KEYPAD_BITS::KEY_A);
    /// Keypad `B` button.
    pub const B: Self = Self(KEYPAD_BITS::KEY_B);
    /// Keypad `SELECT` button.
    pub const SELECT: Self = Self(KEYPAD_BITS::KEY_SELECT);
    /// Keypad `START` button.
    pub const START: Self = Self(KEYPAD_BITS::KEY_START);
    /// Keypad `RIGHT` button.
    pub const RIGHT: Self = Self(KEYPAD_BITS::KEY_RIGHT);
    /// Keypad `LEFT` button.
    pub const LEFT: Self = Self(KEYPAD_BITS::KEY_LEFT);
    /// Keypad `UP` button.
    pub const UP: Self = Self(KEYPAD_BITS::KEY_UP);
    /// Keypad `DOWN` button.
    pub const DOWN: Self = Self(KEYPAD_BITS::KEY_DOWN);
    /// Keypad `R` (right shoulder) button.
    pub const R: Self = Self(KEYPAD_BITS::KEY_R);
    /// Keypad `L` (left shoulder) button.
    pub const L: Self = Self(KEYPAD_BITS::KEY_L);
    /// Keypad `X` button.
    pub const X: Self = Self(KEYPAD_BITS::KEY_X);
    /// Keypad `Y` button.
    pub const Y: Self = Self(KEYPAD_BITS::KEY_Y);
    /// Touchscreen `TOUCH` pendown.
    pub const TOUCH: Self = Self(KEYPAD_BITS::KEY_TOUCH);
    /// Handheld `LID` state.
    pub const LID: Self = Self(KEYPAD_BITS::KEY_LID);
    /// Debug `DEBUG` button.
    pub const DEBUG: Self = Self(KEYPAD_BITS::KEY_DEBUG);

    /// Obtains the current keypad state.
    ///
    /// Call this function once per main loop in order to use the keypad functions.
    #[inline(always)]
    pub fn scan() {
        // SAFETY: todo!()
        unsafe { nds_sys::scanKeys() };
    }

    /// Obtains the keys that have been pressed right now.
    #[inline(always)]
    pub fn down() -> Self {
        // SAFETY: todo!()
        unsafe { Self(KEYPAD_BITS(nds_sys::keysDown())) }
    }

    /// Obtains the current keypad held state.
    #[inline(always)]
    pub fn held() -> Self {
        // SAFETY: todo!()
        unsafe { Self(KEYPAD_BITS(nds_sys::keysHeld())) }
    }

    #[inline]
    pub fn contains(self, rhs: Keys) -> bool {
        (self.0.0 & rhs.0.0) != 0
    }
}

impl BitOr for Keys {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Keys {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Keys {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Keys {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
