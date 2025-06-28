use crate::sync::atomic::Atomic;
use alloc::alloc::{Allocator, Global};
use alloc::boxed::Box;
use core::marker::Unsize;
use core::mem::ManuallyDrop;
use core::ops::{CoerceUnsized, DispatchFromDyn};
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::{marker::PhantomData, ptr::NonNull};

pub struct Arc<T: ?Sized, A: Allocator = Global> {
    ptr: NonNull<ArcInner<T>>,
    phantom: PhantomData<ArcInner<T>>,
    alloc: A,
}

unsafe impl<T: ?Sized + Sync + Send, A: Allocator + Send> Send for Arc<T, A> {}
unsafe impl<T: ?Sized + Sync + Send, A: Allocator + Sync> Sync for Arc<T, A> {}

impl<T: RefUnwindSafe + ?Sized, A: Allocator + UnwindSafe> UnwindSafe for Arc<T, A> {}
impl<T: ?Sized + Unsize<U>, U: ?Sized, A: Allocator> CoerceUnsized<Arc<U, A>> for Arc<T, A> {}
impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<Arc<U>> for Arc<T> {}

impl<T> Arc<T> {
    const MAX_REFCOUNT: usize = (isize::MAX) as usize;

    #[inline]
    pub fn new(data: T) -> Self {
        let x: Box<_> = Box::new(ArcInner {
            strong: Atomic::new(1),
            weak: Atomic::new(1),
            data,
        });

        unsafe { Self::from_inner(Box::leak(x).into()) }
    }

    // pub fn pin(data: T) -> Pin<Arc<T>> {
    //     unsafe { Pin::new_unchecked(Arc::new(data)) }
    // }
}

impl<T: ?Sized> Arc<T> {
    unsafe fn from_inner(ptr: NonNull<ArcInner<T>>) -> Self {
        unsafe { Self::from_inner_in(ptr, Global) }
    }

    unsafe fn from_ptr(ptr: *mut ArcInner<T>) -> Self {
        unsafe { Self::from_ptr_in(ptr, Global) }
    }
}

impl<T: ?Sized, A: Allocator> Arc<T, A> {
    #[inline]
    fn into_inner_with_allocator(this: Self) -> (NonNull<ArcInner<T>>, A) {
        let this = ManuallyDrop::new(this);
        (this.ptr, unsafe { core::ptr::read(&this.alloc) })
    }

    #[inline]
    unsafe fn from_inner_in(ptr: NonNull<ArcInner<T>>, alloc: A) -> Self {
        Self {
            ptr,
            phantom: PhantomData,
            alloc,
        }
    }

    #[inline]
    unsafe fn from_ptr_in(ptr: *mut ArcInner<T>, alloc: A) -> Self {
        unsafe { Self::from_inner_in(NonNull::new_unchecked(ptr), alloc) }
    }
}

#[repr(C)]
struct ArcInner<T: ?Sized> {
    strong: Atomic<usize>,
    weak: Atomic<usize>,
    data: T,
}