use crate::RcWithAlloc;
use alloc::alloc::{Allocator, Layout};
use alloc::rc::{is_dangling, RcBox, RcInnerPtr};
use core::fmt::{Debug, Formatter};
use core::ptr::NonNull;
use ylong_stdx_common::{ContainerError, SafeClone};

const INVALID_ADDR: usize = usize::MAX;

/// WeakWithAlloc that provides functions the same as std's Weak, but won't panic when OOM.
pub struct WeakWithAlloc<V: ?Sized, A: Allocator> {
    shared_inner: NonNull<RcBox<V>>,
    alloc: A,
}

impl<V, A: Allocator> WeakWithAlloc<V, A> {
    /// Creates a new WeakWithAlloc. The new instance won't allocate any memory. Upgrade on it will
    /// always return None.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_rc::WeakWithAlloc;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// let weak: WeakWithAlloc<(), System> = WeakWithAlloc::new(System::default());
    /// assert!(weak.upgrade()?.is_none());
    /// # Ok::<(), ContainerError>(())
    /// ```
    pub fn new(alloc: A) -> WeakWithAlloc<V, A> {
        // SAFETY:
        // Deliberately create an invalid pointer. It will be checked through `is_dangling` method
        // and avoid freeing on this invalid pointer.
        let share_inner =
            unsafe { NonNull::new_unchecked(core::ptr::invalid_mut::<RcBox<V>>(INVALID_ADDR)) };
        WeakWithAlloc {
            shared_inner: share_inner,
            alloc,
        }
    }
}

impl<V: ?Sized, A: Allocator> WeakWithAlloc<V, A> {
    #[inline]
    pub(crate) fn from_inner(inner: NonNull<RcBox<V>>, alloc: A) -> WeakWithAlloc<V, A> {
        WeakWithAlloc {
            shared_inner: inner,
            alloc,
        }
    }

    fn shared_inner(&self) -> Option<&RcBox<V>> {
        if is_dangling(self.shared_inner.as_ptr()) {
            None
        } else {
            // SAFETY:
            // Safe because all restrictions of nonnull are met:
            // 1. Pointer is initialized and aligned.
            // 2. Lifetime is bound by self
            let ret = unsafe { self.shared_inner.as_ref() };
            Some(ret)
        }
    }

    /// Gets a reference of the allocator in a WeakWithAlloc instance.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_rc::RcWithAlloc;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// let val = 5;
    /// let rc = RcWithAlloc::new(val, System::default())?;
    /// let weak = RcWithAlloc::downgrade(&rc)?;
    /// let alloc = weak.allocator();
    /// # Ok::<(), ContainerError>(())
    /// ```
    #[inline]
    pub fn allocator(&self) -> &A {
        &self.alloc
    }

    /// Gets the number of strong references of the WeakWithAlloc instance.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_rc::RcWithAlloc;
    /// use std::alloc::System;
    /// use ylong_stdx_common::{SafeClone, ContainerError};
    ///
    /// let val = 5;
    /// let rc = RcWithAlloc::new(val, System::default())?;
    /// let weak = RcWithAlloc::downgrade(&rc)?;
    /// assert_eq!(weak.strong_count(), 1);
    ///
    /// let _rc2 = rc.safe_clone()?;
    /// assert_eq!(weak.strong_count(), 2);
    /// # Ok::<(), ContainerError>(())
    /// ```
    #[inline]
    pub fn strong_count(&self) -> usize {
        self.shared_inner().map_or(0, |inner| inner.strong())
    }

    /// Gets the number of weak references of the RcWithAlloc instance.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_rc::RcWithAlloc;
    /// use std::alloc::System;
    /// use ylong_stdx_common::{SafeClone, ContainerError};
    ///
    /// let val = 5;
    /// let rc = RcWithAlloc::new(val, System::default())?;
    /// let weak = RcWithAlloc::downgrade(&rc)?;
    /// assert_eq!(weak.weak_count(), 1);
    ///
    /// let weak2 = RcWithAlloc::downgrade(&rc)?;
    /// assert_eq!(weak.weak_count(), 2);
    /// # Ok::<(), ContainerError>(())
    /// ```
    pub fn weak_count(&self) -> usize {
        // Decrease by 1 because of the self-pointing relations
        self.shared_inner().map_or(0, |inner| {
            if inner.strong() > 0 {
                inner.weak() - 1
            } else {
                0
            }
        })
    }
}

impl<V: ?Sized, A: Allocator + SafeClone> WeakWithAlloc<V, A> {
    /// Upgrades a weak reference to a strong reference. Returns None if the inner value has
    /// been dropped.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_rc::RcWithAlloc;
    /// use std::alloc::System;
    /// use ylong_stdx_common::{SafeClone, ContainerError};
    ///
    /// let val = 5;
    /// let rc = RcWithAlloc::new(val, System::default())?;
    /// let weak = RcWithAlloc::downgrade(&rc)?;
    ///
    /// let strong = weak.upgrade()?.ok_or(ContainerError::Unlikely)?;
    /// assert_eq!(RcWithAlloc::strong_count(&strong), 2);
    /// # Ok::<(), ContainerError>(())
    /// ```
    pub fn upgrade(&self) -> Result<Option<RcWithAlloc<V, A>>, ContainerError> {
        match self.shared_inner() {
            None => Ok(None),
            Some(inner) => {
                if inner.strong() == 0 {
                    return Ok(None);
                }
                inner.inc_strong();
                let ret = RcWithAlloc {
                    inner: self.shared_inner,
                    alloc: self.alloc.safe_clone()?,
                    phantom: Default::default(),
                };
                Ok(Some(ret))
            }
        }
    }
}

impl<V: ?Sized, A: Allocator + SafeClone> SafeClone for WeakWithAlloc<V, A> {
    fn safe_clone(&self) -> Result<Self, ContainerError> {
        // If valid, increase the weak count
        if let Some(inner) = self.shared_inner() {
            inner.inc_weak();
        }
        Ok(WeakWithAlloc {
            shared_inner: self.shared_inner,
            alloc: self.alloc.safe_clone()?,
        })
    }

    fn safe_clone_from(&mut self, other: &Self) -> Result<(), ContainerError> {
        *self = other.safe_clone()?;
        Ok(())
    }
}

// SAFETY:
// Because as for one instance, when it is dropped, its inner pointer might not be dropped yet, but
// it is safe because other strong references are sharing this pointer. As for *this* instance,
// mark the trait as #[may_dangle] to clarify to the compiler that we know what we're doing, and
// it's safe for lifetime and memory management.
unsafe impl<#[may_dangle] V: ?Sized, A: Allocator> Drop for WeakWithAlloc<V, A> {
    fn drop(&mut self) {
        if let Some(inner) = self.shared_inner() {
            inner.dec_weak();
            if inner.weak() == 0 {
                // SAFETY:
                // Valid because self.shared_inner is a valid pointer
                unsafe {
                    self.alloc.deallocate(
                        self.shared_inner.cast(),
                        Layout::for_value_raw(self.shared_inner.as_ptr()),
                    );
                }
            }
        }
    }
}

impl<V: Debug + ?Sized, A: Allocator + Debug> Debug for WeakWithAlloc<V, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.shared_inner() {
            None => f.write_fmt(format_args!(
                "WeakWithAlloc: {{Value: None, Allocator: {:?}}}",
                self.alloc
            )),
            Some(inner) => f.write_fmt(format_args!(
                "WeakWithAlloc: {{Value: {:?}, Allocator: {:?}}}",
                inner.value(),
                self.alloc
            )),
        }
    }
}
