//! Synchronization primitives.

pub mod atomic;

mod arc;
mod mutex;

pub use arc::*;
pub use mutex::*;
