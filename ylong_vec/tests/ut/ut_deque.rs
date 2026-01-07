use super::*;
use std::alloc::{Global, Layout};
use std::ptr::NonNull;

fn create_queue_from_arr_front<T, const N: usize>(
    arr: [T; N],
    cap: usize,
) -> FixedCapVecDeque<T, Global> {
    let mut v = FixedCapVecDeque::new(cap, Global).unwrap();
    for e in arr {
        let _ = v.push_front(e);
    }
    v
}

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

/// UT test cases for deque `new`.
///
/// # Brief
/// 1. create deque
/// 2. check new success
#[test]
fn ut_vec_deque_new() {
    let q = FixedCapVecDeque::<i32, Global>::new(10, Global);
    assert!(q.is_ok());
}

/// UT test cases for deque `push_back`.
///
/// # Brief
/// 1. create deque
/// 2. push back element to deque
/// 3. check operation success
#[test]
fn ut_vec_deque_push_back() {
    let mut q = FixedCapVecDeque::new(5, Global).unwrap();
    assert!(q.push_back(1).is_ok());
    assert!(q.push_back(2).is_ok());
    assert!(q.push_back(3).is_ok());
    assert!(q.push_back(4).is_ok());
    assert!(q.push_back(5).is_ok());
    assert!(q.push_back(6).is_err());
}

/// UT test cases for deque `push_front`.
///
/// # Brief
/// 1. create deque
/// 2. push front element to deque
/// 3. check operation success
#[test]
fn ut_vec_deque_push_front() {
    let mut q = FixedCapVecDeque::new(5, Global).unwrap();
    assert!(q.push_front(1).is_ok());
    assert!(q.push_front(2).is_ok());
    assert!(q.push_front(3).is_ok());
    assert!(q.push_front(4).is_ok());
    assert!(q.push_front(5).is_ok());
    assert!(q.push_front(6).is_err());
}

/// UT test cases for deque `pop_front`.
///
/// # Brief
/// 1. create deque
/// 2. pop front element from deque
/// 3. check operation success
#[test]
fn ut_vec_deque_pop_front() {
    let mut q = FixedCapVecDeque::new(10, Global).unwrap();
    assert!(q.push_front(1).is_ok());
    assert!(q.push_front(2).is_ok());
    assert!(q.push_front(3).is_ok());

    assert_eq!(q.pop_front().unwrap(), 3);
    assert_eq!(q.pop_front().unwrap(), 2);
    assert_eq!(q.pop_front().unwrap(), 1);
    assert!(q.pop_front().is_none());
}

/// UT test cases for deque `pop_back`.
///
/// # Brief
/// 1. create deque
/// 2. pop back element from deque
/// 3. check operation success
#[test]
fn ut_vec_deque_pop_back() {
    let mut q = FixedCapVecDeque::new(10, Global).unwrap();
    assert!(q.push_front(1).is_ok());
    assert!(q.push_front(2).is_ok());
    assert!(q.push_front(3).is_ok());

    assert_eq!(q.pop_back().unwrap(), 1);
    assert_eq!(q.pop_back().unwrap(), 2);
    assert_eq!(q.pop_back().unwrap(), 3);
    assert!(q.pop_back().is_none());
}

/// UT test cases for deque `clear`.
///
/// # Brief
/// 1. create deque
/// 2. clear deque
/// 3. check operation success
#[test]
fn ut_vec_deque_clear() {
    let mut q = create_queue_from_arr_front([1, 2, 3, 4, 5], 10);
    q.clear();
    assert!(q.pop_front().is_none());
}

/// UT test cases for deque `front`.
///
/// # Brief
/// 1. create deque
/// 2. get front element from deque
/// 3. check operation success
#[test]
fn ut_vec_deque_front() {
    let mut q = create_queue_from_arr_back([1, 2, 3, 4, 5], 10);
    assert_eq!(*q.front().unwrap(), 1);
    let _ = q.push_front(123);
    assert_eq!(*q.front().unwrap(), 123);
    q.clear();
    assert!(q.front().is_none());
    let _ = q.push_front(456);
    assert_eq!(*q.front().unwrap(), 456);
    let f = q.front_mut().unwrap();
    *f = 789;
    assert_eq!(*q.front().unwrap(), 789);
}

/// UT test cases for deque `back`.
///
/// # Brief
/// 1. create deque
/// 2. get back element from deque
/// 3. check operation success
#[test]
fn ut_vec_deque_back() {
    let mut q = create_queue_from_arr_front([5, 4, 3, 2, 1], 10);
    assert_eq!(*q.back().unwrap(), 5);
    let _ = q.push_back(123);
    assert_eq!(*q.back().unwrap(), 123);
    q.clear();
    assert!(q.back().is_none());
    let _ = q.push_back(456);
    assert_eq!(*q.back().unwrap(), 456);
    let f = q.back_mut().unwrap();
    *f = 789;
    assert_eq!(*q.back().unwrap(), 789);
}

/// UT test cases for deque `empty` and `full`.
///
/// # Brief
/// 1. create deque
/// 2. get empty or full from deque
/// 3. check operation success
#[test]
fn ut_vec_deque_empty_full() {
    let mut q = create_queue_from_arr_back([1, 2, 3, 4, 5], 5);
    assert!(q.is_full());
    q.clear();
    assert!(q.is_empty());
}

/// UT test cases for deque `capacity`.
///
/// # Brief
/// 1. create deque
/// 2. get capacity from deque
/// 3. check operation success
#[test]
fn ut_vec_deque_capacity() {
    let q = create_queue_from_arr_back([1, 2, 3, 4, 5], 10);
    assert_eq!(q.capacity(), 10);

    let q = create_queue_from_arr_back([()], 10);
    assert_eq!(q.capacity(), usize::MAX);
}

/// UT test cases for deque `len`.
///
/// # Brief
/// 1. create deque
/// 2. get len from deque
/// 3. check operation success
#[test]
fn ut_vec_deque_len() {
    let q = create_queue_from_arr_back([1, 2, 3, 4, 5], 10);
    assert_eq!(q.len(), 5);

    let q = create_queue_from_arr_back([(), (), ()], 10);
    assert_eq!(q.len(), 3);
}

#[derive(PartialEq, Debug)]
struct TestTemp {
    value: i32,
}

impl TestTemp {
    fn new(x: i32) -> Self {
        Self { value: x }
    }
}

/// UT test cases for deque `contains`.
///
/// # Brief
/// 1. create deque
/// 2. check contanis element
#[test]
fn ut_vec_deque_contains() {
    let q = create_queue_from_arr_back([1, 2, 3, 4, 5], 10);
    assert!(q.contains(&1));
    assert!(!q.contains(&6));
    assert!(!q.contains(&0));

    let q = create_queue_from_arr_back([TestTemp::new(1), TestTemp::new(2), TestTemp::new(3)], 10);
    assert!(q.contains(&TestTemp::new(1)));
    assert!(q.contains(&TestTemp::new(2)));
    assert!(q.contains(&TestTemp::new(3)));
    assert!(!q.contains(&TestTemp::new(0)));
    assert!(!q.contains(&TestTemp::new(4)));
}

/// UT test cases for deque `get`.
///
/// # Brief
/// 1. create deque
/// 2. check get index element
#[test]
fn ut_vec_deque_get() {
    let mut q = create_queue_from_arr_back([1, 2, 3, 4, 5], 10);
    assert_eq!(*q.get(0).unwrap(), 1);
    assert_eq!(*q.get(1).unwrap(), 2);
    assert_eq!(*q.get(2).unwrap(), 3);
    assert_eq!(*q.get(4).unwrap(), 5);
    assert!(q.get(5).is_none());

    let _ = q.push_front(123);
    assert_eq!(*q.get(0).unwrap(), 123);
    assert_eq!(*q.get(1).unwrap(), 1);
    assert_eq!(*q.get(5).unwrap(), 5);
    assert!(q.get(6).is_none());

    let x = q.get_mut(3);
    *x.unwrap() = 456;
    assert_eq!(*q.get(3).unwrap(), 456);
    let x = q.get_mut(6);
    assert!(x.is_none());
}

/// UT test cases for deque `safe_clone`.
///
/// # Brief
/// 1. create deque
/// 2. safe_clone deque
/// 3. check operation success
#[test]
fn ut_vec_deque_safe_clone() {
    let q = create_queue_from_arr_back([1, 2, 3, 4, 5], 10);

    let q2 = q.safe_clone().unwrap();
    assert_eq!(*q2.get(0).unwrap(), 1);
    assert_eq!(*q2.get(1).unwrap(), 2);
    assert_eq!(*q2.get(2).unwrap(), 3);
    assert_eq!(*q2.get(4).unwrap(), 5);
    assert!(q2.get(5).is_none());
}

#[derive(Clone)]
struct ConditionFailAllocator {
    cnt: *mut i32,
}

unsafe impl Allocator for ConditionFailAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, core::alloc::AllocError> {
        unsafe {
            if *self.cnt == 1 {
                Global.allocate(layout)
            } else {
                Err(core::alloc::AllocError)
            }
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe { Global.deallocate(ptr, layout) }
    }
}

/// UT test cases for deque `safe_clone`.
///
/// # Brief
/// 1. create deque
/// 2. safe_clone deque
/// 3. check operation failed
#[test]
#[allow(unused_assignments)]
fn ut_vec_deque_safe_clone_failed() {
    let mut x = 1;
    let ptr = &mut x as *mut i32;
    let alloc = ConditionFailAllocator { cnt: ptr };

    let mut q = FixedCapVecDeque::new(10, alloc).unwrap();
    let _ = q.push_front(1);
    x = 0;
    let r = q.safe_clone();
    assert!(r.is_err());

    let alloc = ConditionFailAllocator { cnt: ptr };
    let mut q = FixedCapVecDeque::new(10, alloc).unwrap();
    let _ = q.push_front(());
    x = 0;
    let r = q.safe_clone();
    assert!(r.is_ok());

    let alloc = ConditionFailAllocator { cnt: ptr };
    let q: FixedCapVecDeque<(), _> = FixedCapVecDeque::new(0, alloc).unwrap();
    x = 0;
    let r = q.safe_clone();
    assert!(r.is_ok());
}

/// UT test cases for deque `safe_clone_from`.
///
/// # Brief
/// 1. create deque
/// 2. safe_clone_from deque
/// 3. check operation success
#[test]
fn ut_vec_deque_safe_clone_from() {
    let q = create_queue_from_arr_back([1, 2, 3, 4, 5], 10);

    let mut q2 = FixedCapVecDeque::new(10, Global).unwrap();
    let _ = q2.safe_clone_from(&q);
    assert_eq!(*q2.get(0).unwrap(), 1);
    assert_eq!(*q2.get(1).unwrap(), 2);
    assert_eq!(*q2.get(2).unwrap(), 3);
    assert_eq!(*q2.get(4).unwrap(), 5);
    assert!(q2.get(5).is_none());

    let mut q3 = FixedCapVecDeque::new(5, Global).unwrap();
    let r = q3.safe_clone_from(&q);
    assert!(r.is_ok());

    let mut q4 = FixedCapVecDeque::new(4, Global).unwrap();
    let r = q4.safe_clone_from(&q);
    assert!(r.is_err());
}

/// UT test cases for deque `as_mut`.
///
/// # Brief
/// 1. create deque
/// 2. get as_mut of deque
/// 3. check operation success
#[test]
fn ut_vec_deque_as_mut_ref() {
    let mut q = create_queue_from_arr_back([1, 2, 3, 4, 5], 10);

    let r1: &FixedCapVecDeque<i32, Global> = q.as_ref();
    assert_eq!(*r1.front().unwrap(), 1);
    let r2: &mut FixedCapVecDeque<i32, Global> = q.as_mut();

    let _ = r2.push_front(123);
    assert_eq!(*r2.front().unwrap(), 123)
}

/// UT test cases for deque `debug`.
///
/// # Brief
/// 1. create deque
/// 2. show debug info of deque
/// 3. check operation success
#[test]
fn ut_vec_deque_debug() {
    let mut q = create_queue_from_arr_back([TestTemp::new(3), TestTemp::new(4)], 10);
    let _ = q.push_front(TestTemp::new(2));
    let _ = q.push_front(TestTemp::new(1));
    let _ = q.push_front(TestTemp::new(0));
    assert_eq!(
        format!("{q:?}"),
        "[TestTemp { value: 0 }, TestTemp { value: 1 }, TestTemp { value: 2 }, TestTemp { value: 3 }, TestTemp { value: 4 }]"
    );
}
