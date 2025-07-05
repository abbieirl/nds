#![no_std]

/// System specific `todo!()`.
pub mod sys {
    pub use nds_core::*;
}

/// Custom std implementation.
#[cfg(feature = "std")]
pub mod std {
    pub use nds_std::*;
}

pub use sys::{dtcm, entry, itcm};
