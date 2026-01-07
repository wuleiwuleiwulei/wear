use crate::raw_buffer::AllocatedVecBuffer;
use core::alloc::Allocator;
use core::ops::Deref;
use core::ops::DerefMut;
use core::ptr;
use core::slice::SlicePattern;
use core::{fmt, slice};
use ylong_stdx_common::ContainerError;
use ylong_stdx_common::SafeClone;

/// A vector with fixed capacity, and customizable allocator.
///
///  # Examples
///
/// ```
/// #![feature(allocator_api)]
/// use std::alloc::Global;
/// use ylong_vec::FixedCapVec;
///
/// let mut vec : FixedCapVec::<u8, _> = FixedCapVec::<u8, Global>::new(4, Global).unwrap();
///
/// assert!(vec.push(1).is_ok());
/// assert!(vec.push(2).is_ok());
///
/// assert_eq!(vec.len(), 2);
/// assert_eq!(vec[0], 1);
///
/// assert_eq!(vec.pop(), Some(2));
/// assert_eq!(vec.len(), 1);
///
/// vec[0] = 7;
/// assert_eq!(vec[0], 7);
///
/// assert!(vec.extend_from_slice(&[1, 2, 3]).is_ok());
///
/// for x in &vec {
///     println!("{x}");
/// }
/// assert_eq!(vec.as_ref(), [7, 1, 2, 3]);
///
/// ```
///
/// The capacity and allocator must be specified when vec is created. The capacity cannot be changed after being created.
/// When the number of elements in the container reaches the upper limit of capacity, or the Allocator cannot continue to allocate memory,
/// ContainerError is returned for the corresponding push and other operations.
///
/// #Indexing
/// You can use index to access values in FixedCapVec, but note that an out-of-range index can cause a panic!
///
/// ```should_panic
/// #![feature(allocator_api)]
/// use std::alloc::Global;
/// use ylong_vec::FixedCapVec;
///
/// let mut vec : FixedCapVec::<u8, _> = FixedCapVec::<u8, Global>::new(2, Global).unwrap();
/// vec.push(1);
/// vec.push(2);
/// println!("{}", vec[1]); // success
/// println!("{}", vec[6]); // it will panic
///
/// ```
/// Use get and get_mut if you want to check whether the index is in the FixedCapVec.
///
/// #Slicing
/// A FixedCapVec can be mutable. On the other hand, slices are read-only objects. To get a slice, use &. Example:
///
/// ```
/// #![feature(allocator_api)]
/// use std::alloc::Global;
/// use ylong_vec::FixedCapVec;
///
/// fn read_slice(slice: &[usize]) {
///     // ...
/// }
///
/// let v = FixedCapVec::<usize, Global>::new(2, Global).unwrap();
/// read_slice(&v);
///
/// let u: &[usize] = &v;
/// let u: &[_] = &v;
/// ```
///
pub struct FixedCapVec<T, A: Allocator> {
    pub(crate) buf: AllocatedVecBuffer<T, A>,
    pub(crate) len: usize,
}

impl<T, A: Allocator> FixedCapVec<T, A> {
    /// Constructs a new `FixedCapVec<T, A>` with specified capacity and Allocator.
    /// FixedCapVec allocates the memory corresponding to capacity.
    /// The capacity cannot be changed after being specified.
    ///
    /// Note: For ZST, the capacity parameter is ignored. The container capacity is always usize::MAX.
    /// The max memory space allocation is isize::MAX, exceeding this limit `new` will fail.(including 32 and 16 bits)
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    ///
    /// let mut vec: FixedCapVec<i32, _> = FixedCapVec::new(1, Global).unwrap();
    /// ```
    pub fn new(capacity: usize, alloc: A) -> Result<Self, ContainerError> {
        let raw = AllocatedVecBuffer::allocate_in(capacity, alloc)?;

        Ok(Self { buf: raw, len: 0 })
    }

    /// Clears the vector, removing all values.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    ///
    /// let mut vec: FixedCapVec<i32, _> = FixedCapVec::new(1, Global).unwrap();
    /// vec.push(1).is_ok();
    ///
    /// vec.clear();
    /// assert!(vec.is_empty());
    /// ```
    pub fn clear(&mut self) {
        let elems: *mut [T] = self.as_mut();

        self.len = 0;
        // SAFETY: elements are safe mem to drop.
        unsafe {
            ptr::drop_in_place(elems);
        }
    }

    /// Appends an element to the back of a collection.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    ///
    /// let mut vec: FixedCapVec<i32, _> = FixedCapVec::new(1, Global).unwrap();
    /// assert!(vec.push(1).is_ok());
    /// ```
    pub fn push(&mut self, v: T) -> Result<(), ContainerError> {
        if self.len == self.buf.capacity() {
            return Err(ContainerError::CapacityOverflow);
        }

        let ptr = self.buf.as_mut_ptr();
        // SAFETY: mem are safely allocated and len is in range.
        unsafe {
            ptr.add(self.len).write(v);
        }
        self.len += 1;
        Ok(())
    }

    /// Removes the last element from a vector and returns it, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    ///
    /// let mut vec: FixedCapVec<i32, _> = FixedCapVec::new(1, Global).unwrap();
    /// vec.push(1).is_ok();
    /// assert_eq!(vec.pop(), Some(1));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let ptr = self.buf.as_mut_ptr();
        // SAFETY: mem are safely allocated and len is in range.
        unsafe { Some(ptr.add(self.len).read()) }
    }

    /// Removes and returns the element at position index within the vector,
    /// shifting all elements after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    ///
    /// let mut vec: FixedCapVec<i32, _> = FixedCapVec::new(1, Global).unwrap();
    /// vec.push(1).is_ok();
    /// assert_eq!(vec.remove(0).unwrap(), 1);
    /// ```
    pub fn remove(&mut self, index: usize) -> Result<T, ContainerError> {
        if index >= self.len {
            return Err(ContainerError::IndexInvalid);
        }

        let value: T;
        let ptr = self.buf.as_mut_ptr();
        // SAFETY: mem are safely allocated and index is in range.
        unsafe {
            let index_ptr = ptr.add(index);
            value = index_ptr.read();
            ptr::copy(index_ptr.add(1), index_ptr, self.len - index - 1);
        }
        self.len -= 1;

        Ok(value)
    }

    /// Returns true if the vector contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    ///
    /// let mut vec: FixedCapVec<i32, _> = FixedCapVec::new(0, Global).unwrap();
    /// assert!(vec.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns true if the vector is full.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    ///
    /// let mut vec: FixedCapVec<i32, _> = FixedCapVec::new(0, Global).unwrap();
    /// assert!(vec.is_full());
    /// ```
    pub fn is_full(&self) -> bool {
        self.len == self.buf.capacity()
    }

    fn remain_capacity(&self) -> usize {
        self.buf.capacity - self.len
    }

    /// Extracts a mutable slice of the entire vector.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    ///
    /// let mut vec: FixedCapVec<i32, _> = FixedCapVec::new(10, Global).unwrap();
    /// let _slice:&[i32] = vec.as_mut_slice();
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.deref_mut()
    }

    /// Returns the number of elements in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    ///
    /// let mut vec: FixedCapVec<i32, _> = FixedCapVec::new(10, Global).unwrap();
    ///
    /// vec.push(1).is_ok();
    /// assert_eq!(vec.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the total number of elements the vector can hold.
    ///
    /// Note: For ZST, capacity is always usize::MAX.
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    ///
    /// let mut vec: FixedCapVec<i32, _> = FixedCapVec::new(10, Global).unwrap();
    ///
    /// assert_eq!(vec.capacity(), 10);
    /// ```
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    /// Shortens the vector, keeping the first `len` elements and dropping
    /// the rest.
    ///
    /// If `len` is greater or equal to the vector's current length, this has
    /// no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    ///
    /// let mut vec: FixedCapVec<i32, _> = FixedCapVec::new(10, Global).unwrap();
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    /// vec.push(4);
    /// vec.push(5);
    ///
    /// assert_eq!(vec.len(), 5);
    ///
    /// vec.truncate(2);
    ///
    /// assert_eq!(vec.len(), 2);
    /// assert_eq!(vec.as_ref(), [1, 2]);
    /// ```
    pub fn truncate(&mut self, len: usize) {
        if len >= self.len {
            return;
        }
        let remaining_len = self.len - len;
        // SAFETY: mem are safely allocated and index is in range.
        unsafe {
            let s = ptr::slice_from_raw_parts_mut(self.as_mut_ptr().add(len), remaining_len);
            self.len = len;
            ptr::drop_in_place(s);
        }
    }
}

// SAFETY: Because as for one instance, when it is dropped, its inner pointer might not be dropped yet, but
// it is safe because other strong references are sharing this pointer. As for *this* instance,
// mark the trait as #[may_dangle] to clarify to the compiler that we know what we're doing, and
// it's safe for lifetime and memory management.
unsafe impl<#[may_dangle] T, A: Allocator> Drop for FixedCapVec<T, A> {
    fn drop(&mut self) {
        // SAFETY: elements have ownership so it's safe to drop.
        unsafe {
            // use drop for [T] if T has a drop trait
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), self.len))
        }
        // then AllocatedVecBuffer does drop, handles deallocation
    }
}

impl<T: SafeClone, A: Allocator> FixedCapVec<T, A> {
    /// Safe Clone and appends all elements in a slice to the vec.
    /// Note: If the length of the slice exceeds the capacity of the vec,
    /// it does nothing and ContainerError is returned.
    ///
    /// # Panics
    /// Panics if the slice `other` does `safe_clone` paniced.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    ///
    /// let mut vec: FixedCapVec<i32, _> = FixedCapVec::new(10, Global).unwrap();
    ///
    /// vec.extend_from_slice(&[1, 2, 3, 4]);
    /// assert_eq!(vec.as_ref(), [1, 2, 3, 4]);
    /// ```
    pub fn extend_from_slice(&mut self, other: &[T]) -> Result<(), ContainerError> {
        if other.len() > self.remain_capacity() {
            return Err(ContainerError::CapacityOverflow);
        }
        for v in other {
            let e = v.safe_clone()?;
            let ptr = self.buf.as_mut_ptr();
            // SAFETY: mem are safely allocated and len is in range.
            unsafe {
                ptr.add(self.len).write(e);
            }
            self.len += 1;
        }
        Ok(())
    }
}

impl<T, A: Allocator> Deref for FixedCapVec<T, A> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        // SAFETY: mem are safely allocated and len is in range.
        unsafe { slice::from_raw_parts(self.buf.ptr.as_ptr(), self.len) }
    }
}

impl<T, A: Allocator> DerefMut for FixedCapVec<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        // SAFETY: mem are safely allocated and len is in range.
        unsafe { slice::from_raw_parts_mut(self.buf.ptr.as_ptr(), self.len) }
    }
}

impl<T, A: Allocator> AsRef<FixedCapVec<T, A>> for FixedCapVec<T, A> {
    fn as_ref(&self) -> &FixedCapVec<T, A> {
        self
    }
}

impl<T, A: Allocator> AsMut<FixedCapVec<T, A>> for FixedCapVec<T, A> {
    fn as_mut(&mut self) -> &mut FixedCapVec<T, A> {
        self
    }
}

impl<T, A: Allocator> AsRef<[T]> for FixedCapVec<T, A> {
    fn as_ref(&self) -> &[T] {
        self
    }
}

impl<T, A: Allocator> AsMut<[T]> for FixedCapVec<T, A> {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T: fmt::Debug, A: Allocator> fmt::Debug for FixedCapVec<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: SafeClone, A: Allocator + SafeClone> SafeClone for FixedCapVec<T, A> {
    /// Safe clone self, and return Err if failing in allocation or other operations.
    /// Both elements and allocator of vec will be cloned.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    /// use ylong_vec::SafeClone;
    ///
    /// let mut vec: FixedCapVec<i32, _> = FixedCapVec::new(10, Global).unwrap();
    /// vec.push(1);
    ///
    /// let clone = vec.safe_clone().unwrap();
    /// assert_eq!(clone.get(0), Some(&1));
    /// ```
    fn safe_clone(&self) -> Result<Self, ContainerError> {
        let alloc = self.buf.alloc.safe_clone()?;
        let mut vec: FixedCapVec<T, A> = FixedCapVec::new(self.capacity(), alloc)?;
        for i in self.as_slice() {
            vec.push(i.safe_clone()?)?;
        }
        Ok(vec)
    }

    /// Safe clone self from an other instance, and return Err if failing in allocation or other
    /// operations.
    /// This method will not clone allocator or reallocate vec memory. It only clone elements of vec.
    /// If the capacity of destination is less than source, this method will fail.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVec;
    /// use ylong_vec::SafeClone;
    ///
    /// let mut vec: FixedCapVec<i32, _> = FixedCapVec::new(10, Global).unwrap();
    /// vec.push(1);
    ///
    /// let mut clone = FixedCapVec::new(10, Global).unwrap();
    /// clone.safe_clone_from(&vec);
    /// assert_eq!(clone.get(0), Some(&1));
    /// ```
    fn safe_clone_from(&mut self, source: &Self) -> Result<(), ContainerError> {
        if self.capacity() < source.len() {
            return Err(ContainerError::CapacityOverflow);
        }
        // drop the extra parts.
        self.truncate(source.len());

        // due to truncate， self.len must <= other.len
        let len = self.len();
        let head = &source[..len];
        let tail = &source[len..];
        let head_len = head.len();
        for i in 0..head_len {
            self[i].safe_clone_from(&head[i])?;
        }

        for e in tail {
            self.push(e.safe_clone()?)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    include!("../../tests/ut/ut_vector.rs");
}
