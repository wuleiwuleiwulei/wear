use crate::weak::WeakWithAlloc;
use alloc::alloc::{Allocator, Layout};
use alloc::rc::{RcBox, RcInnerPtr};
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use core::ops::Deref;
use core::ptr::{drop_in_place, NonNull};
use ylong_box::AllocatorBox;
use ylong_stdx_common::{ContainerError, SafeClone};

#[cfg(test)]
mod tests;

/// RcWithAlloc that provides functions the same as std's Rc, but won't panic when OOM.
///
/// Designed the same as Rc in std, the inherent methods of RcWithAlloc are all associated
/// functions to avoid conflicting names of the inner type `V`.
pub struct RcWithAlloc<V: ?Sized, A: Allocator> {
    pub(crate) inner: NonNull<RcBox<V>>,
    pub(crate) alloc: A,
    // Can't use PhantomData<V> because we clarify V can be unsized / DST (dynamically-sized type)
    pub(crate) phantom: PhantomData<RcBox<V>>,
}

impl<V, A: Allocator> RcWithAlloc<V, A> {
    /// Initializes a new instance of RcWithAlloc. Users need to clarify the allocator. The
    /// allocator must implement Allocator trait in the Rust standard library. Return error if
    /// anything fails when creating the instance, e.g. failure to allocate the memory specified.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_rc::RcWithAlloc;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// let val = 5;
    /// let rc = RcWithAlloc::new(val, System::default())?;
    /// # Ok::<(), ContainerError>(())
    /// ```
    pub fn new(value: V, allocator: A) -> Result<RcWithAlloc<V, A>, ContainerError> {
        let rcbox = RcBox::new(value);
        let allocated = AllocatorBox::new(rcbox, allocator)?;
        let (inner, alloc) = AllocatorBox::into_raw_with_allocator(allocated);
        Ok(RcWithAlloc {
            // Won't be null because Box should return a valid ptr
            inner: NonNull::new(inner).ok_or(ContainerError::Unlikely)?,
            alloc,
            phantom: Default::default(),
        })
    }

    fn into_inner_with_alloc_unchecked(self) -> (V, A) {
        // Bitwise copy, and keep RcBox to prevent Weak ptr to panic
        //
        // SAFETY:
        // Unsafe because use pointer to read from mems. It is actually safe because the inner
        // value is of type V / A.
        let alloc = unsafe { core::ptr::read(&self.alloc as *const A) };
        // SAFETY:
        // Safe as the previous line states.
        let val = unsafe { core::ptr::read(self.inner().value() as *const V) };
        // Delete implicit strong and weak relation
        self.inner().dec_strong();
        self.inner().dec_weak();
        // Drop RcBox if needed
        self.drop_inner();
        // Avoid dropping value and alloc.
        core::mem::forget(self);
        (val, alloc)
    }

    /// Consumes the RcWithAlloc provided and returns the inner value, if RcWithAlloc only has one
    /// strong reference. If not, this function will return None, but still drop the RcWithAlloc
    /// passed in.
    ///
    /// This function doesn't return the allocator. If it is needed, please use
    /// RcWithAlloc::into_inner_with_alloc.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_rc::RcWithAlloc;
    /// use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// let val = 5;
    /// let rc = RcWithAlloc::new(val, System::default())?;
    /// assert_eq!(RcWithAlloc::into_inner(rc).ok_or(ContainerError::Unlikely)?, 5);
    /// # Ok::<(), ContainerError>(())
    /// ```
    pub fn into_inner(rc: RcWithAlloc<V, A>) -> Option<V> {
        if RcWithAlloc::strong_count(&rc) > 1 {
            return None;
        }
        Some(rc.into_inner_with_alloc_unchecked().0)
    }

    /// Consumes the RcWithAlloc provided and returns the inner value and the allocator, if
    /// RcWithAlloc only has one strong reference. If not, this function will return Ok(None), but
    /// still drop the RcWithAlloc passed in.
    ///
    /// We can't take allocator out separately or put it on the heap. Drop trait will restricts the
    /// completeness of an instance, and an allocator can't allocate itself. So to return it, the
    /// allocator has to be cloned. If error happens when cloning the allocator, an error
    /// will be returned.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_rc::RcWithAlloc;
    /// use std::alloc::System;
    /// use ylong_stdx_common::{SafeClone, ContainerError};
    ///
    /// let val = 5;
    /// let rc = RcWithAlloc::new(val, System::default())?;
    /// let rc2 = rc.safe_clone()?;
    /// assert!(RcWithAlloc::into_inner_with_alloc(rc2).is_none());
    /// let (returned, _alloc) = RcWithAlloc::into_inner_with_alloc(rc).ok_or(ContainerError::Unlikely)?;
    /// assert_eq!(returned, 5);
    /// # Ok::<(), ContainerError>(())
    /// ```
    pub fn into_inner_with_alloc(rc: RcWithAlloc<V, A>) -> Option<(V, A)> {
        if RcWithAlloc::strong_count(&rc) > 1 {
            return None;
        }
        Some(rc.into_inner_with_alloc_unchecked())
    }

    /// Try to unwrap the RcWithAlloc provided and returns the inner value and the allocator, if
    /// RcWithAlloc only has one strong reference. If not, this function will return Err(Self) to
    /// return back the instance.
    ///
    /// We can't take allocator out separately or put it on the heap. Drop trait will restricts the
    /// completeness of an instance, and an allocator can't allocate itself. So to return it, the
    /// allocator has to be cloned. If error happens when cloning the allocator, an error
    /// will be returned.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_rc::RcWithAlloc;
    /// use std::alloc::System;
    /// use ylong_stdx_common::{SafeClone, ContainerError};
    ///
    /// let val = 5;
    /// let rc = RcWithAlloc::new(val, System::default())?;
    /// let rc2 = rc.safe_clone()?;
    /// assert_eq!(
    ///     RcWithAlloc::try_unwrap_with_alloc(rc2).err().unwrap(),
    ///     RcWithAlloc::new(5, System::default())?
    /// );
    /// let (returned, _alloc) = RcWithAlloc::try_unwrap_with_alloc(rc)
    ///     .map_err(|_| ContainerError::Unlikely)?;
    /// assert_eq!(returned, 5);
    /// # Ok::<(), ContainerError>(())
    /// ```
    pub fn try_unwrap_with_alloc(rc: RcWithAlloc<V, A>) -> Result<(V, A), Self> {
        if RcWithAlloc::strong_count(&rc) > 1 {
            return Err(rc);
        }
        Ok(rc.into_inner_with_alloc_unchecked())
    }
}

impl<V: ?Sized, A: Allocator> RcWithAlloc<V, A> {
    #[inline]
    fn inner(&self) -> &RcBox<V> {
        // SAFETY:
        // Safe because all restrictions of nonnull are met:
        // 1. Pointer is initialized and aligned.
        // 2. Lifetime is bound by self
        unsafe { self.inner.as_ref() }
    }

    #[inline]
    fn inner_mut(&mut self) -> &mut RcBox<V> {
        // SAFETY:
        // Safe because all restrictions of nonnull are met:
        // 1. Pointer is initialized and aligned.
        // 2. Lifetime is bound by self
        unsafe { self.inner.as_mut() }
    }

    fn drop_inner(&self) {
        if self.inner().weak() == 0 {
            // SAFETY:
            // Valid because self.shared_inner is a valid pointer
            unsafe {
                self.alloc.deallocate(
                    self.inner.cast(),
                    Layout::for_value_raw(self.inner.as_ptr()),
                );
            }
        }
    }

    /// Gets a reference of the allocator in a RcWithAlloc instance.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_rc::RcWithAlloc;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// let val = 5;
    /// let rc = RcWithAlloc::new(val, System::default())?;
    /// let alloc = RcWithAlloc::allocator(&rc);
    /// # Ok::<(), ContainerError>(())
    /// ```
    #[inline]
    pub fn allocator(rc: &RcWithAlloc<V, A>) -> &A {
        &rc.alloc
    }

    /// Gets a mutable reference of the value stored in a RcWithAlloc instance. Returns
    /// Some(&mut V) if there's no other strong reference and no weak reference. Otherwise,
    /// returns None.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_rc::RcWithAlloc;
    /// use std::alloc::System;
    /// use ylong_stdx_common::{SafeClone, ContainerError};
    ///
    /// let val = 5;
    /// let mut rc = RcWithAlloc::new(val, System::default())?;
    /// assert_eq!(*rc, 5);
    /// *RcWithAlloc::get_mut(&mut rc).ok_or(ContainerError::Unlikely)? += 1;
    /// assert_eq!(*rc, 6);
    ///
    /// let _rc2 = rc.safe_clone()?;
    /// assert!(RcWithAlloc::get_mut(&mut rc).is_none());
    /// # Ok::<(), ContainerError>(())
    /// ```
    pub fn get_mut(rc: &mut RcWithAlloc<V, A>) -> Option<&mut V> {
        // If other sharing exists, it is unsafe to return a mutable reference
        if RcWithAlloc::strong_count(rc) > 1 || RcWithAlloc::weak_count(rc) > 0 {
            return None;
        }
        Some(rc.inner_mut().value_mut())
    }

    /// Gets the number of strong references of the RcWithAlloc instance.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_rc::RcWithAlloc;
    /// use std::alloc::System;
    /// use ylong_stdx_common::{SafeClone, ContainerError};
    ///
    /// let val = 5;
    /// let rc = RcWithAlloc::new(val, System::default())?;
    /// assert_eq!(RcWithAlloc::strong_count(&rc), 1);
    ///
    /// let _rc2 = rc.safe_clone()?;
    /// assert_eq!(RcWithAlloc::strong_count(&rc), 2);
    /// # Ok::<(), ContainerError>(())
    /// ```
    #[inline]
    pub fn strong_count(rc: &RcWithAlloc<V, A>) -> usize {
        rc.inner().strong()
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
    /// assert_eq!(RcWithAlloc::weak_count(&rc), 0);
    ///
    /// let _weak = RcWithAlloc::downgrade(&rc)?;
    /// assert_eq!(RcWithAlloc::weak_count(&rc), 1);
    /// # Ok::<(), ContainerError>(())
    /// ```
    #[inline]
    pub fn weak_count(rc: &RcWithAlloc<V, A>) -> usize {
        // Decrease by 1 because of the self-pointing relations
        rc.inner().weak() - 1
    }
}

impl<V: ?Sized, A: Allocator + SafeClone> RcWithAlloc<V, A> {
    /// Creates a weak reference to the RcWithAlloc instance.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_rc::RcWithAlloc;
    /// use std::alloc::System;
    /// use ylong_stdx_common::{SafeClone, ContainerError};
    ///
    /// let val = 5;
    /// let rc = RcWithAlloc::new(val, System::default())?;
    /// assert_eq!(RcWithAlloc::weak_count(&rc), 0);
    ///
    /// let _weak = RcWithAlloc::downgrade(&rc)?;
    /// assert_eq!(RcWithAlloc::weak_count(&rc), 1);
    /// # Ok::<(), ContainerError>(())
    /// ```
    pub fn downgrade(rc: &RcWithAlloc<V, A>) -> Result<WeakWithAlloc<V, A>, ContainerError> {
        rc.inner().inc_weak();
        Ok(WeakWithAlloc::from_inner(rc.inner, rc.alloc.safe_clone()?))
    }
}

impl<V: SafeClone, A: Allocator + SafeClone> RcWithAlloc<V, A> {
    /// Gets a mutable reference of the value stored in a RcWithAlloc instance. Returns
    /// Some(&mut V) if there's no other strong reference and no weak reference.
    ///
    /// If there're multiple strong references, the inner value and allocator will be cloned to a
    /// new allocation before the precondition (there should be no other strong reference and no
    /// weak reference) is met.
    ///
    /// If there's only one strong reference, but multiple weak references, those refereneces will
    /// be disassociated with the strong references. No value clone will happen.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_rc::RcWithAlloc;
    /// use std::alloc::System;
    /// use ylong_stdx_common::{SafeClone, ContainerError};
    ///
    /// let val = 5;
    /// let mut rc = RcWithAlloc::new(val, System::default())?;
    /// let rc2 = rc.safe_clone()?;
    /// assert_eq!(*rc, 5);
    /// assert_eq!(*rc2, 5);
    /// *RcWithAlloc::make_mut(&mut rc)? += 1;
    /// assert_eq!(*rc, 6);
    /// assert_eq!(*rc2, 5);
    ///
    /// let weak = RcWithAlloc::downgrade(&rc)?;
    /// *RcWithAlloc::make_mut(&mut rc)? += 1;
    /// assert_eq!(*rc, 7);
    /// assert!(weak.upgrade()?.is_none());
    /// # Ok::<(), ContainerError>(())
    /// ```
    pub fn make_mut(rc: &mut RcWithAlloc<V, A>) -> Result<&mut V, ContainerError> {
        if RcWithAlloc::strong_count(rc) > 1 {
            // If there're more strong count (multiple Rc structs), clone this one,
            // leave other weak / strong combinations alone
            let alloc = rc.alloc.safe_clone()?;
            let val = rc.inner().value().safe_clone()?;
            let new_rc = RcWithAlloc::new(val, alloc)?;
            *rc = new_rc;
        } else if RcWithAlloc::weak_count(rc) > 0 {
            // If there're only one strong count, but other Weaks, disassociate them, as said in
            // `get_mut`: if other sharing exists, it is unsafe to return a mutable reference.
            // Because Weaks don't own the value, just take it away.
            // Make a new RcBox. To avoid copy the allocator, here we don't use Box to do the
            // allocation. We manually repeat the procedure.
            //
            // SAFETY:
            // Safe because V is Sized.
            let layout = unsafe { Layout::for_value_raw(rc.inner.as_ptr()) };
            let mut mem: NonNull<RcBox<V>> = rc
                .alloc
                .allocate(layout)
                .map_err(|_| ContainerError::AllocFailure(layout))?
                .cast();
            // SAFETY:
            // Safe because all restrictions of nonnull are met:
            // 1. Pointer is aligned, and here we're initializing it.
            // 2. Lifetime is bound by mem, and it is valid now
            unsafe {
                mem.as_ref().strong_ref().set(1);
                mem.as_ref().weak_ref().set(1);
                // Create a bitwise copy
                let inner = core::ptr::read(rc.inner().value() as *const V);
                core::ptr::write(mem.as_mut().value_mut() as *mut V, inner);
            }
            // Correct count info
            rc.inner().dec_strong();
            // Remove the self-pointing relations
            rc.inner().dec_weak();
            // Replacing and dropping the original NonNull is safe. It won't call destructor of
            // its inner pointer
            rc.inner = mem;
        }
        Ok(rc.inner_mut().value_mut())
    }
}

impl<V: ?Sized, A: Allocator + SafeClone> SafeClone for RcWithAlloc<V, A> {
    fn safe_clone(&self) -> Result<Self, ContainerError> {
        self.inner().inc_strong();
        Ok(RcWithAlloc {
            inner: self.inner,
            alloc: self.alloc.safe_clone()?,
            phantom: Default::default(),
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
unsafe impl<#[may_dangle] V: ?Sized, A: Allocator> Drop for RcWithAlloc<V, A> {
    fn drop(&mut self) {
        self.inner().dec_strong();
        if self.inner().strong() == 0 {
            // Call destructor on the inner value
            //
            // SAFETY:
            // Safe because the pointer is valid and aligned.
            unsafe {
                drop_in_place(self.inner_mut().value_mut());
            }
            // Else, the value should be safely kept

            self.inner().dec_weak();
            if self.inner().weak() == 0 {
                // Deallocate RcBox allocated by Box methods. We can't call destructor of Box
                // directly, because the inner value and the outer RcBox might be deallocated
                // in a different time.
                //
                // SAFETY:
                // Valid because self.inner is a valid pointer
                unsafe {
                    self.alloc
                        .deallocate(self.inner.cast(), Layout::for_value(self.inner.as_ref()));
                }
            }
        }
    }
}

impl<V: Debug + ?Sized, A: Allocator + Debug> Debug for RcWithAlloc<V, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "RcWithAlloc: {{Value: {:?}, Allocator: {:?}}}",
            self.inner().value(),
            self.alloc
        ))
    }
}

impl<V: ?Sized, A: Allocator> Deref for RcWithAlloc<V, A> {
    type Target = V;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner().value()
    }
}

impl<V: ?Sized, A: Allocator> AsRef<V> for RcWithAlloc<V, A> {
    #[inline]
    fn as_ref(&self) -> &V {
        self.inner().value()
    }
}

impl<V: Eq + ?Sized, A: Allocator> Eq for RcWithAlloc<V, A> {}

impl<V: PartialEq + ?Sized, A: Allocator> PartialEq for RcWithAlloc<V, A> {
    #[inline]
    default fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

// Add specialization for Eq: eq represents a reflexive relationship, and if two
// Rc points to the same location, they should be thought as equal.
impl<V: Eq + ?Sized, A: Allocator> PartialEq for RcWithAlloc<V, A> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.as_ptr() == other.inner.as_ptr() || **self == **other
    }
}
