//! Traits, helpers, and type definitions for core I/O functionality.

use crate::background::{Size, Type};
use alloc::ffi::CString;
use core::fmt::{Error, Result, Write};
use core::mem::zeroed;
use nds_sys::{ConsoleColor, PrintConsole};

/// Colors of the default palettes of libnds.
pub type Color = ConsoleColor;

/// Console structure used to store the state of a console render context.
#[repr(transparent)]
pub struct Console(*mut PrintConsole);

impl Console {
    /// Initialize a new console on the top screen.
    #[inline]
    pub fn top(layer: i32, kind: Type, size: Size) -> Self {
        let kind = kind as u32;
        let size = size as u32;

        Self(unsafe { nds_sys::consoleInit(zeroed(), layer, kind, size, 31, 0, true, true) })
    }

    /// Initialize a new console on the bottom screen.
    #[inline]
    pub fn bottom(layer: i32, kind: Type, size: Size) -> Self {
        let kind = kind as u32;
        let size = size as u32;

        Self(unsafe { nds_sys::consoleInit(zeroed(), layer, kind, size, 31, 0, false, true) })
    }

    /// Initialize the console to a default state for prototyping.
    ///
    /// This function sets the console to use sub display, VRAM_C, and BG0 and enables MODE_0_2D on the sub display.\
    /// It is intended for use in prototyping applications which need print ability and not actual game use.\
    /// Print functionality can be utilized with just this call.
    #[inline(always)]
    pub fn demo() -> Self {
        Self(unsafe { nds_sys::consoleDemoInit() })
    }

    /// Make the specified console the render target.
    #[inline]
    pub fn select(&self) {
        unsafe { nds_sys::consoleSelect(self.0) };
    }

    /// Sets the color to use to print new text.
    #[inline]
    pub fn set_color(&mut self, color: Color) {
        unsafe { nds_sys::consoleSetColor(self.0, color) };
    }

    /// Clears the console and returns the cursor to the top left corner.
    #[inline]
    pub fn clear(&mut self) {
        self.select();
        unsafe { nds_sys::consoleClear() };
    }
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> Result {
        self.select();

        let c_str = CString::new(s).map_err(|_| Error)?;
        unsafe { nds_sys::printf(c_str.as_ptr()) };

        Ok(())
    }
}
