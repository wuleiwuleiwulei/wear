#![feature(allocator_api)]
#![feature(iter_advance_by)]
#![feature(strict_provenance)]
#![feature(specialization)]
#![allow(incomplete_features)]
#![feature(exact_size_is_empty)]

use std::alloc::{AllocError, Allocator, Global, Layout};
use std::ptr::NonNull;
use ylong_stdx_common::{ContainerError, SafeClone};
use ylong_vec::FixedCapVec;

fn create_vec_from_arr<T, const N: usize>(arr: [T; N], cap: usize) -> FixedCapVec<T, Global> {
    let mut v = FixedCapVec::new(cap, Global).unwrap();
    for e in arr {
        let _ = v.push(e);
    }
    v
}

struct FailAllocator;

unsafe impl Allocator for FailAllocator {
    fn allocate(&self, _layout: Layout) -> Result<NonNull<[u8]>, core::alloc::AllocError> {
        Err(core::alloc::AllocError)
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
}

/// SDV test cases for vec.
///
/// # Brief
/// 1. create a vec
/// 2. modify and remove element
#[test]
fn vec_mul_op() {
    let mut v1 = create_vec_from_arr([1, 2, 3], 3);
    assert!(!v1.is_empty());
    assert!(v1.is_full());

    let s = v1.as_mut_slice();
    assert_eq!(s[0], 1);

    assert_eq!(v1.remove(0).unwrap(), 1);
    assert_eq!(v1.len(), 2);

    assert!(v1.remove(2).is_err());

    assert!(v1.extend_from_slice(&[1, 2]).is_err());
}

/// SDV test cases for vec.
///
/// # Brief
/// 1. create a vec
/// 2. transfer vec into ref
#[test]
fn vec_as() {
    let mut v1 = create_vec_from_arr([1, 2, 3], 6);

    let r: &FixedCapVec<i32, _> = v1.as_ref();
    assert_eq!(r.as_ptr(), v1.as_ptr());

    let r2: &FixedCapVec<i32, _> = v1.as_mut();
    assert_eq!(r2.as_ptr(), v1.as_ptr());

    let r3: &[i32] = v1.as_ref();
    assert_eq!(r3.as_ptr(), v1.as_ptr());

    let r4: &[i32] = v1.as_mut();
    assert_eq!(r4.as_ptr(), v1.as_ptr());
}

/// SDV test cases for vec `new`.
///
/// # Brief
/// 1. create a allocator that always fail
/// 2. new vec with capacity 0, fail
#[test]
fn vec_allocator_fail_1() {
    let mut v = FixedCapVec::<u32, FailAllocator>::new(0, FailAllocator).unwrap();

    assert!(v.push(1).is_err())
}

/// SDV test cases for vec `new`.
///
/// # Brief
/// 1. create a allocator that always fail
/// 2. new vec with capacity 1, fail
#[test]
fn vec_allocator_fail_2() {
    let v = FixedCapVec::<u32, FailAllocator>::new(1, FailAllocator);

    assert!(v.is_err())
}

/// SDV test cases for vec `new`.
///
/// # Brief
/// 1. create a allocator that always fail
/// 2. new vec with zst, success
#[test]
fn vec_allocator_fail_zst() {
    let v = FixedCapVec::<(), FailAllocator>::new(1, FailAllocator);

    assert!(v.is_ok());
    assert!(v.unwrap().push(()).is_ok())
}

/// SDV test cases for vec `drop`.
///
/// # Brief
/// 1. create a vec, include a ref of T
/// 2. drop T;then vec may include a dangle ptr, but it does noting for drop
/// 3. if the case compilation is success, the case is correct.
#[test]
fn vec_may_dangle() {
    let mut v = FixedCapVec::<&str, Global>::new(10, Global).unwrap();

    let s: String = "Short-lived".into();
    let _ = v.push(&s);
    drop(s);
}

/// SDV test cases for IntoIter `drop`.
///
/// # Brief
/// 1. create a vec, include a ref of T
/// 2. drop T;then vec may include a dangle ptr, but it does noting for drop
/// 3. if the case compilation is success, the case is correct.
#[test]
fn vec_intoiter_may_dangle() {
    let mut v = FixedCapVec::<&str, Global>::new(10, Global).unwrap();

    let s: String = "Short-lived".into();
    let _ = v.push(&s);
    let _it = v.into_iter();
    drop(s);
}

/// SDV test cases for vec `truncate`.
///
/// # Brief
/// 1. create a vec.
/// 2. truncate vec.
#[test]
fn vec_truncate_drop() {
    static mut TEST_FLAG: usize = 0;
    struct DropCounter;

    impl Drop for DropCounter {
        fn drop(&mut self) {
            unsafe {
                TEST_FLAG += 1;
            }
        }
    }

    let mut v = FixedCapVec::<DropCounter, Global>::new(5, Global).unwrap();
    for _i in 0..5 {
        let _ = v.push(DropCounter);
    }
    assert_eq!(v.len(), 5);

    v.truncate(6);
    assert_eq!(v.len(), 5);
    v.truncate(5);
    assert_eq!(v.len(), 5);
    v.truncate(4);
    assert_eq!(v.len(), 4);
    v.truncate(0);
    assert_eq!(v.len(), 0);
    unsafe {
        assert_eq!(TEST_FLAG, 5);
    }
}

/// SDV test cases for vec `safe_clone`.
///
/// # Brief
/// 1. clone vec
/// 2. check it will call safe_clone of element, so it will panic
#[test]
fn vec_safe_clone_impl() {
    static mut TEST_FLAG: usize = 0;
    #[derive(Clone)]
    struct Temp {
        i: usize,
    }

    impl SafeClone for Temp {
        fn safe_clone(&self) -> Result<Self, ContainerError> {
            unsafe {
                TEST_FLAG = self.i;
            }
            Ok(self.clone())
        }

        fn safe_clone_from(&mut self, _other: &Self) -> Result<(), ContainerError> {
            todo!()
        }
    }

    let temp = Temp { i: 1 };

    let mut v1 = FixedCapVec::<Temp, Global>::new(10, Global).unwrap();
    let _ = v1.push(temp);

    let _v2 = v1.safe_clone();

    unsafe {
        assert_eq!(TEST_FLAG, 1);
    }
}

/// SDV test cases for vec `safe_clone_from`.
///
/// # Brief
/// 1. clone_from vec
/// 2. check it will call clone_from of element
#[test]
fn vec_safe_clone_from_impl() {
    static mut TEST_FLAG: usize = 0;

    #[derive(Clone)]
    struct Temp {
        i: u8,
    }

    impl SafeClone for Temp {
        fn safe_clone(&self) -> Result<Self, ContainerError> {
            Ok(self.clone())
        }

        fn safe_clone_from(&mut self, other: &Self) -> Result<(), ContainerError> {
            unsafe {
                TEST_FLAG = 1;
            }
            self.clone_from(other);
            Ok(())
        }
    }

    let temp1 = Temp { i: 1 };
    let temp2 = Temp { i: 2 };
    let temp3 = Temp { i: 3 };

    let mut v1 = FixedCapVec::<Temp, Global>::new(1, Global).unwrap();
    let _ = v1.push(temp1);

    let mut v2 = FixedCapVec::<Temp, Global>::new(10, Global).unwrap();
    let _ = v2.push(temp2);

    assert!(v1.safe_clone_from(&v2).is_ok());
    assert_eq!(v1[0].i, 2);

    let _ = v2.push(temp3);
    assert!(v1.safe_clone_from(&v2).is_err());
    assert_eq!(v1[0].i, 2); // safe clone failed, does nothing

    unsafe {
        assert_eq!(TEST_FLAG, 1);
    }
}

/// SDV test cases for vec `new`.
///
/// # Brief
/// 1. new, valid arg, succuess.
/// 2. new, invalid arg, fail.
#[test]
fn vec_new() {
    let v1 = FixedCapVec::<u8, Global>::new(10, Global);
    assert!(v1.is_ok());

    // In 64 and 32 bit, memory allocation is not allowed for >= isize::MAX + 1
    let v1 = FixedCapVec::<u8, Global>::new(isize::MAX as usize + 1, Global);
    assert!(v1.is_err());
}

/// SDV test cases for vec `new`.
///
/// # Brief
/// 1. new, valid arg, succuess.
/// 2. new, invalid arg, fail.
#[test]
#[cfg_attr(feature = "__asan", ignore)]
fn vec_new_2() {
    let v1 = FixedCapVec::<u8, Global>::new(isize::MAX as usize, Global);

    if usize::BITS == 64 {
        assert!(v1.is_err()); // For common 64 bit machine, memory for isize::MAX is surely OOM.
    } else {
        assert!(v1.is_ok()); // Note: This branch is for 32 bit, memory allocation for isize::MAX may success.Or maybe fail.
    }
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

/// SDV test cases for IntoIter `drop`.
///
/// # Brief
/// 1. drop IntoIter,
/// 2. check whether drop each element.
/// 3. check whether drop allocator.
#[test]
fn vec_intoiter_drop() {
    static mut TEST_FLAG: usize = 0;
    static mut TEST_FLAG2: usize = 0;

    struct AllocDropCounter {
        count: u32,
    }

    impl Drop for AllocDropCounter {
        fn drop(&mut self) {
            println!("drop {}", self.count);
            unsafe {
                TEST_FLAG2 += 1;
            }
        }
    }

    unsafe impl Allocator for AllocDropCounter {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            Global.allocate(layout)
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            Global.deallocate(ptr, layout)
        }
    }

    struct DropCounter {
        count: u32,
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            println!("drop {}", self.count);
            unsafe {
                TEST_FLAG += 1;
            }
        }
    }

    let alloc = AllocDropCounter { count: 0 };

    let d1 = DropCounter { count: 1 };
    let d2 = DropCounter { count: 2 };
    let d3 = DropCounter { count: 3 };
    let mut v1 = FixedCapVec::<DropCounter, _>::new(10, alloc).unwrap();

    let _ = v1.push(d1);
    let _ = v1.push(d2);
    let _ = v1.push(d3);

    let it = v1.into_iter();
    drop(it);
    unsafe {
        assert_eq!(TEST_FLAG, 3);
        assert_eq!(TEST_FLAG2, 1);
    }
}

/// SDV test cases for IntoIter `drop`.
///
/// # Brief
/// 1. push 3 element into vec
/// 2. pop 1 element from IntoIter
/// 3. drop IntoIter.
/// 4. check whether drop each element.
#[test]
fn vec_intoiter_drop_2() {
    static mut TEST_FLAG: usize = 0;

    struct DropCounter {
        count: u32,
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            println!("drop {}", self.count);
            unsafe {
                TEST_FLAG += 1;
            }
        }
    }

    let d1 = DropCounter { count: 1 };
    let d2 = DropCounter { count: 2 };
    let d3 = DropCounter { count: 3 };
    let mut v1 = FixedCapVec::<DropCounter, _>::new(10, Global).unwrap();

    let _ = v1.push(d1);
    let _ = v1.push(d2);
    let _ = v1.push(d3);

    let mut it = v1.into_iter();
    let c = it.next().unwrap();
    drop(it);
    unsafe {
        assert_eq!(TEST_FLAG, 2);
    }
    println!("poped :{}", c.count);
}

/// SDV test cases for IntoIter `debug`.
///
/// # Brief
/// 1. create IntoIter.
/// 2. get debug message of IntoIter.
#[test]
fn test_into_iter_debug_impl() {
    #[derive(Debug)]
    struct Temp {
        _i: u8,
    }

    let mut vec = create_vec_from_arr([], 4);
    for i in 0..4 {
        let _ = vec.push(Temp { _i: i });
    }
    let into_iter = vec.into_iter();
    assert!(!into_iter.is_empty());
    let debug = format!("{into_iter:?}");
    assert_eq!(
        debug,
        "IntoIter([Temp { _i: 0 }, Temp { _i: 1 }, Temp { _i: 2 }, Temp { _i: 3 }])"
    );
}
