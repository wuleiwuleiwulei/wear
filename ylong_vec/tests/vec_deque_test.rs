#![feature(allocator_api)]
#![feature(iter_advance_by)]
#![feature(exact_size_is_empty)]
#![feature(specialization)] // For specialization
#![allow(incomplete_features)] // Mask the unstable alarm of specialization.

use std::alloc::Global;
use std::fmt::Debug;
use std::num::NonZeroUsize;
use std::panic::catch_unwind;
use ylong_stdx_common::SafeClone;
use ylong_vec::ContainerError;
use ylong_vec::FixedCapVecDeque;

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

/// SDV test cases for deque `new`.
///
/// # Brief
/// 1. new, valid arg, succuess.
/// 2. new, invalid arg, fail.
#[test]
fn sdv_vec_deque_new() {
    let v1 = FixedCapVecDeque::<u8, Global>::new(10, Global);
    assert!(v1.is_ok());

    let v1 = FixedCapVecDeque::<(), Global>::new(usize::MAX, Global);
    assert!(v1.is_ok());

    // In 64 and 32 bit, memory allocation is not allowed for >= isize::MAX + 1
    let v1 = FixedCapVecDeque::<u8, Global>::new(isize::MAX as usize + 1, Global);
    assert!(v1.is_err());
}

/// SDV test cases for deque `new`.
///
/// # Brief
/// 1. new, valid arg, succuess.
/// 2. new, invalid arg, fail.
#[test]
#[cfg_attr(feature = "__asan", ignore)]
fn sdv_vec_deque_new_2() {
    let v1 = FixedCapVecDeque::<u8, Global>::new(isize::MAX as usize, Global);

    if usize::BITS == 64 {
        assert!(v1.is_err()); // For common 64 bit machine, memory for isize::MAX is surely OOM.
    } else {
        assert!(v1.is_ok()); // Note: This branch is for 32 bit, memory allocation for isize::MAX may success.Or maybe fail.
    }
}

/// SDV test cases for deque `push` and `pop`.
///
/// # Brief
/// 1. create a queue.
/// 2. do any push and pop actions.
/// 3. create a queue of 0 capacity.
/// 4. do push for zero capacity queue.
#[test]
fn sdv_vec_deque_test_simple() {
    let mut deque = FixedCapVecDeque::new(10, Global).unwrap();
    assert_eq!(deque.len(), 0);
    let _ = deque.push_front(17);
    let _ = deque.push_front(42);
    let _ = deque.push_back(137);
    assert_eq!(deque.as_ref().len(), 3);
    let _ = deque.push_back(137);
    assert_eq!(deque.len(), 4);
    assert_eq!(*deque.front().unwrap(), 42);
    assert_eq!(*deque.back_mut().unwrap(), 137);
    let mut i = deque.pop_front();
    assert_eq!(i, Some(42));
    i = deque.pop_back();
    assert_eq!(i, Some(137));
    i = deque.pop_back();
    assert_eq!(i, Some(137));
    i = deque.pop_back();
    assert_eq!(i, Some(17));
    assert_eq!(deque.len(), 0);
    let _ = deque.push_back(3);
    assert_eq!(deque.len(), 1);
    let _ = deque.push_front(2);
    assert_eq!(deque.len(), 2);
    let _ = deque.push_back(4);
    assert_eq!(deque.len(), 3);
    let _ = deque.push_front(1);
    assert_eq!(deque.len(), 4);
    assert_eq!(*deque.get(0).unwrap(), 1);
    assert_eq!(*deque.get(1).unwrap(), 2);
    assert_eq!(*deque.get(2).unwrap(), 3);
    assert_eq!(*deque.get(3).unwrap(), 4);

    let mut d: FixedCapVecDeque<i32, Global> = FixedCapVecDeque::new(0, Global).unwrap();
    assert!(d.push_front(1).is_err());
    assert!(d.push_back(1).is_err());
    assert!(d.pop_front().is_none());
    assert!(d.pop_back().is_none());
    assert!(d.front().is_none());
    assert!(d.back().is_none());
}

fn deque_test_parameterized<T: Clone + PartialEq + Debug>(a: T, b: T, c: T, d: T) {
    let mut deque = FixedCapVecDeque::new(10, Global).unwrap();
    assert_eq!(deque.len(), 0);
    let _ = deque.push_front(a.clone());
    let _ = deque.push_front(b.clone());
    let _ = deque.push_back(c.clone());
    assert_eq!(deque.len(), 3);
    let _ = deque.push_back(d.clone());
    assert_eq!(deque.len(), 4);
    assert_eq!((*deque.front().unwrap()).clone(), b.clone());
    assert_eq!((*deque.back().unwrap()).clone(), d.clone());
    assert_eq!(deque.pop_front().unwrap(), b.clone());
    assert_eq!(deque.pop_back().unwrap(), d.clone());
    assert_eq!(deque.pop_back().unwrap(), c.clone());
    assert_eq!(deque.pop_back().unwrap(), a.clone());
    assert_eq!(deque.len(), 0);
    let _ = deque.push_back(c.clone());
    assert_eq!(deque.len(), 1);
    let _ = deque.push_front(b.clone());
    assert_eq!(deque.len(), 2);
    let _ = deque.push_back(d.clone());
    assert_eq!(deque.len(), 3);
    let _ = deque.push_front(a.clone());
    assert_eq!(deque.len(), 4);
    assert_eq!(deque.get(0).unwrap().clone(), a.clone());
    assert_eq!(deque.get(1).unwrap().clone(), b.clone());
    assert_eq!(deque.get(2).unwrap().clone(), c.clone());
    assert_eq!(deque.get(3).unwrap().clone(), d.clone());
}

#[derive(Clone, PartialEq, Debug)]
enum Foo {
    One(i32),
    Two(i32, i32),
    Three(i32, i32, i32),
}

#[derive(Clone, PartialEq, Debug)]
enum Bar<T> {
    Onebar(T),
    Twobar(T, T),
    Threebar(T, T, T),
}

#[derive(Clone, PartialEq, Debug)]
struct RecCy {
    x: i32,
    y: i32,
    t: Foo,
}

/// SDV test cases for deque `push` and `pop`.
///
/// # Brief
/// 1. create a queue of `int` type.
/// 2. do any push and pop actions.
#[test]
fn sdv_vec_deque_test_param_int() {
    deque_test_parameterized::<i32>(5, 72, 64, 175);
}

/// SDV test cases for deque `push` and `pop`.
///
/// # Brief
/// 1. create a queue of `Foo` type.
/// 2. do any push and pop actions.
#[test]
fn sdv_vec_deque_test_param_foo() {
    deque_test_parameterized::<Foo>(
        Foo::One(1),
        Foo::Two(1, 2),
        Foo::Three(1, 2, 3),
        Foo::Two(17, 42),
    );
}

/// SDV test cases for deque `push` and `pop`.
///
/// # Brief
/// 1. create a queue of `Bar` type.
/// 2. do any push and pop actions.
#[test]
fn sdv_vec_deque_test_param_bar() {
    deque_test_parameterized::<Bar<i32>>(
        Bar::Onebar::<i32>(1),
        Bar::Twobar::<i32>(1, 2),
        Bar::Threebar::<i32>(1, 2, 3),
        Bar::Twobar::<i32>(17, 42),
    );
}

/// SDV test cases for deque `push` and `pop`.
///
/// # Brief
/// 1. create a queue of `RecCy` type.
/// 2. do any push and pop actions.
#[test]
fn sdv_vec_deque_test_param_reccy() {
    let reccy1 = RecCy {
        x: 1,
        y: 2,
        t: Foo::One(1),
    };
    let reccy2 = RecCy {
        x: 345,
        y: 2,
        t: Foo::Two(1, 2),
    };
    let reccy3 = RecCy {
        x: 1,
        y: 777,
        t: Foo::Three(1, 2, 3),
    };
    let reccy4 = RecCy {
        x: 19,
        y: 252,
        t: Foo::Two(17, 42),
    };
    deque_test_parameterized::<RecCy>(reccy1, reccy2, reccy3, reccy4);
}

/// SDV test cases for deque `push` and `pop`.
///
/// # Brief
/// 1. create a queue, which capacity is non power two.
/// 2. do any push and pop actions.
#[test]
fn sdv_vec_deque_test_with_capacity_non_power_two() {
    let mut deque = FixedCapVecDeque::new(3, Global).unwrap();
    let _ = deque.push_back(1);

    // X = None, | = lo
    // [|1, X, X]
    assert_eq!(deque.pop_front(), Some(1));
    // [X, |X, X]
    assert_eq!(deque.front(), None);

    // [X, |3, X]
    let _ = deque.push_back(3);
    // [X, |3, 6]
    let _ = deque.push_back(6);
    // [X, X, |6]
    assert_eq!(deque.pop_front(), Some(3));

    // Pushing the lo past half way point to trigger
    // the 'B' scenario for growth
    // [9, X, |6]
    let _ = deque.push_back(9);
    // [9, 12, |6]
    let _ = deque.push_back(12);
    let _ = deque.push_back(15);
}

/// SDV test cases for deque `iter`.
///
/// # Brief
/// 1. create a queue.
/// 2. take iter of queue.
/// 3. check element and size of iter.
#[test]
fn sdv_vec_deque_test_iter() {
    let mut d = FixedCapVecDeque::new(10, Global).unwrap();
    assert_eq!(d.iter().next(), None);
    assert_eq!(d.iter().size_hint(), (0, Some(0)));
    println!("{:?}", d.iter());
    println!("{:?}", d.iter_mut());

    for i in 0..5 {
        let _ = d.push_back(i);
    }
    {
        let b: &[_] = &[&0, &1, &2, &3, &4];
        assert_eq!(d.iter().collect::<Vec<_>>(), b);
    }

    for i in 6..9 {
        let _ = d.push_front(i);
    }
    {
        let b1: &[_] = &[&8, &7, &6, &0, &1, &2, &3, &4];
        assert_eq!(d.iter().collect::<Vec<_>>(), b1);
    }

    let mut iter = d.iter();
    let mut len = d.len();
    loop {
        match iter.next() {
            None => break,
            _ => {
                len -= 1;
                assert_eq!(iter.size_hint(), (len, Some(len)))
            }
        }
    }
}

/// SDV test cases for deque `iter`.
///
/// # Brief
/// 1. create a queue.
/// 2. take iter of queue.
/// 3. check rev element of iter.
#[test]
fn sdv_vec_deque_test_rev_iter() {
    let mut d = FixedCapVecDeque::new(10, Global).unwrap();
    assert_eq!(d.iter().next_back(), None);

    for i in 0..5 {
        let _ = d.push_back(i);
    }
    {
        let b: &[_] = &[&4, &3, &2, &1, &0];
        assert_eq!(d.iter().rev().collect::<Vec<_>>(), b);
    }

    for i in 6..9 {
        let _ = d.push_front(i);
    }
    let b: &[_] = &[&4, &3, &2, &1, &0, &6, &7, &8];
    assert_eq!(d.iter().rev().collect::<Vec<_>>(), b);
}

/// SDV test cases for deque `iter_mut`.
///
/// # Brief
/// 1. create a queue.
/// 2. take mut iter of queue.
/// 3. check element of iter.
#[test]
fn sdv_vec_deque_test_mut_rev_iter_wrap() {
    let mut d = FixedCapVecDeque::new(10, Global).unwrap();
    assert!(d.iter_mut().next_back().is_none());

    let _ = d.push_back(1);
    let _ = d.push_back(2);
    let _ = d.push_back(3);
    assert_eq!(d.pop_front(), Some(1));
    let _ = d.push_back(4);

    assert_eq!(
        d.iter_mut().rev().map(|x| *x).collect::<Vec<_>>(),
        vec![4, 3, 2]
    );
}

/// SDV test cases for deque `iter_mut`.
///
/// # Brief
/// 1. create a queue.
/// 2. take mut iter of queue.
/// 3. modify and check element of iter.
#[test]
fn sdv_vec_deque_test_mut_iter() {
    let mut deque = FixedCapVecDeque::new(10, Global).unwrap();
    assert!(deque.iter_mut().next().is_none());

    for i in 0..3 {
        let _ = deque.push_front(i);
    }

    for (i, e) in deque.iter_mut().enumerate() {
        assert_eq!(*e, 2 - i);
        *e = i;
    }

    {
        let mut it = deque.iter_mut();
        assert_eq!(*it.next().unwrap(), 0);
        assert_eq!(*it.next().unwrap(), 1);
        assert_eq!(*it.next().unwrap(), 2);
        assert!(it.next().is_none());
    }
}

/// SDV test cases for deque `iter_mut`.
///
/// # Brief
/// 1. create a queue.
/// 2. take rev mut iter of queue.
/// 3. modify and check element of rev iter.
#[test]
fn sdv_vec_deque_test_mut_rev_iter() {
    let mut d = FixedCapVecDeque::new(10, Global).unwrap();
    assert!(d.iter_mut().next_back().is_none());

    for i in 0..3 {
        let _ = d.push_front(i);
    }

    for (i, e) in d.iter_mut().rev().enumerate() {
        assert_eq!(*e, i);
        *e = i;
    }

    {
        let mut iter = d.iter_mut().rev();
        assert_eq!(*iter.next().unwrap(), 0);
        assert_eq!(*iter.next().unwrap(), 1);
        assert_eq!(*iter.next().unwrap(), 2);
        assert!(iter.next().is_none());
    }
}

/// SDV test cases for deque `into_iter`.
///
/// # Brief
/// 1. create a queue.
/// 2. take intoiter of queue.
/// 3. check empty iter.
/// 4. check simple iter
/// 5. check wrapped iter
/// 6. check partially used iter
/// 7. check advance_by iter
#[test]
fn sdv_vec_deque_test_into_iter() {
    // Empty iter
    {
        let d: FixedCapVecDeque<i32, std::alloc::Global> =
            FixedCapVecDeque::new(10, Global).unwrap();
        let mut iter = d.into_iter();
        println!("{iter:?}");
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    // simple iter
    {
        let mut deque = FixedCapVecDeque::new(10, Global).unwrap();
        for i in 0..5 {
            let _ = deque.push_back(i);
        }

        let b = vec![0, 1, 2, 3, 4];
        assert_eq!(deque.into_iter().collect::<Vec<_>>(), b);
    }

    // wrapped iter
    {
        let mut deque = FixedCapVecDeque::new(10, Global).unwrap();
        for i in 0..5 {
            let _ = deque.push_back(i);
        }
        for i in 6..9 {
            let _ = deque.push_front(i);
        }

        let b = vec![8, 7, 6, 0, 1, 2, 3, 4];
        assert_eq!(deque.into_iter().collect::<Vec<_>>(), b);
    }

    // partially used
    {
        let mut deque = FixedCapVecDeque::new(10, Global).unwrap();
        for i in 0..5 {
            let _ = deque.push_back(i);
        }
        for i in 6..9 {
            let _ = deque.push_front(i);
        }

        let mut iter = deque.into_iter();
        assert_eq!(iter.size_hint(), (8, Some(8)));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.size_hint(), (7, Some(7)));
        assert_eq!(iter.next_back(), Some(4));
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.size_hint(), (5, Some(5)));
    }

    // advance_by
    {
        let mut d = FixedCapVecDeque::new(10, Global).unwrap();
        for i in 0..=4 {
            let _ = d.push_back(i);
        }
        for i in 6..=8 {
            let _ = d.push_front(i);
        }

        let mut it = d.into_iter();
        assert_eq!(it.advance_by(1), Ok(()));
        assert_eq!(it.next(), Some(7));
        assert_eq!(it.advance_back_by(1), Ok(()));
        assert_eq!(it.next_back(), Some(3));

        let mut it = create_queue_from_arr_back([1, 2, 3, 4, 5], 10).into_iter();
        assert_eq!(it.advance_by(10), Err(NonZeroUsize::new(5).unwrap()));
        let mut it = create_queue_from_arr_back([1, 2, 3, 4, 5], 10).into_iter();
        assert_eq!(it.advance_back_by(10), Err(NonZeroUsize::new(5).unwrap()));
    }
}

/// SDV test cases for deque `safe_clone`.
///
/// # Brief
/// 1. create a queue.
/// 2. safe_clone of queue.
#[test]
fn sdv_vec_deque_test_safe_clone() {
    let mut d = FixedCapVecDeque::new(10, Global).unwrap();
    let _ = d.push_front(17);
    let _ = d.push_front(42);
    let _ = d.push_back(137);
    let _ = d.push_back(137);
    assert_eq!(d.len(), 4);
    let mut e = d.safe_clone().unwrap();
    assert_eq!(e.len(), 4);
    while !d.is_empty() {
        assert_eq!(d.pop_back(), e.pop_back());
    }
    assert_eq!(d.len(), 0);
    assert_eq!(e.len(), 0);
}

#[derive(Clone)]
struct Temp {
    _x: i32,
}

impl SafeClone for Temp {
    fn safe_clone(&self) -> Result<Self, ContainerError> {
        Err(ContainerError::LayoutError)
    }

    fn safe_clone_from(&mut self, _other: &Self) -> Result<(), ContainerError> {
        Err(ContainerError::LayoutError)
    }
}

/// SDV test cases for deque `safe_clone`.
///
/// # Brief
/// 1. create a queue.
/// 2. safe_clone of queue failed.
/// 3. check result
#[test]
fn sdv_vec_deque_test_safe_clone_element_fail() {
    let mut d = FixedCapVecDeque::new(10, Global).unwrap();
    let _ = d.as_mut().push_front(Temp { _x: 123 });
    let _ = d.push_front(Temp { _x: 456 });
    let d2 = d.safe_clone();
    assert!(d2.is_err());

    let mut d3 = FixedCapVecDeque::new(10, Global).unwrap();
    let r = d3.safe_clone_from(&d);
    assert!(r.is_err());
}

/// SDV test cases for deque `debug`.
///
/// # Brief
/// 1. create a queue.
/// 2. print debug of queue.
#[test]
fn sdv_vec_deque_test_show() {
    let ringbuf = create_queue_from_arr_back([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 10);
    assert_eq!(format!("{ringbuf:?}"), "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]");

    let ringbuf = create_queue_from_arr_back(["just", "one", "test", "more"], 10);
    assert_eq!(
        format!("{ringbuf:?}"),
        "[\"just\", \"one\", \"test\", \"more\"]"
    );
}

/// SDV test cases for deque `drop`.
///
/// # Brief
/// 1. create a queue.
/// 2. drop queue.
/// 3. check elements are dropped.
#[test]
fn sdv_vec_deque_test_drop() {
    static mut TEST_FLAGS: i32 = 0;
    struct Elem;
    impl Drop for Elem {
        fn drop(&mut self) {
            unsafe {
                TEST_FLAGS += 1;
            }
        }
    }

    let mut ring = FixedCapVecDeque::new(10, Global).unwrap();
    let _ = ring.push_back(Elem);
    let _ = ring.push_front(Elem);
    let _ = ring.push_back(Elem);
    let _ = ring.push_front(Elem);
    drop(ring);

    assert_eq!(unsafe { TEST_FLAGS }, 4);
}

/// SDV test cases for deque `drop`.
///
/// # Brief
/// 1. create a queue.
/// 2. pop element and drop queue.
/// 3. check elements are dropped.
#[test]
fn sdv_vec_deque_test_drop_with_pop() {
    static mut TEST_FLAGS: i32 = 0;
    struct Elem;
    impl Drop for Elem {
        fn drop(&mut self) {
            unsafe {
                TEST_FLAGS += 1;
            }
        }
    }

    let mut ring = FixedCapVecDeque::new(10, Global).unwrap();
    let _ = ring.push_back(Elem);
    let _ = ring.push_front(Elem);
    let _ = ring.push_back(Elem);
    let _ = ring.push_front(Elem);

    drop(ring.pop_back());
    drop(ring.pop_front());
    assert_eq!(unsafe { TEST_FLAGS }, 2);

    drop(ring);
    assert_eq!(unsafe { TEST_FLAGS }, 4);
}

/// SDV test cases for deque `clear`.
///
/// # Brief
/// 1. create a queue.
/// 2. clear queue.
/// 3. check elements are dropped.
#[test]
fn sdv_vec_deque_test_drop_clear() {
    static mut DROPS: i32 = 0;
    struct Elem;
    impl Drop for Elem {
        fn drop(&mut self) {
            unsafe {
                DROPS += 1;
            }
        }
    }

    let mut ring = FixedCapVecDeque::new(10, Global).unwrap();
    let _ = ring.push_back(Elem);
    let _ = ring.push_front(Elem);
    let _ = ring.push_back(Elem);
    let _ = ring.push_front(Elem);
    ring.clear();
    assert_eq!(unsafe { DROPS }, 4);

    drop(ring);
    assert_eq!(unsafe { DROPS }, 4);
}

/// SDV test cases for deque `clear`.
///
/// # Brief
/// 1. create a queue.
/// 2. drop queue.
/// 3. check elements are dropped.
#[test]
#[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
fn sdv_vec_deque_test_drop_panic() {
    static mut TEST_FLAGS: i32 = 0;

    struct DTemp(bool);

    impl Drop for DTemp {
        fn drop(&mut self) {
            unsafe {
                TEST_FLAGS += 1;
            }

            if self.0 {
                panic!("panic in `drop`");
            }
        }
    }

    let mut q = FixedCapVecDeque::new(10, Global).unwrap();
    let _ = q.push_back(DTemp(false));
    let _ = q.push_back(DTemp(false));
    let _ = q.push_back(DTemp(false));
    let _ = q.push_back(DTemp(false));
    let _ = q.push_back(DTemp(false));
    let _ = q.push_front(DTemp(false));
    let _ = q.push_front(DTemp(false));
    let _ = q.push_front(DTemp(true));

    catch_unwind(move || drop(q)).ok();

    assert_eq!(unsafe { TEST_FLAGS }, 8);
}

/// SDV test cases for deque `get`.
///
/// # Brief
/// 1. create a queue.
/// 2. get element from queue.
#[test]
fn sdv_vec_deque_test_get() {
    let mut deque = FixedCapVecDeque::new(10, Global).unwrap();
    let _ = deque.push_back(0);
    assert_eq!(deque.get(0), Some(&0));
    assert_eq!(deque.get(1), None);

    let _ = deque.push_back(1);
    assert_eq!(deque.get(0), Some(&0));
    assert_eq!(deque.get(1), Some(&1));
    assert_eq!(deque.get(2), None);

    let _ = deque.push_back(2);
    assert_eq!(deque.get(0), Some(&0));
    assert_eq!(deque.get(1), Some(&1));
    assert_eq!(deque.get(2), Some(&2));
    assert_eq!(deque.get(3), None);

    assert_eq!(deque.pop_front(), Some(0));
    assert_eq!(deque.get(0), Some(&1));
    assert_eq!(deque.get(1), Some(&2));
    assert_eq!(deque.get(2), None);

    assert_eq!(deque.pop_front(), Some(1));
    assert_eq!(deque.get(0), Some(&2));
    assert_eq!(deque.get(1), None);

    assert_eq!(deque.pop_front(), Some(2));
    assert_eq!(deque.get(0), None);
    assert_eq!(deque.get(1), None);
}

/// SDV test cases for deque `get_mut`.
///
/// # Brief
/// 1. create a queue.
/// 2. get mut element from queue.
/// 3. modify and check element of queue.
#[test]
fn sdv_vec_deque_test_get_mut() {
    let mut deque = FixedCapVecDeque::new(10, Global).unwrap();
    for i in 0..3 {
        let _ = deque.push_back(i);
    }

    if let Some(x) = deque.get_mut(1) {
        *x = -1
    };

    assert_eq!(deque.get_mut(0), Some(&mut 0));
    assert_eq!(deque.get_mut(1), Some(&mut -1));
    assert_eq!(deque.get_mut(2), Some(&mut 2));
    assert_eq!(deque.get_mut(3), None);

    assert_eq!(deque.pop_front(), Some(0));
    assert_eq!(deque.get_mut(0), Some(&mut -1));
    assert_eq!(deque.get_mut(1), Some(&mut 2));
    assert_eq!(deque.get_mut(2), None);
}

/// SDV test cases for deque `front`.
///
/// # Brief
/// 1. create a queue.
/// 2. get front element from queue.
/// 3. modify and check element of queue.
#[test]
fn sdv_vec_deque_test_front() {
    let mut ring = FixedCapVecDeque::new(10, Global).unwrap();
    let _ = ring.push_back(10);
    let _ = ring.push_back(20);
    assert_eq!(ring.front(), Some(&10));
    let _ = ring.pop_front();
    assert_eq!(*ring.front_mut().unwrap(), 20);
    let _ = ring.pop_front();
    assert_eq!(ring.front(), None);
}

/// SDV test cases for deque `contains`.
///
/// # Brief
/// 1. create a queue.
/// 2. push element into queue.
/// 3. check contains element of queue.
#[test]
fn sdv_vec_deque_test_contains() {
    let mut v = create_queue_from_arr_back([2, 3, 4], 5);

    assert!(v.contains(&3));
    assert!(!v.contains(&1));

    v.clear();

    assert!(!v.contains(&3));
}

/// SDV test cases for deque `is_empty`.
///
/// # Brief
/// 1. create a queue.
/// 2. push element into queue and pop.
/// 3. check queue is is_empty.
#[test]
#[allow(clippy::len_zero)]
fn sdv_vec_deque_test_is_empty() {
    let mut deque = FixedCapVecDeque::new(10, Global).unwrap();
    assert!(deque.is_empty());
    assert!(deque.iter().is_empty());
    assert!(deque.iter_mut().is_empty());
    let _ = deque.push_back(2);
    let _ = deque.push_back(3);
    let _ = deque.push_back(4);
    assert!(!deque.is_empty());
    assert!(!deque.iter().is_empty());
    assert!(!deque.iter_mut().is_empty());
    while deque.pop_front().is_some() {
        assert_eq!(deque.is_empty(), deque.len() == 0);
        assert_eq!(deque.iter().is_empty(), deque.iter().len() == 0);
        assert_eq!(deque.iter_mut().is_empty(), deque.iter_mut().len() == 0);
    }
    assert!(deque.is_empty());
    assert!(deque.iter().is_empty());
    assert!(deque.iter_mut().is_empty());
    assert!(deque.into_iter().is_empty());
}

/// SDV test cases for deque `push`.
///
/// # Brief
/// 1. create a queue.
/// 2. push zst element into queue and pop.
/// 3. check queue element.
#[test]
fn sdv_vec_deque_test_zero_sized_push() {
    const N: usize = 8;

    struct Zst;

    for len in 0..N {
        let mut deque = FixedCapVecDeque::new(len, Global).unwrap();
        assert_eq!(deque.len(), 0);
        assert!(deque.capacity() >= len);
        for case in 0..(1 << len) {
            assert_eq!(deque.len(), 0);
            for bit in 0..len {
                if case & (1 << bit) != 0 {
                    let _ = deque.push_front(Zst);
                } else {
                    let _ = deque.push_back(Zst);
                }
            }
            assert_eq!(deque.len(), len);
            assert_eq!(deque.iter().count(), len);
            deque.clear();
        }
    }
}
