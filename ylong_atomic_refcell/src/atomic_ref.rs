use crate::error::BorrowError;
use crate::INIT_STATE;
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::ops::Deref;
use core::sync::atomic::AtomicIsize;
use core::sync::atomic::Ordering::Relaxed;

#[inline(always)]
fn is_borrow_success(x: isize) -> bool {
    x > INIT_STATE
}

pub struct AtomicRef<'a, T: ?Sized + 'a> {
    pub(crate) value: &'a T,
    pub(crate) borrow: AtomicRefInner<'a>,
}

impl<T: ?Sized> Deref for AtomicRef<'_, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        self.value
    }
}

impl<T: ?Sized> Clone for AtomicRef<'_, T> {
    fn clone(&self) -> Self {
        AtomicRef {
            value: self.value,
            borrow: self.borrow.clone(),
        }
    }
}

impl<T: ?Sized + PartialEq> PartialEq for AtomicRef<'_, T> {
    #[inline]
    fn eq(&self, o: &AtomicRef<T>) -> bool {
        self.value == o.value
    }
}

impl<T: ?Sized + PartialEq> Eq for AtomicRef<'_, T> {}

pub(crate) struct AtomicRefInner<'a> {
    borrow: &'a AtomicIsize,
}

impl<'a> Clone for AtomicRefInner<'a> {
    #[inline]
    fn clone(&self) -> AtomicRefInner<'a> {
        AtomicRefInner::try_borrow(self.borrow).unwrap()
    }
}

impl<'a> AtomicRefInner<'a> {
    pub(crate) fn try_borrow(borrow: &'a AtomicIsize) -> Result<Self, BorrowError> {
        let b = borrow.fetch_add(1, Relaxed) + 1;
        if is_borrow_success(b) {
            Ok(AtomicRefInner { borrow })
        } else {
            borrow.fetch_sub(1, Relaxed);
            Err(BorrowError::AlreadyBorrowMut)
        }
    }
}

impl Drop for AtomicRefInner<'_> {
    fn drop(&mut self) {
        let old = self.borrow.fetch_sub(1, Relaxed);
        debug_assert!(is_borrow_success(old));
    }
}

impl<'a, T: ?Sized + Debug + 'a> Debug for AtomicRef<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}
