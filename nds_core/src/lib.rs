//! # NDS

#![no_std]

extern crate alloc;

pub mod allocator;
pub mod input;
pub mod interrupt;
pub mod console;
pub mod process;
pub mod system;

#[cfg(feature = "proc")]
pub use nds_proc::{dtcm, entry, itcm};
