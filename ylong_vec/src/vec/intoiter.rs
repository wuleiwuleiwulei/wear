use crate::FixedCapVec;
use core::alloc::Allocator;
use core::alloc::Layout;
use core::fmt::{Debug, Formatter};
use core::mem::ManuallyDrop;
use core::mem::SizedTypeProperties;
use core::ptr::Unique;
use core::{fmt, slice};
use core::{mem, ptr};

/// An iterator that moves out of a FixedCapVec.
/// This struct is created by the into_iter method on vec (provided by the IntoIterator trait).
///
/// # Example
///
/// ```
/// #![feature(allocator_api)]
/// use std::alloc::Global;
/// use ylong_vec::FixedCapVec;
///
/// let v = FixedCapVec::<u8, Global>::new(1, Global);
/// let iter = v.into_iter();
/// ```
pub struct IntoIter<T, A: Allocator> {
    pub(crate) buf: Unique<T>,
    pub(crate) cap: usize,
    // alloc is from FixedCapVec;it can not be move or copy,
    // so use ManuallyDrop to wrap it
    pub(crate) alloc: ManuallyDrop<A>,
    pub(crate) ptr: *const T,
    pub(crate) end: *const T,
}

impl<T, A: Allocator> Iterator for IntoIter<T, A> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.ptr == self.end {
            return None;
        }

        if T::IS_ZST {
            // `ptr` has to stay still to remain aligned, so reduce the length by 1 by
            // reducing the `end`.
            self.end = self.end.wrapping_byte_sub(1);

            // SAFETY: for ZST, mem will never be actually visited.
            return Some(unsafe { mem::zeroed() });
        }

        let old = self.ptr;
        // SAFETY: mem is between begin and end so it's safe.
        self.ptr = unsafe { self.ptr.add(1) };

        // SAFETY: mem is between begin and end so it's safe.
        Some(unsafe { ptr::read(old) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = if T::IS_ZST {
            self.end.addr().wrapping_sub(self.ptr.addr())
        } else {
            // SAFETY: mem is between begin and end so it's safe.
            unsafe { self.end.sub_ptr(self.ptr) }
        };
        (exact, Some(exact))
    }
}

impl<T, A: Allocator> ExactSizeIterator for IntoIter<T, A> {
    fn is_empty(&self) -> bool {
        self.ptr == self.end
    }
}

impl<T, A: Allocator> IntoIter<T, A> {
    /// Returns items haven't traversed before in this iterator as a slice.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    ///
    /// let mut v = FixedCapVec::<u8, Global>::new(3, Global).unwrap();
    /// v.push(1);
    /// v.push(2);
    /// v.push(3);
    /// let iter = v.into_iter();
    ///
    /// assert_eq!(iter.as_slice(), &[1, 2, 3]);
    /// ```
    pub fn as_slice(&self) -> &[T] {
        // SAFETY: mem is allocated before so it's safe.
        unsafe { slice::from_raw_parts(self.ptr, self.len()) }
    }

    /// Returns items haven't traversed before in this iterator as a mutable slice.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    ///
    /// let mut v = FixedCapVec::<u8, Global>::new(3, Global).unwrap();
    /// v.push(1);
    /// v.push(2);
    /// v.push(3);
    /// let mut iter = v.into_iter();
    /// assert_eq!(iter.as_slice(), &[1, 2, 3]);
    ///
    /// iter.as_mut_slice()[2] = 4;
    /// assert_eq!(iter.next().unwrap(), 1);
    /// assert_eq!(iter.next().unwrap(), 2);
    /// assert_eq!(iter.next().unwrap(), 4);
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: mem is allocated before so it's safe.
        unsafe { &mut *self.as_raw_mut_slice() }
    }

    fn as_raw_mut_slice(&mut self) -> *mut [T] {
        ptr::slice_from_raw_parts_mut(self.ptr as *mut T, self.len())
    }
}

impl<T: Debug, A: Allocator> Debug for IntoIter<T, A> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.as_slice()).finish()
    }
}

// SAFETY: Because as for one instance, when it is dropped, its inner pointer might not be dropped yet, but
// it is safe because other strong references are sharing this pointer. As for *this* instance,
// mark the trait as #[may_dangle] to clarify to the compiler that we know what we're doing, and
// it's safe for lifetime and memory management.
unsafe impl<#[may_dangle] T, A: Allocator> Drop for IntoIter<T, A> {
    fn drop(&mut self) {
        struct DropGuard<'a, T, A: Allocator>(&'a mut IntoIter<T, A>);

        impl<T, A: Allocator> Drop for DropGuard<'_, T, A> {
            fn drop(&mut self) {
                // SAFETY: mem and layout have been check when allocated so it's safe.
                // Take alloc for drop it.
                unsafe {
                    let _alloc = ManuallyDrop::take(&mut self.0.alloc);
                    if T::IS_ZST || self.0.cap == 0 {
                        return;
                    }

                    let align = mem::align_of::<T>();
                    let size = mem::size_of::<T>().unchecked_mul(self.0.cap);
                    let layout = Layout::from_size_align_unchecked(size, align);
                    let ptr = self.0.buf.cast().into();

                    self.0.alloc.deallocate(ptr, layout)
                }
            }
        }

        let guard = DropGuard(self);
        // SAFETY: elements have ownership so it's safe to drop.
        unsafe {
            // drop remaining elements
            ptr::drop_in_place(guard.0.as_raw_mut_slice());
            // handles deallocation
        }
    }
}

impl<T, A: Allocator> IntoIterator for FixedCapVec<T, A> {
    type Item = T;
    type IntoIter = IntoIter<T, A>;

    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: elements are valid in FixedCapVec.
        unsafe {
            let mut me = ManuallyDrop::new(self);
            let alloc = ManuallyDrop::new(ptr::read(&me.buf.alloc));

            let begin = me.as_mut_ptr();
            let end = if T::IS_ZST {
                begin.wrapping_byte_add(me.len())
            } else {
                begin.add(me.len()) as *const T
            };

            IntoIter {
                buf: me.buf.ptr,
                cap: me.buf.capacity,
                alloc,
                ptr: begin,
                end,
            }
        }
    }
}

impl<'a, T, A: Allocator> IntoIterator for &'a FixedCapVec<T, A> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, A: Allocator> IntoIterator for &'a mut FixedCapVec<T, A> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    include!("../../tests/ut/ut_intoiter.rs");
}
