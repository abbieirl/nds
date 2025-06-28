use crate::sync::atomic::sealed::Sealed;
use core::cell::UnsafeCell;
use nds_core::interrupt::critical_section;

/// A memory location which can be safely modified from multiple threads.
#[repr(transparent)]
pub struct Atomic<T: Primitive> {
    v: UnsafeCell<T>,
}

unsafe impl<T: Primitive> Sync for Atomic<T> {}

impl<T: Primitive> Atomic<T> {
    /// Creates a new `Atomic`.
    #[inline]
    #[must_use]
    pub const fn new(v: T) -> Self {
        Self {
            v: UnsafeCell::new(v),
        }
    }

    #[inline]
    pub const unsafe fn from_ptr<'a>(ptr: *mut T) -> &'a Self {
        unsafe { &*ptr.cast() }
    }

    /// Returns a mutable reference to the underlying value.
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        // SAFETY: The mutable reference guarantees unique ownership.
        unsafe { &mut *(self.v.get().cast()) }
    }

    /// Creates a new `Atomic` from a pointer.
    #[inline]
    pub const fn into_inner(self) -> T {
        self.v.into_inner()
    }

    #[inline]
    pub fn load(&self) -> T {
        critical_section(|| unsafe { self.v.get().read() })
    }

    #[inline]
    pub fn store(&self, val: T) {
        critical_section(|| unsafe { self.v.get().write(val) });
    }

    #[inline]
    pub fn swap(&self, val: T) -> T {
        critical_section(|| unsafe { self.v.get().replace(val) })
    }

    #[inline]
    pub fn compare_exchange(&self, current: T, new: T) -> Result<T, T>
    where
        T: PartialEq,
    {
        critical_section(|| unsafe {
            let old = self.v.get().read();
            match old == current {
                true => Ok(self.v.get().replace(new)),
                false => Err(old),
            }
        })
    }

    #[inline]
    pub fn fetch_update<F>(&self, mut f: F) -> Result<T, T>
    where
        F: FnMut(T) -> Option<T>,
    {
        critical_section(|| unsafe {
            let old = *self.v.get();
            f(old).map_or(Err(old), |new| {
                self.v.get().write(new);
                Ok(old)
            })
        })
    }
}

impl Atomic<bool> {
    #[inline]
    pub fn fetch_and(&self, val: bool) -> bool {
        todo!()
    }

    #[inline]
    pub fn fetch_or(&self, val: bool) -> bool {
        todo!()
    }

    #[inline]
    pub fn fetch_xor(&self, val: bool) -> bool {
        todo!()
    }
}

impl<T: Primitive + Default> Default for Atomic<T> {
    #[inline]
    fn default() -> Self {
        Self::new(T::default())
    }
}

pub trait Primitive: Sized + Copy + Sealed {}

impl_primitive!(f16, f32, f64, f128);
impl_primitive!(u8, u16, u32, u64, u128);
impl_primitive!(i8, i16, i32, i64, i128);
impl_primitive!(usize, isize, bool, *mut T);

macro impl_primitive {
    // Base case
    () => {},

    // Match *mut T pattern (generic pointer)
    (*mut $t:ident, $($rest:tt)*) => {
        impl<$t> Sealed for *mut $t {}
        impl<$t> Primitive for *mut $t {}
        impl_primitive!($($rest)*);
    },

    // Match concrete type pattern
    ($ty:ty, $($rest:tt)*) => {
        impl Sealed for $ty {}
        impl Primitive for $ty {}
        impl_primitive!($($rest)*);
    },

    // Last single item (generic mut pointer)
    (*mut $t:ident) => {
        impl<$t> Sealed for *mut $t {}
        impl<$t> Primitive for *mut $t {}
    },

    // Last single item (concrete)
    ($ty:ty) => {
        impl Sealed for $ty {}
        impl Primitive for $ty {}
    },
}

mod sealed {
    pub trait Sealed {}
}
