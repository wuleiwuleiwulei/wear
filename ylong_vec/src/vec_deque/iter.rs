use core::mem;
use core::{fmt, slice};

/// An iterator over the elements of a `FixedCapVecDeque`.
///
/// This `struct` is created by the `iter` method of `FixedCapVecDeque`.
pub struct Iter<'a, T: 'a> {
    i1: slice::Iter<'a, T>,
    i2: slice::Iter<'a, T>,
}

impl<'a, T> Iter<'a, T> {
    pub(crate) fn new(i1: slice::Iter<'a, T>, i2: slice::Iter<'a, T>) -> Self {
        Self { i1, i2 }
    }
}

impl<T: fmt::Debug> fmt::Debug for Iter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Iter")
            .field(&self.i1.as_slice())
            .field(&self.i2.as_slice())
            .finish()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        match self.i1.next() {
            Some(val) => Some(val),
            None => {
                mem::swap(&mut self.i1, &mut self.i2);
                self.i1.next()
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        match self.i2.next_back() {
            Some(val) => Some(val),
            None => {
                mem::swap(&mut self.i1, &mut self.i2);
                self.i2.next_back()
            }
        }
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {
    fn len(&self) -> usize {
        self.i1.len() + self.i2.len()
    }

    fn is_empty(&self) -> bool {
        self.i1.is_empty() && self.i2.is_empty()
    }
}

/// An mutable iterator over the elements of a `FixedCapVecDeque`.
///
/// This `struct` is created by the `iter` method of `FixedCapVecDeque`.
pub struct IterMut<'a, T: 'a> {
    i1: slice::IterMut<'a, T>,
    i2: slice::IterMut<'a, T>,
}

impl<'a, T> IterMut<'a, T> {
    pub(super) fn new(i1: slice::IterMut<'a, T>, i2: slice::IterMut<'a, T>) -> Self {
        Self { i1, i2 }
    }
}

impl<T: fmt::Debug> fmt::Debug for IterMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IterMut")
            .field(&self.i1.as_slice())
            .field(&self.i2.as_slice())
            .finish()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<&'a mut T> {
        match self.i1.next() {
            Some(val) => Some(val),
            None => {
                mem::swap(&mut self.i1, &mut self.i2);
                self.i1.next()
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut T> {
        match self.i2.next_back() {
            Some(val) => Some(val),
            None => {
                mem::swap(&mut self.i1, &mut self.i2);
                self.i2.next_back()
            }
        }
    }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {
    fn len(&self) -> usize {
        self.i1.len() + self.i2.len()
    }

    fn is_empty(&self) -> bool {
        self.i1.is_empty() && self.i2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::FixedCapVecDeque;
    use std::alloc::Global;

    fn create_queue_from_arr_back<T, const N: usize>(
        arr: [T; N],
        cap: usize,
    ) -> FixedCapVecDeque<T, Global> {
        let mut v = FixedCapVecDeque::new(cap, Global).unwrap();
        for e in arr {
            let _ = v.push_back(e);
        }
        v
    }

    /// UT test cases for iter `Iterator`.
    ///
    /// # Brief
    /// 1. create deque
    /// 2. get iter of deque, then do iteration
    /// 3. check operation success
    #[test]
    fn ut_vec_deque_iter() {
        let mut q = create_queue_from_arr_back([3, 4], 10);
        let _ = q.push_front(2);
        let _ = q.push_front(1);
        let _ = q.push_front(0);
        let it = q.iter();
        assert_eq!(it.len(), 5);
        assert_eq!(it.size_hint(), (5, Some(5)));
        assert!(!it.is_empty());
        for (i, e) in it.enumerate() {
            assert_eq!(*e, i as i32);
        }
    }

    /// UT test cases for iter `Iterator`.
    ///
    /// # Brief
    /// 1. create deque
    /// 2. get iter of reference deque, then do iteration
    /// 3. check operation success
    #[test]
    #[allow(clippy::explicit_counter_loop)]
    fn ut_vec_deque_iter_2() {
        let mut q = create_queue_from_arr_back([3, 4], 10);
        let _ = q.push_front(2);
        let _ = q.push_front(1);
        let _ = q.push_front(0);
        let mut cnt = 0;
        for e in &q {
            assert_eq!(*e, cnt);
            cnt += 1;
        }
    }

    /// UT test cases for iter_mut `Iterator`.
    ///
    /// # Brief
    /// 1. create deque
    /// 2. get mut iter of deque, then do iteration
    /// 3. check operation success
    #[test]
    fn ut_vec_deque_iter_mut() {
        let mut q = create_queue_from_arr_back([3, 4], 10);
        let _ = q.push_front(2);
        let _ = q.push_front(1);
        let _ = q.push_front(0);
        let it = q.iter_mut();
        assert_eq!(it.len(), 5);
        assert_eq!(it.size_hint(), (5, Some(5)));
        assert!(!it.is_empty());
        for e in it {
            *e = 123;
        }
        assert_eq!(*q.get(0).unwrap(), 123);
    }

    /// UT test cases for iter_mut `Iterator`.
    ///
    /// # Brief
    /// 1. create deque
    /// 2. get mut iter of reference mut deque, then do iteration
    /// 3. check operation success
    #[test]
    fn ut_vec_deque_iter_mut_2() {
        let mut q = create_queue_from_arr_back([3, 4], 10);
        let _ = q.push_front(2);
        let _ = q.push_front(1);
        let _ = q.push_front(0);
        for e in &mut q {
            *e = 123;
        }
        assert_eq!(*q.get(0).unwrap(), 123);
    }

    /// UT test cases for iter `DoubleEndedIterator`.
    ///
    /// # Brief
    /// 1. create deque
    /// 2. get iter of deque, then do iteration back
    /// 3. check operation success
    #[test]
    fn ut_vec_deque_iter_back() {
        let mut q = create_queue_from_arr_back([3, 4], 10);
        let _ = q.push_front(2);
        let _ = q.push_front(1);
        let _ = q.push_front(0);
        let mut it = q.iter();
        assert_eq!(*it.next_back().unwrap(), 4);
        assert_eq!(*it.next_back().unwrap(), 3);
        assert_eq!(*it.next_back().unwrap(), 2);
        assert_eq!(*it.next_back().unwrap(), 1);
        assert_eq!(*it.next_back().unwrap(), 0);
        assert_eq!(it.len(), 0);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert!(it.is_empty());
    }

    /// UT test cases for iter_mut `DoubleEndedIterator`.
    ///
    /// # Brief
    /// 1. create deque
    /// 2. get mut iter of deque, then do iteration back
    /// 3. check operation success
    #[test]
    fn ut_vec_deque_iter_mut_back() {
        let mut q = create_queue_from_arr_back([3, 4], 10);
        let _ = q.push_front(2);
        let _ = q.push_front(1);
        let _ = q.push_front(0);
        let mut it = q.iter_mut();
        *it.next_back().unwrap() = 40;
        *it.next_back().unwrap() = 41;
        *it.next_back().unwrap() = 42;
        *it.next_back().unwrap() = 43;
        *it.next_back().unwrap() = 44;
        assert_eq!(*q.get(4).unwrap(), 40);
    }

    /// UT test cases for iter `DoubleEndedIterator`.
    ///
    /// # Brief
    /// 1. create deque
    /// 2. get debug info of iter
    /// 3. check operation success
    #[test]
    fn ut_vec_deque_iter_debug() {
        let mut q = create_queue_from_arr_back([3, 4], 10);
        let _ = q.push_front(2);
        let _ = q.push_front(1);
        let _ = q.push_front(0);
        let it = q.iter();
        assert_eq!(format!("{it:?}"), "Iter([0, 1, 2], [3, 4])");
        let it = q.iter_mut();
        assert_eq!(format!("{it:?}"), "IterMut([0, 1, 2], [3, 4])");
    }
}
