use crate::raw_buffer::AllocatedVecBuffer;
use crate::vec_deque::iter::{Iter, IterMut};
use crate::vec_deque::IntoIter;
use core::alloc::Allocator;
use core::ops::Range;
use core::ptr;
use core::{fmt, slice};
use ylong_stdx_common::{ContainerError, SafeClone};

/// A double-ended queue implemented with fixed capacity, and customizable allocator.
///
/// The "default" usage of this type as a queue is to use push_back to add to the queue,
/// and pop_front to remove from the queue. And iterating over VecDeque goes front to back.
///
///  # Examples
//
/// ```
/// #![feature(allocator_api)]
/// use std::alloc::Global;
/// use ylong_vec::FixedCapVecDeque;
///
/// let mut deque : FixedCapVecDeque::<u8, _> = FixedCapVecDeque::<u8, Global>::new(4, Global).unwrap();
///
/// assert!(deque.push_front(1).is_ok());
/// assert!(deque.push_front(2).is_ok());
///
/// assert_eq!(deque.len(), 2);
/// assert_eq!(deque.front(), Some(&2));
///
/// assert_eq!(deque.pop_front(), Some(2));
/// assert_eq!(deque.len(), 1);
///
/// assert!(deque.contains(&1));
///
/// for x in &deque {
///     println!("{x}");
/// }
/// ```
///
/// The capacity and allocator must be specified when queue is created. The capacity cannot be changed after being created.
/// When the number of elements in the container reaches the upper limit of capacity, or the Allocator cannot continue to allocate memory,
/// ContainerError is returned for the corresponding push and other operations.
///
/// Since deque is a ring buffer, its elements are not necessarily contiguous in memory.
///
pub struct FixedCapVecDeque<T, A: Allocator> {
    head: usize,
    len: usize,
    buf: AllocatedVecBuffer<T, A>,
}

impl<T, A: Allocator> FixedCapVecDeque<T, A> {
    /// Creates an empty deque, with fixed capacity and customized allocator.
    ///
    /// Note: For ZST, the capacity parameter is ignored. The container capacity is always usize::MAX.
    /// The max memory space allocation is isize::MAX, exceeding this limit `new` will fail.(including 32 and 16 bits)
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let deque : FixedCapVecDeque::<u8, _> = FixedCapVecDeque::<u8, Global>::new(4, Global).unwrap();
    /// ```
    pub fn new(capacity: usize, alloc: A) -> Result<Self, ContainerError> {
        let raw = AllocatedVecBuffer::allocate_in(capacity, alloc)?;

        Ok(Self {
            head: 0,
            buf: raw,
            len: 0,
        })
    }

    #[inline]
    fn ptr(&self) -> *mut T {
        self.buf.as_ptr()
    }

    /// Removes all values, clears the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque : FixedCapVecDeque::<u8, _> = FixedCapVecDeque::<u8, Global>::new(4, Global).unwrap();
    /// deque.push_back(1);
    /// deque.clear();
    /// assert!(deque.is_empty());
    /// ```
    pub fn clear(&mut self) {
        // SAFETY: mem are safely allocated and len is in range.
        let elems: *mut [T] = unsafe { slice::from_raw_parts_mut(self.ptr(), self.len) };

        self.len = 0;
        self.head = 0;
        // SAFETY: elems are safely allocated so it's safe to drop.
        unsafe {
            ptr::drop_in_place(elems);
        }
    }

    /// Provides a reference of the front element, or `None` if the deque is
    /// empty.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque : FixedCapVecDeque::<u8, _> = FixedCapVecDeque::<u8, Global>::new(4, Global).unwrap();
    /// assert_eq!(deque.front(), None);
    ///
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// assert_eq!(deque.front(), Some(&1));
    /// ```
    pub fn front(&self) -> Option<&T> {
        self.get(0)
    }

    /// Provides a mutable reference of the front element, or `None` if the
    /// deque is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque = FixedCapVecDeque::<u8, Global>::new(4, Global).unwrap();
    /// assert_eq!(deque.front_mut(), None);
    ///
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// match deque.front_mut() {
    ///     Some(x) => *x = 9,
    ///     None => (),
    /// }
    /// assert_eq!(deque.front(), Some(&9));
    /// ```
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    /// Provides a reference of the back element, or `None` if the deque is
    /// empty.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque = FixedCapVecDeque::<u8, Global>::new(4, Global).unwrap();
    /// assert_eq!(deque.back(), None);
    ///
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// assert_eq!(deque.back(), Some(&2));
    /// ```
    pub fn back(&self) -> Option<&T> {
        self.get(self.len.wrapping_sub(1))
    }

    /// Provides a mutable reference to the back element, or `None` if the
    /// deque is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque = FixedCapVecDeque::<u8, Global>::new(4, Global).unwrap();
    /// assert_eq!(deque.back(), None);
    ///
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// match deque.back_mut() {
    ///     Some(x) => *x = 9,
    ///     None => (),
    /// }
    /// assert_eq!(deque.back(), Some(&9));
    /// ```
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.get_mut(self.len.wrapping_sub(1))
    }

    #[inline]
    fn buffer_read(&mut self, off: usize) -> T {
        // SAFETY: mem are safely allocated and index is in range.
        unsafe { ptr::read(self.ptr().add(off)) }
    }

    #[inline]
    fn buffer_write(&mut self, off: usize, value: T) {
        // SAFETY: mem are safely allocated and index is in range.
        unsafe {
            ptr::write(self.ptr().add(off), value);
        }
    }

    /// Pushes an element to the front of the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque = FixedCapVecDeque::<u8, Global>::new(4, Global).unwrap();
    /// deque.push_front(1);
    /// deque.push_front(2);
    /// assert_eq!(deque.front(), Some(&2));
    /// ```
    pub fn push_front(&mut self, value: T) -> Result<(), ContainerError> {
        if self.is_full() {
            return Err(ContainerError::Full);
        }

        self.head = self.idx_sub_one(self.head);
        self.len += 1;

        self.buffer_write(self.head, value);
        Ok(())
    }

    /// Pushes an element to the back of the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque = FixedCapVecDeque::<u8, Global>::new(4, Global).unwrap();
    /// deque.push_back(1);
    /// deque.push_back(3);
    /// assert_eq!(deque.back(), Some(&3));
    /// ```
    pub fn push_back(&mut self, value: T) -> Result<(), ContainerError> {
        if self.is_full() {
            return Err(ContainerError::Full);
        }
        self.buffer_write(self.to_physical_idx(self.len), value);
        self.len += 1;
        Ok(())
    }

    /// Removes the front element and return it, or `None` if the deque is
    /// empty.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque = FixedCapVecDeque::<u8, Global>::new(4, Global).unwrap();
    /// deque.push_back(1);
    /// deque.push_back(2);
    ///
    /// assert_eq!(deque.pop_front(), Some(1));
    /// assert_eq!(deque.pop_front(), Some(2));
    /// assert_eq!(deque.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let old_head = self.head;
            self.head = self.to_physical_idx(1);
            self.len -= 1;
            Some(self.buffer_read(old_head))
        }
    }

    /// Removes the back element from the deque and return it, or `None` if
    /// it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque = FixedCapVecDeque::<u8, Global>::new(4, Global).unwrap();
    /// assert_eq!(deque.pop_back(), None);
    /// deque.push_back(1);
    /// deque.push_back(3);
    /// assert_eq!(deque.pop_back(), Some(3));
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            Some(self.buffer_read(self.to_physical_idx(self.len)))
        }
    }

    /// Returns true if the deque is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque = FixedCapVecDeque::<u8, Global>::new(4, Global).unwrap();
    /// assert!(deque.is_empty());
    /// deque.push_front(1);
    /// assert!(!deque.is_empty());
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns true if the buffer is at full capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque = FixedCapVecDeque::<u8, Global>::new(4, Global).unwrap();
    /// assert!(!deque.is_full());
    /// deque.push_front(1);
    /// deque.push_front(2);
    /// deque.push_front(3);
    /// deque.push_front(4);
    /// assert!(deque.is_full());
    pub fn is_full(&self) -> bool {
        self.len == self.buf.capacity()
    }

    /// Returns the max number of elements the deque can hold.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let deque = FixedCapVecDeque::<u8, Global>::new(10, Global).unwrap();
    /// assert_eq!(deque.capacity(), 10);
    /// ```
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    /// Returns the number of elements in the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque = FixedCapVecDeque::<u8, Global>::new(10, Global).unwrap();
    /// assert_eq!(deque.len(), 0);
    /// deque.push_back(1);
    /// assert_eq!(deque.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    fn buffer_range(&self, range: Range<usize>) -> *mut [T] {
        // SAFETY: mem are safely allocated and index is in range.
        unsafe {
            ptr::slice_from_raw_parts_mut(self.ptr().add(range.start), range.end - range.start)
        }
    }

    #[inline]
    fn as_slices(&self) -> (&[T], &[T]) {
        let (a, b) = self.as_mut_slices();
        (a, b)
    }

    #[inline]
    fn as_mut_slices(&self) -> (&mut [T], &mut [T]) {
        let len = self.len;
        let wrapped_start = self.to_physical_idx(0);
        let head_len = self.capacity() - wrapped_start;
        let (a_range, b_range) = if head_len >= len {
            (wrapped_start..wrapped_start + len, 0..0)
        } else {
            let tail_len = len - head_len;
            (wrapped_start..self.capacity(), 0..tail_len)
        };

        // SAFETY: mem are safely allocated and index is in range.
        unsafe {
            (
                &mut *self.buffer_range(a_range),
                &mut *self.buffer_range(b_range),
            )
        }
    }

    /// Returns `true` if the deque contains an element equal to the
    /// given value.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque = FixedCapVecDeque::<u8, Global>::new(10, Global).unwrap();
    ///
    /// deque.push_back(0);
    /// deque.push_back(1);
    ///
    /// assert_eq!(deque.contains(&1), true);
    /// assert_eq!(deque.contains(&10), false);
    /// ```
    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq<T>,
    {
        let (a, b) = self.as_slices();
        a.contains(value) || b.contains(value)
    }

    /// Returns a front-to-back iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque = FixedCapVecDeque::<i32, Global>::new(10, Global).unwrap();
    /// deque.push_back(0);
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// let mut cnt = 0;
    /// for e in deque.iter() {
    ///     assert_eq!(*e, cnt);
    ///     cnt += 1;
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        let (a, b) = self.as_slices();
        Iter::new(a.iter(), b.iter())
    }

    /// Returns a front-to-back mutable iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque = FixedCapVecDeque::<u8, Global>::new(10, Global).unwrap();
    /// deque.push_back(5);
    /// deque.push_back(3);
    /// deque.push_back(4);
    ///
    /// for e in deque.iter_mut() {
    ///     *e = *e - 2;
    /// }
    /// assert_eq!(deque.front(), Some(&3));
    /// ```
    pub fn iter_mut(&self) -> IterMut<'_, T> {
        let (a, b) = self.as_mut_slices();
        IterMut::new(a.iter_mut(), b.iter_mut())
    }

    #[inline]
    fn wrap_add(&self, idx: usize, addend: usize) -> usize {
        // For Sized Type, this wrapping_add can not be wrapped, because max capactity is isize::MAX
        wrap_index(idx.wrapping_add(addend), self.capacity())
    }

    #[inline]
    fn idx_sub_one(&self, idx: usize) -> usize {
        if idx == 0 {
            self.capacity() - 1
        } else {
            idx - 1
        }
    }

    #[inline]
    fn to_physical_idx(&self, idx: usize) -> usize {
        self.wrap_add(self.head, idx)
    }

    /// Provides a reference to the element at the given index.
    ///
    /// Element at index 0 is the front of the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque = FixedCapVecDeque::<u8, Global>::new(10, Global).unwrap();
    /// deque.push_back(3);
    /// deque.push_back(4);
    /// deque.push_back(5);
    /// deque.push_back(6);
    /// assert_eq!(deque.get(1), Some(&4));
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            let idx = self.to_physical_idx(index);
            // SAFETY: mem are safely allocated and index is in range.
            unsafe { Some(&*self.buf.as_ptr().add(idx)) }
        } else {
            None
        }
    }

    /// Provides a mutable reference to the element at the given index.
    ///
    /// Element at index 0 is the front of the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    ///
    /// let mut deque = FixedCapVecDeque::<u8, Global>::new(10, Global).unwrap();
    /// deque.push_back(3);
    /// deque.push_back(4);
    /// deque.push_back(5);
    /// deque.push_back(6);
    /// assert_eq!(deque.get(1), Some(&4));
    /// if let Some(elem) = deque.get_mut(1) {
    ///     *elem = 7;
    /// }
    /// assert_eq!(deque.get(1), Some(&7));
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            let idx = self.to_physical_idx(index);
            // SAFETY: mem are safely allocated and index is in range.
            unsafe { Some(&mut *self.buf.as_ptr().add(idx)) }
        } else {
            None
        }
    }

    fn allocator(&self) -> &A {
        self.buf.allocator()
    }
}

#[inline]
fn wrap_index(logical_index: usize, capacity: usize) -> usize {
    if logical_index >= capacity {
        logical_index - capacity
    } else {
        logical_index
    }
}

impl<T: SafeClone, A: Allocator + SafeClone> SafeClone for FixedCapVecDeque<T, A> {
    /// Safe clones self, and returns Err if failing in allocation or other operations.
    /// Both elements and allocator of deque will be cloned.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    /// use ylong_vec::SafeClone;
    ///
    /// let mut queue: FixedCapVecDeque<i32, _> = FixedCapVecDeque::new(10, Global).unwrap();
    /// queue.push_front(1);
    ///
    /// let clone = queue.safe_clone().unwrap();
    /// assert_eq!(clone.get(0), Some(&1));
    /// ```
    fn safe_clone(&self) -> Result<Self, ContainerError> {
        let mut q = FixedCapVecDeque::new(self.capacity(), self.allocator().safe_clone()?)?;
        for v in self.iter() {
            q.push_back(v.safe_clone()?)?;
        }
        Ok(q)
    }

    /// Safe clones self from an other instance, and returns Err if failing in allocation or other
    /// operations.
    /// This method will not clone allocator or reallocate deque memory. It only clone elements of deque.
    /// If the capacity of destination is less than source, this method will fail.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_vec::FixedCapVecDeque;
    /// use ylong_vec::SafeClone;
    ///
    /// let mut queue: FixedCapVecDeque<i32, _> = FixedCapVecDeque::new(10, Global).unwrap();
    /// queue.push_front(1);
    ///
    /// let mut clone = FixedCapVecDeque::new(10, Global).unwrap();
    /// clone.safe_clone_from(&queue);
    /// assert_eq!(clone.get(0), Some(&1));
    /// ```
    fn safe_clone_from(&mut self, source: &Self) -> Result<(), ContainerError> {
        if self.capacity() < source.len() {
            return Err(ContainerError::CapacityOverflow);
        }
        self.clear();
        for v in source.iter() {
            self.push_back(v.safe_clone()?)?;
        }
        Ok(())
    }
}

impl<T: fmt::Debug, A: Allocator> fmt::Debug for FixedCapVecDeque<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T, A: Allocator> AsRef<FixedCapVecDeque<T, A>> for FixedCapVecDeque<T, A> {
    fn as_ref(&self) -> &FixedCapVecDeque<T, A> {
        self
    }
}

impl<T, A: Allocator> AsMut<FixedCapVecDeque<T, A>> for FixedCapVecDeque<T, A> {
    fn as_mut(&mut self) -> &mut FixedCapVecDeque<T, A> {
        self
    }
}

// SAFETY: Because as for one instance, when it is dropped, its inner pointer might not be dropped yet, but
// it is safe because other strong references are sharing this pointer. As for *this* instance,
// mark the trait as #[may_dangle] to clarify to the compiler that we know what we're doing, and
// it's safe for lifetime and memory management.
unsafe impl<#[may_dangle] T, A: Allocator> Drop for FixedCapVecDeque<T, A> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut [T]);

        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                // SAFETY: elements have ownership so it's safe to drop.
                unsafe {
                    ptr::drop_in_place(self.0);
                }
            }
        }

        let (front, back) = self.as_mut_slices();
        // SAFETY: elements have ownership so it's safe to drop.
        unsafe {
            let _back_dropguard = DropGuard(back);
            ptr::drop_in_place(front);
        }
        // RawVec handles deallocation
    }
}

impl<T, A: Allocator> IntoIterator for FixedCapVecDeque<T, A> {
    type Item = T;
    type IntoIter = IntoIter<T, A>;

    /// Consumes the deque into a front-to-back iterator yielding elements by
    /// value.
    fn into_iter(self) -> IntoIter<T, A> {
        IntoIter::new(self)
    }
}

impl<'a, T, A: Allocator> IntoIterator for &'a FixedCapVecDeque<T, A> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T, A: Allocator> IntoIterator for &'a mut FixedCapVecDeque<T, A> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    include!("../../tests/ut/ut_deque.rs");
}
