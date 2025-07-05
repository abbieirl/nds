//! # NDS

#![no_std]

extern crate alloc;

pub mod allocator;
pub mod background;
pub mod console;
pub mod input;
pub mod interrupt;
pub mod math;
pub mod process;
pub mod system;
pub mod video;

#[cfg(feature = "proc")]
pub use nds_proc::{dtcm, entry, itcm};
