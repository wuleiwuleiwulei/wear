use crate::vec_deque::FixedCapVecDeque;
use core::alloc::Allocator;
use core::fmt;

/// An owning iterator over the elements of a `FixedCapVecDeque`.
///
/// This `struct` is created by the `into_iter` method of `FixedCapVecDeque`.
pub struct IntoIter<T, A: Allocator> {
    inner: FixedCapVecDeque<T, A>,
}

impl<T, A: Allocator> IntoIter<T, A> {
    pub(crate) fn new(inner: FixedCapVecDeque<T, A>) -> Self {
        IntoIter { inner }
    }
}

impl<T: fmt::Debug, A: Allocator> fmt::Debug for IntoIter<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.inner).finish()
    }
}

impl<T, A: Allocator> Iterator for IntoIter<T, A> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.inner.len();
        (len, Some(len))
    }
}

impl<T, A: Allocator> DoubleEndedIterator for IntoIter<T, A> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.inner.pop_back()
    }
}

impl<T, A: Allocator> ExactSizeIterator for IntoIter<T, A> {
    #[inline]
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    /// UT test cases for intoiter `intoiter`.
    ///
    /// # Brief
    /// 1. create deque
    /// 2. get into iter of deque, then do iteration
    /// 3. check operation success
    #[test]
    fn ut_vec_deque_intoiter() {
        let mut q = create_queue_from_arr_back([3, 4], 10);
        let _ = q.push_front(2);
        let _ = q.push_front(1);
        let _ = q.push_front(0);
        let it = q.into_iter();
        assert_eq!(it.len(), 5);
        assert_eq!(it.size_hint(), (5, Some(5)));
        assert!(!it.is_empty());
        for (i, e) in it.enumerate() {
            assert_eq!(e, i as i32);
        }
    }

    /// UT test cases for intoiter `DoubleEndedIterator`.
    ///
    /// # Brief
    /// 1. create deque
    /// 2. get into iter of deque, then do iteration back
    /// 3. check operation success
    #[test]
    fn ut_vec_deque_intoiter_back() {
        let mut q = create_queue_from_arr_back([3, 4], 10);
        let _ = q.push_front(2);
        let _ = q.push_front(1);
        let _ = q.push_front(0);
        let mut it = q.into_iter();
        assert_eq!(it.next_back().unwrap(), 4);
        assert_eq!(it.next_back().unwrap(), 3);
        assert_eq!(it.next_back().unwrap(), 2);
        assert_eq!(it.next_back().unwrap(), 1);
        assert_eq!(it.next_back().unwrap(), 0);
        assert!(it.next_back().is_none());
        assert_eq!(it.len(), 0);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert!(it.is_empty());
    }

    /// UT test cases for intoiter `debug`.
    ///
    /// # Brief
    /// 1. create deque
    /// 2. get debug info of intoiter
    /// 3. check operation success
    #[test]
    fn ut_vec_deque_intoiter_debug() {
        let mut q = create_queue_from_arr_back([3, 4], 10);
        let _ = q.push_front(2);
        let _ = q.push_front(1);
        let _ = q.push_front(0);
        let it = q.into_iter();
        assert_eq!(format!("{it:?}"), "IntoIter([0, 1, 2, 3, 4])");
    }
}
