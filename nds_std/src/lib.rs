#![no_std]
#![feature(unsize, coerce_unsized)]
#![feature(dispatch_from_dyn)]
#![feature(negative_impls)]
#![feature(allocator_api)]
#![feature(thread_local)]
#![feature(decl_macro)]
#![feature(f16, f128)]

extern crate alloc;

pub mod sync;
pub mod thread;
