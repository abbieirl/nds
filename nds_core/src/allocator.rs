//! Memory allocation APIs.

use core::alloc::{GlobalAlloc, Layout};
use nds_sys::{aligned_alloc, free};

#[cfg(feature = "alloc")]
#[global_allocator]
static ALLOC: NdsAllocator = NdsAllocator;

pub struct NdsAllocator;

unsafe impl GlobalAlloc for NdsAllocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { aligned_alloc(layout.align() as u32, layout.size() as u32).cast() }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { free(ptr.cast()) };
    }
}
