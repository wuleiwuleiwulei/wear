use crate::error::BorrowError;
use crate::{BORROW_FLAG, INIT_STATE};
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::ops::{Deref, DerefMut};
use core::sync::atomic::AtomicIsize;
use core::sync::atomic::Ordering::Relaxed;

#[inline(always)]
fn already_borrow_mut(x: isize) -> bool {
    x < INIT_STATE
}

pub(crate) struct AtomicRefMutInner<'a> {
    borrow: &'a AtomicIsize,
}

impl Drop for AtomicRefMutInner<'_> {
    #[inline]
    fn drop(&mut self) {
        self.borrow.fetch_sub(BORROW_FLAG, Relaxed);
    }
}

impl<'a> AtomicRefMutInner<'a> {
    pub(crate) fn try_borrow_mut(
        borrow: &'a AtomicIsize,
    ) -> Result<AtomicRefMutInner<'a>, BorrowError> {
        let b = match borrow.compare_exchange(0, BORROW_FLAG, Relaxed, Relaxed) {
            Ok(v) => v,
            Err(v) => v,
        };
        if b == INIT_STATE {
            Ok(AtomicRefMutInner { borrow })
        } else if already_borrow_mut(b) {
            Err(BorrowError::AlreadyBorrowMut)
        } else {
            Err(BorrowError::AlreadyBorrow)
        }
    }
}

pub struct AtomicRefMut<'a, T: ?Sized + 'a> {
    pub(crate) value: &'a mut T,
    pub(crate) _borrow: AtomicRefMutInner<'a>,
}

impl<T: ?Sized> Deref for AtomicRefMut<'_, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        self.value
    }
}

impl<T: ?Sized> DerefMut for AtomicRefMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}

impl<'a, T: ?Sized + Debug + 'a> Debug for AtomicRefMut<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: ?Sized + PartialEq> PartialEq for AtomicRefMut<'_, T> {
    #[inline]
    fn eq(&self, o: &AtomicRefMut<T>) -> bool {
        self.value == o.value
    }
}

impl<T: ?Sized + PartialEq> Eq for AtomicRefMut<'_, T> {}
