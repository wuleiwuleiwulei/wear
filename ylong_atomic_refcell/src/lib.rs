#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unsafe_code)]

mod atomic_ref;
mod atomic_ref_mut;
mod error;

use atomic_ref::{AtomicRef, AtomicRefInner};
use atomic_ref_mut::{AtomicRefMut, AtomicRefMutInner};
use core::cell::UnsafeCell;
use core::cmp;
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::sync::atomic::AtomicIsize;
pub use error::BorrowError;

const BORROW_FLAG: isize = isize::MIN;
const INIT_STATE: isize = 0;

pub struct AtomicRefCell<T: ?Sized> {
    borrow: AtomicIsize,
    value: UnsafeCell<T>,
}

impl<T> AtomicRefCell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            borrow: AtomicIsize::new(INIT_STATE),
            value: UnsafeCell::new(value),
        }
    }
}

impl<T: ?Sized> AtomicRefCell<T> {
    pub fn borrow(&self) -> AtomicRef<T> {
        let borrow = AtomicRefInner::try_borrow(&self.borrow).expect("already mutably borrowed");
        AtomicRef {
            value: unsafe { &*self.value.get() },
            borrow,
        }
    }

    pub fn try_borrow(&self) -> Result<AtomicRef<T>, BorrowError> {
        let borrow = AtomicRefInner::try_borrow(&self.borrow)?;
        Ok(AtomicRef {
            value: unsafe { &*self.value.get() },
            borrow,
        })
    }

    pub fn borrow_mut(&self) -> AtomicRefMut<T> {
        let borrow = AtomicRefMutInner::try_borrow_mut(&self.borrow).expect("already borrowed");
        AtomicRefMut {
            value: unsafe { &mut *self.value.get() },
            _borrow: borrow,
        }
    }

    pub fn try_borrow_mut(&self) -> Result<AtomicRefMut<T>, BorrowError> {
        let borrow = AtomicRefMutInner::try_borrow_mut(&self.borrow)?;
        Ok(AtomicRefMut {
            value: unsafe { &mut *self.value.get() },
            _borrow: borrow,
        })
    }

    /// # Safety
    ///
    /// 此处代码不安全。
    #[inline]
    pub unsafe fn as_ptr(&self) -> *mut T {
        self.value.get()
    }
}

impl<T: ?Sized + Ord> Ord for AtomicRefCell<T> {
    #[inline]
    fn cmp(&self, other: &AtomicRefCell<T>) -> cmp::Ordering {
        self.borrow().cmp(&*other.borrow())
    }
}

impl<T: ?Sized + PartialEq> PartialEq for AtomicRefCell<T> {
    #[inline]
    fn eq(&self, o: &AtomicRefCell<T>) -> bool {
        *self.borrow() == *o.borrow()
    }
}

impl<T: ?Sized + PartialEq> Eq for AtomicRefCell<T> {}

impl<T: ?Sized + PartialOrd> PartialOrd for AtomicRefCell<T> {
    #[inline]
    fn partial_cmp(&self, other: &AtomicRefCell<T>) -> Option<cmp::Ordering> {
        self.borrow().partial_cmp(&*other.borrow())
    }
}

impl<T: Clone> Clone for AtomicRefCell<T> {
    fn clone(&self) -> AtomicRefCell<T> {
        AtomicRefCell::new((*self.borrow()).clone())
    }
}

impl<T: ?Sized + Debug> Debug for AtomicRefCell<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

unsafe impl<T: ?Sized + Send + Sync> Send for AtomicRefCell<T> {}

unsafe impl<T: ?Sized + Send + Sync> Sync for AtomicRefCell<T> {}
