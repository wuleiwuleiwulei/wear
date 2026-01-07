//! Adapt tests from std to make sure functions of `RcWithAlloc` follow the same logic
//! with `Rc` in Rust std.
#![feature(allocator_api)]

use core::alloc::{AllocError, Allocator, Layout};
use core::ptr::NonNull;
use std::alloc::Global;
use std::cell::RefCell;
use std::cmp::PartialEq;
use ylong_box::AllocatorBox;
use ylong_rc::{RcWithAlloc, WeakWithAlloc};
use ylong_stdx_common::{ContainerError, SafeClone};

/// SDV test case for cyclic relations using RcWithAlloc and WeakWithAlloc.
///
/// # Brief
/// 1. Create a struct with RcWithAlloc and WeakWithAlloc.
/// 2. Dropping it should not cause any memory issues.
#[test]
fn sdv_rc_cycle() {
    struct Cycle {
        strong: RcWithAlloc<u32, Global>,
        weak: WeakWithAlloc<u32, Global>,
    }

    let x = RcWithAlloc::new(3, Global).unwrap();
    let weak = RcWithAlloc::downgrade(&x).unwrap();
    // Should drop correctly and not introduce memory leak
    let c = Cycle { strong: x, weak };
    let c_rc = RcWithAlloc::new(c, Global).unwrap();
    assert_eq!(RcWithAlloc::strong_count(&c_rc), 1);
    assert_eq!(RcWithAlloc::weak_count(&c_rc), 0);
    let (c, _) = RcWithAlloc::into_inner_with_alloc(c_rc).unwrap();
    assert_eq!(*c.strong, 3);
    assert_eq!(WeakWithAlloc::strong_count(&c.weak), 1);
    assert_eq!(WeakWithAlloc::weak_count(&c.weak), 1);
    let another_weak = RcWithAlloc::downgrade(&c.strong).unwrap();
    drop(c);
    assert!(another_weak.upgrade().unwrap().is_none());
}

/// SDV test case for getting allocator of RcWithAlloc and WeakWithAlloc.
///
/// # Brief
/// 1. Create a RcWithAlloc and its WeakWithAlloc.
/// 2. Getting allocators from them and freeing the values inside of them should not cause
///    any memory issues, which means those values are allocated by these allocators.
#[test]
fn sdv_rc_get_allocator() {
    let val = AllocatorBox::new(8, Global).unwrap();
    let ptr = AllocatorBox::into_raw(val);
    let rc = RcWithAlloc::new(ptr, Global).unwrap();
    let (val, alloc) = RcWithAlloc::into_inner_with_alloc(rc).unwrap();
    let b = unsafe { AllocatorBox::from_raw(val, alloc) };
    assert_eq!(*b, 8);

    let ptr = AllocatorBox::into_raw(b);
    let rc = RcWithAlloc::new(ptr, Global).unwrap();
    let (val, alloc) = RcWithAlloc::try_unwrap_with_alloc(rc).unwrap();
    let b = unsafe { AllocatorBox::from_raw(val, alloc) };
    assert_eq!(*b, 8);

    let ptr = AllocatorBox::into_raw(b);
    let rc = RcWithAlloc::new(ptr, Global).unwrap();
    let alloc = *RcWithAlloc::allocator(&rc);
    let val = RcWithAlloc::into_inner(rc).unwrap();
    let b = unsafe { AllocatorBox::from_raw(val, alloc) };
    assert_eq!(*b, 8);
}

/// SDV test case for dangling WeakWithAllocs.
///
/// # Brief
/// 1. Create a new WeakWithAlloc.
/// 2. Test that it is dangling, and it's safe to free it.
#[test]
fn sdv_rc_dangling() {
    let w: WeakWithAlloc<(), Global> = WeakWithAlloc::new(Global);
    assert!(w.upgrade().unwrap().is_none());
    assert_eq!(WeakWithAlloc::strong_count(&w), 0);
    assert_eq!(WeakWithAlloc::weak_count(&w), 0);
    assert_eq!(
        format!("{w:?}"),
        "WeakWithAlloc: {Value: None, Allocator: Global}"
    );
}

/// SDV test case for using RcWithAlloc and WeakWithAllocs.
///
/// # Brief
/// 1. Create a batch of RcWithAlloc and WeakWithAlloc.
/// 2. Cloning, counting, upgrading / downgrading on them should work as expected.
/// 3. Freeing all of these should not cause any memory issue.
#[test]
fn sdv_rc_usage() {
    let x = RcWithAlloc::new(3, Global).unwrap();
    let x2 = x.safe_clone().unwrap();
    assert_eq!(RcWithAlloc::strong_count(&x), 2);
    assert_eq!(RcWithAlloc::weak_count(&x), 0);
    let w = RcWithAlloc::downgrade(&x).unwrap();
    let w2 = w.safe_clone().unwrap();
    assert_eq!(RcWithAlloc::weak_count(&x), 2);
    let mut x3 = RcWithAlloc::new(6, Global).unwrap();
    assert_eq!(*x3.as_ref(), 6);
    x3.safe_clone_from(&x2).unwrap();
    assert_eq!(*x3.as_ref(), 3);
    assert_eq!(RcWithAlloc::strong_count(&x), 3);
    let mut x4 = w.upgrade().unwrap().unwrap();
    assert_eq!(RcWithAlloc::strong_count(&x), 4);
    assert_eq!(RcWithAlloc::weak_count(&x), 2);

    // fmt
    assert_eq!(
        format!("{x:?}"),
        "RcWithAlloc: {Value: 3, Allocator: Global}"
    );
    assert_eq!(
        format!("{w:?}"),
        "WeakWithAlloc: {Value: 3, Allocator: Global}"
    );

    // make_mut with many strongs
    assert_eq!(RcWithAlloc::strong_count(&x4), 4);
    *RcWithAlloc::make_mut(&mut x4).unwrap() = 1;
    assert_eq!(*x4, 1);
    assert_eq!(RcWithAlloc::strong_count(&x4), 1);
    assert_eq!(RcWithAlloc::weak_count(&x4), 0);

    // into_inner and try_unwrap
    assert!(RcWithAlloc::into_inner_with_alloc(x).is_none());
    assert_eq!(RcWithAlloc::strong_count(&x2), 2);
    assert!(RcWithAlloc::try_unwrap_with_alloc(x2).is_err());
    assert_eq!(RcWithAlloc::strong_count(&x3), 1);
    assert_eq!(RcWithAlloc::weak_count(&x3), 2);
    let x5 = x3.safe_clone().unwrap();
    assert!(RcWithAlloc::into_inner(x5).is_none());

    // get_mut
    assert!(RcWithAlloc::get_mut(&mut x3).is_none());
    drop(w);
    drop(w2);
    assert!(RcWithAlloc::get_mut(&mut x3).is_some());
    *RcWithAlloc::get_mut(&mut x3).unwrap() = 5;
    assert_eq!(RcWithAlloc::into_inner(x3).unwrap(), 5);

    // make_mut with a Weak
    assert_eq!(*x4, 1);
    let w = RcWithAlloc::downgrade(&x4).unwrap();
    assert_eq!(RcWithAlloc::strong_count(&x4), 1);
    assert_eq!(RcWithAlloc::weak_count(&x4), 1);
    *RcWithAlloc::make_mut(&mut x4).unwrap() = 2;
    assert_eq!(*x4, 2);
    assert_eq!(RcWithAlloc::strong_count(&x4), 1);
    assert_eq!(RcWithAlloc::weak_count(&x4), 0);

    // Check the association between the Weak and the Strong
    assert!(w.upgrade().unwrap().is_none());
    assert_eq!(WeakWithAlloc::strong_count(&w), 0);
    assert_eq!(WeakWithAlloc::weak_count(&w), 0);
}

/// SDV test case for passing NAN into RcWithALloc.
///
/// # Brief
/// 1. Create a RcWithAlloc with the value f32::NAN.
/// 2. Rc instance shouldn't be equal.
#[test]
fn sdv_rc_float_nan_ne() {
    let x = RcWithAlloc::new(f32::NAN, Global).unwrap();
    assert_ne!(x, x);
    assert!(!(x == x));
}

/// SDV test case for partial eq of RcWithAlloc.
///
/// # Brief
/// 1. Create a RcWithAlloc.
/// 2. Checking equality of RcWithAlloc instances with objects that implements only partial_eq
///    traits will only test their values.
#[test]
fn sdv_rc_partial_eq() {
    struct TestPEq(RefCell<usize>);
    impl PartialEq for TestPEq {
        fn eq(&self, other: &TestPEq) -> bool {
            *self.0.borrow_mut() += 1;
            *other.0.borrow_mut() += 1;
            true
        }
    }
    let x = RcWithAlloc::new(TestPEq(RefCell::new(0)), Global).unwrap();
    assert!(x == x);
    assert!(!(x != x));
    assert_eq!(*x.0.borrow(), 4);
}

/// SDV test case for partial eq of RcWithAlloc.
///
/// # Brief
/// 1. Create a RcWithAlloc.
/// 2. Checking equality of RcWithAlloc instances with objects that implements both partial_eq and
///    eq traits will test their pointers and their values.
#[test]
fn sdv_rc_eq() {
    #[derive(Eq)]
    struct TestEq(RefCell<usize>);
    impl PartialEq for TestEq {
        fn eq(&self, other: &TestEq) -> bool {
            *self.0.borrow_mut() += 1;
            *other.0.borrow_mut() += 1;
            true
        }
    }
    let x = RcWithAlloc::new(TestEq(RefCell::new(0)), Global).unwrap();
    assert!(x == x);
    assert!(!(x != x));
    assert_eq!(*x.0.borrow(), 0);
}

/// SDV test case for freeing of WeakWithAlloc
///
/// # Brief
/// 1. Create a WeakWithAlloc.
/// 2. If `#[may_dangle]` hasn't been marked with the Drop trait of WeakWithAlloc, this test case
///    can't be compiled because the compiler will reserve to think that the inner value will be
///    dangling, as WeakWithAlloc won't drop the value. We know it's safe, so the marker should be
///    there and this case should run successfully.
#[test]
fn sdv_rc_weak_may_dangle() {
    fn hmm<'a>(val: &'a mut WeakWithAlloc<&'a str, Global>) -> WeakWithAlloc<&'a str, Global> {
        val.safe_clone().unwrap()
    }

    // Without #[may_dangle] we get:
    let mut val = WeakWithAlloc::new(Global);
    hmm(&mut val);
    //  ~~~~~~~~ borrowed value does not live long enough
    //
    // `val` dropped here while still borrowed
    // borrow might be used here, when `val` is dropped and runs the `Drop` code for type `std::rc::WeakWithAlloc`
}

#[derive(Clone)]
struct AllocatorAlwaysFail;

unsafe impl Allocator for AllocatorAlwaysFail {
    fn allocate(&self, _layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Err(AllocError)
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
}

/// SDV test case for rc with allocator that always returns error.
///
/// # Brief
/// 1. Create zero cap map with AllocatorAlwaysFail. This should be fine but the map is always
///    empty.
/// 2. Create non-zero cap map with AllocatorAlwaysFail. This should fail.
#[test]
fn sdv_rc_with_alloc_always_fail() {
    match RcWithAlloc::new(3, AllocatorAlwaysFail).err().unwrap() {
        ContainerError::AllocFailure(_) => {}
        _ => panic!(),
    }
}
