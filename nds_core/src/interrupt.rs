//! Interrupt synchronization APIs.

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

/// Wait for a vertical blank interrupt.
#[inline(always)]
pub fn swi_wait_for_vblank() {
    unsafe { nds_sys::swiWaitForVBlank() };
}

#[inline(always)]
pub fn swi_wait_for_irq() {
    unsafe { nds_sys::swiWaitForIRQ() };
}

/// Allow the given interrupt to occur.
#[inline(always)]
pub fn enable(mask: Interrupt) {
    unsafe { nds_sys::irqEnable(mask.0) };
}

/// Prevent the given interrupt from occuring.
#[inline(always)]
pub fn disable(mask: Interrupt) {
    unsafe { nds_sys::irqDisable(mask.0) };
}

/// Executes the given closure in a critical section by disabling IRQs (interrupts).
#[inline(always)]
pub fn critical_section<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    const IME: *mut u32 = 0x04000208 as *mut u32;

    // SAFETY: IME points to a valid hardware IRQ register.
    let ime = unsafe { IME.read_volatile() };

    // SAFETY: Disables interrupts without side effects.
    unsafe { IME.write_volatile(0) };
    let result = f();

    // SAFETY: IME points to a valid hardware IRQ register.
    unsafe { IME.write_volatile(ime) };
    result
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interrupt(u32);

impl Interrupt {
    /// Vertical blank interrupt mask.
    pub const VBLANK: Self = Self(1 << 0);
    /// Horizontal blank interrupt mask.
    pub const HBLANK: Self = Self(1 << 1);
    /// Vcount match interrupt mask.
    pub const VCOUNT: Self = Self(1 << 2);
    /// Timer 0 interrupt mask.
    pub const TIMER0: Self = Self(1 << 3);
    /// Timer 1 interrupt mask.
    pub const TIMER1: Self = Self(1 << 4);
    /// Timer 2 interrupt mask.
    pub const TIMER2: Self = Self(1 << 5);
    /// Timer 3 interrupt mask.
    pub const TIMER3: Self = Self(1 << 6);
    /// DMA 0 interrupt mask.
    pub const DMA0: Self = Self(1 << 8);
    /// DMA 1 interrupt mask.
    pub const DMA1: Self = Self(1 << 9);
    /// DMA 2 interrupt mask.
    pub const DMA2: Self = Self(1 << 10);
    /// DMA 3 interrupt mask.
    pub const DMA3: Self = Self(1 << 11);
    /// Keypad interrupt mask.
    pub const KEYS: Self = Self(1 << 12);
    /// GBA cartridge interrupt mask.
    pub const CART: Self = Self(1 << 13);
    /// IPC sync interrupt mask.
    pub const IPC_SYNC: Self = Self(1 << 16);
    /// Send FIFO empty interrupt mask.
    pub const FIFO_EMPTY: Self = Self(1 << 17);
    /// Receive FIFO not empty interrupt mask.
    pub const FIFO_NOT_EMPTY: Self = Self(1 << 18);
    /// Interrupt mask DS Card Slot.
    pub const CARD: Self = Self(1 << 19);
    /// Interrupt mask.
    pub const CARD_LINE: Self = Self(1 << 20);
}

impl BitOr for Interrupt {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Interrupt {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Interrupt {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Interrupt {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
