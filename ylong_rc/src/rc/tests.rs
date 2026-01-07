//! Adapt tests from std to make sure functions of `RcWithAlloc` follow the same logic
//! with `Rc` in Rust std.

use crate::{RcWithAlloc, WeakWithAlloc};
use ylong_stdx_common::SafeClone;

use std::alloc::Global;
use std::boxed::Box;
use std::cell::RefCell;
use std::convert::From;
use std::mem::drop;

/// UT test case for cloning of RcWithAlloc.
///
/// # Brief
/// 1. Create a RcWithAlloc, and try to clone from it.
/// 2. Borrowing the values, they should have the same inner value.
#[test]
fn ut_rc_test_clone() {
    let x = RcWithAlloc::new(RefCell::new(5), Global).unwrap();
    let y = x.safe_clone().unwrap();
    *(*x).borrow_mut() = 20;
    assert_eq!(*y.borrow(), 20);
}

/// UT test case for the reading from a RcWithAlloc instance.
///
/// # Brief
/// 1. Create a RcWithAlloc, and read value in it. It should be the same as the one passed when
///    creating the instance.
#[test]
fn ut_rc_test_simple() {
    let x = RcWithAlloc::new(5, Global).unwrap();
    assert_eq!(*x, 5);
}

/// UT test case for the reading from a cloned RcWithAlloc instance.
///
/// # Brief
/// 1. Create a RcWithAlloc, and try to clone from it.
/// 2. Read value from the two. They should be the same as the one passed when creating the
///    instance.
#[test]
fn ut_rc_test_simple_clone() {
    let x = RcWithAlloc::new(5, Global).unwrap();
    let y = x.safe_clone().unwrap();
    assert_eq!(*x, 5);
    assert_eq!(*y, 5);
}

/// UT test case for destroying a RcWithAlloc instance.
///
/// # Brief
/// 1. Create a RcWithAlloc.
/// 2. Destroying it should not cause memory leak or other memory problems.
#[test]
fn ut_rc_test_destructor() {
    let x: RcWithAlloc<Box<_>, Global> = RcWithAlloc::new(Box::new(5), Global).unwrap();
    assert_eq!(**x, 5);
    assert_eq!(*x.as_ref().as_ref(), 5);
}

/// UT test case for relations between a RcWithAlloc instance and a WeakWithAlloc instance.
///
/// # Brief
/// 1. Create a RcWithAlloc, and downgrade it to a weak reference.
/// 2. The weak one should be able to upgrade to a strong reference, because the inner value still
///    lives.
#[test]
fn ut_rc_test_live() {
    let x = RcWithAlloc::new(5, Global).unwrap();
    let y = RcWithAlloc::downgrade(&x).unwrap();
    assert!(y.upgrade().unwrap().is_some());
}

/// UT test case for relations between a RcWithAlloc instance and a WeakWithAlloc instance.
///
/// # Brief
/// 1. Create a RcWithAlloc, and downgrade it to a weak reference.
/// 2. Drop the strong reference.
/// 3. The weak one shouldn't be able to upgrade to a strong reference, because the inner value is
///    dead.
#[test]
fn ut_rc_test_dead() {
    let x = RcWithAlloc::new(5, Global).unwrap();
    let y = RcWithAlloc::downgrade(&x).unwrap();
    drop(x);
    assert!(y.upgrade().unwrap().is_none());
}

/// UT test case for cyclic relations.
///
/// # Brief
/// 1. Create a RcWithAlloc, create a weak from it, and make them a cyclic relation.
/// 2. Dropping all of these should not cause any memory issues.
#[test]
fn ut_rc_weak_self_cyclic() {
    struct Cycle {
        x: RefCell<Option<WeakWithAlloc<Cycle, Global>>>,
    }

    let a = RcWithAlloc::new(
        Cycle {
            x: RefCell::new(None),
        },
        Global,
    )
    .unwrap();
    let b = RcWithAlloc::downgrade(&a.safe_clone().unwrap()).unwrap();
    *a.x.borrow_mut() = Some(b);

    // hopefully we don't double-free (or leak)...
}

/// UT test case for counts of RcWithAlloc.
///
/// # Brief
/// 1. Create a RcWithAlloc. It should be unique at first.
/// 2. Create a strong reference by cloning it. It should not be unique right now: it has two
///    strong reference.
/// 3. Create a weak reference by downgrading it. It should not be unique right now: it has a
///    weak reference.
/// 4. Dropping all the extra ones should make it back to a unique one.
#[test]
fn ut_rc_is_unique() {
    fn is_unique(rc: &RcWithAlloc<i32, Global>) -> bool {
        RcWithAlloc::strong_count(rc) == 1 && RcWithAlloc::weak_count(rc) == 0
    }
    let x = RcWithAlloc::new(3, Global).unwrap();
    assert!(is_unique(&x));
    let y = x.safe_clone().unwrap();
    assert!(!is_unique(&x));
    drop(y);
    assert!(is_unique(&x));
    let w = RcWithAlloc::downgrade(&x).unwrap();
    assert!(!is_unique(&x));
    drop(w);
    assert!(is_unique(&x));
}

/// UT test case for strong counts of RcWithAlloc.
///
/// # Brief
/// 1. Create a RcWithAlloc. It should only have one strong ref.
/// 2. Cloning it should increase the strong counts.
/// 3. Downgrading it shouldn't increase the strong counts.
/// 4. Upgrading a weak from it should increase the strong counts.
#[test]
fn ut_rc_test_strong_count() {
    let a = RcWithAlloc::new(0, Global).unwrap();
    assert_eq!(RcWithAlloc::strong_count(&a), 1);
    let w = RcWithAlloc::downgrade(&a).unwrap();
    assert_eq!(RcWithAlloc::strong_count(&a), 1);
    let b = w
        .upgrade()
        .unwrap()
        .expect("upgrade of live RcWithAlloc failed");
    assert_eq!(RcWithAlloc::strong_count(&b), 2);
    assert_eq!(RcWithAlloc::strong_count(&a), 2);
    drop(w);
    drop(a);
    assert_eq!(RcWithAlloc::strong_count(&b), 1);
    let c = b.safe_clone().unwrap();
    assert_eq!(RcWithAlloc::strong_count(&b), 2);
    assert_eq!(RcWithAlloc::strong_count(&c), 2);
}

/// UT test case for weak counts of RcWithAlloc.
///
/// # Brief
/// 1. Create a RcWithAlloc. It should only have zero weak ref.
/// 2. Cloning it shouldn't increase the weak counts.
/// 3. Downgrading it should increase the weak counts.
/// 4. Upgrading a weak from it shouldn't increase the weak counts.
#[test]
fn ut_rc_test_weak_count() {
    let a = RcWithAlloc::new(0, Global).unwrap();
    assert_eq!(RcWithAlloc::strong_count(&a), 1);
    assert_eq!(RcWithAlloc::weak_count(&a), 0);
    let w = RcWithAlloc::downgrade(&a).unwrap();
    assert_eq!(RcWithAlloc::strong_count(&a), 1);
    assert_eq!(RcWithAlloc::weak_count(&a), 1);
    drop(w);
    assert_eq!(RcWithAlloc::strong_count(&a), 1);
    assert_eq!(RcWithAlloc::weak_count(&a), 0);
    let c = a.safe_clone().unwrap();
    assert_eq!(RcWithAlloc::strong_count(&a), 2);
    assert_eq!(RcWithAlloc::weak_count(&a), 0);
    drop(c);
}
/// UT test case for weak counts of RcWithAlloc.
///
/// # Brief
/// 1. Create a RcWithAlloc. It should only have zero weak ref.
/// 2. Cloning it shouldn't increase the weak counts.
/// 3. Downgrading it should increase the weak counts.
/// 4. Upgrading a weak from it shouldn't increase the weak counts.
#[test]
fn ut_rc_weak_counts() {
    assert_eq!(
        WeakWithAlloc::weak_count(&WeakWithAlloc::<u64, Global>::new(Global)),
        0
    );
    assert_eq!(
        WeakWithAlloc::strong_count(&WeakWithAlloc::<u64, Global>::new(Global)),
        0
    );

    let a = RcWithAlloc::new(0, Global).unwrap();
    let w = RcWithAlloc::downgrade(&a).unwrap();
    assert_eq!(WeakWithAlloc::strong_count(&w), 1);
    assert_eq!(WeakWithAlloc::weak_count(&w), 1);
    let w2 = w.safe_clone().unwrap();
    assert_eq!(WeakWithAlloc::strong_count(&w), 1);
    assert_eq!(WeakWithAlloc::weak_count(&w), 2);
    assert_eq!(WeakWithAlloc::strong_count(&w2), 1);
    assert_eq!(WeakWithAlloc::weak_count(&w2), 2);
    drop(w);
    assert_eq!(WeakWithAlloc::strong_count(&w2), 1);
    assert_eq!(WeakWithAlloc::weak_count(&w2), 1);
    let a2 = a.safe_clone().unwrap();
    assert_eq!(WeakWithAlloc::strong_count(&w2), 2);
    assert_eq!(WeakWithAlloc::weak_count(&w2), 1);
    drop(a2);
    drop(a);
    assert_eq!(WeakWithAlloc::strong_count(&w2), 0);
    assert_eq!(WeakWithAlloc::weak_count(&w2), 0);
    drop(w2);
}

/// UT test case for try_unwrap of RcWithAlloc.
///
/// # Brief
/// 1. Create a RcWithAlloc.
/// 2. When it is unique, it's safe to unwrap it and the value will be returned.
/// 3. If there're other strong refs or weak refs, try_unwrap will not return the value, but the
///    instance itself.
#[test]
fn ut_rc_try_unwrap() {
    let x = RcWithAlloc::new(3, Global).unwrap();
    let val = RcWithAlloc::try_unwrap_with_alloc(x).unwrap().0;
    assert_eq!(val, 3);
    let x = RcWithAlloc::new(4, Global).unwrap();
    let _y = x.safe_clone().unwrap();
    let val = RcWithAlloc::try_unwrap_with_alloc(x).err().unwrap();
    assert_eq!(val, RcWithAlloc::new(4, Global).unwrap());
    let x = RcWithAlloc::new(5, Global).unwrap();
    let _w = RcWithAlloc::downgrade(&x).unwrap();
    let val = RcWithAlloc::try_unwrap_with_alloc(x).unwrap().0;
    assert_eq!(val, 5);
}

/// UT test case for into_inner of RcWithAlloc.
///
/// # Brief
/// 1. Create a RcWithAlloc.
/// 2. When it is unique, it's safe to call into_inner on it and the value will be returned.
/// 3. If there're other strong refs or weak refs, into_inner will not return the value but None.
#[test]
fn ut_rc_into_inner() {
    let x = RcWithAlloc::new(3, Global).unwrap();
    assert_eq!(RcWithAlloc::into_inner(x), Some(3));

    let x = RcWithAlloc::new(4, Global).unwrap();
    let y = RcWithAlloc::safe_clone(&x).unwrap();
    assert_eq!(RcWithAlloc::into_inner(x), None);
    assert_eq!(RcWithAlloc::into_inner(y), Some(4));

    let x = RcWithAlloc::new(5, Global).unwrap();
    let _w = RcWithAlloc::downgrade(&x).unwrap();
    assert_eq!(RcWithAlloc::into_inner(x), Some(5));
}

/// UT test case for get_mut of RcWithAlloc.
///
/// # Brief
/// 1. Create a RcWithAlloc.
/// 2. When it is unique, it's safe to get a mut reference of the inner value. The inner value
///    can be updated through this reference.
/// 3. If there're other strong refs or weak refs, get_mut will return None.
#[test]
fn ut_rc_get_mut() {
    let mut x = RcWithAlloc::new(3, Global).unwrap();
    *RcWithAlloc::get_mut(&mut x).unwrap() = 4;
    assert_eq!(*x, 4);
    let y = x.safe_clone().unwrap();
    assert!(RcWithAlloc::get_mut(&mut x).is_none());
    drop(y);
    assert!(RcWithAlloc::get_mut(&mut x).is_some());
    let _w = RcWithAlloc::downgrade(&x).unwrap();
    assert!(RcWithAlloc::get_mut(&mut x).is_none());
}

/// UT test case for make_mut of RcWithAlloc.
///
/// # Brief
/// 1. Create a RcWithAlloc.
/// 2. When it is unique, make_mut will return a mut reference of the inner value and change
///    nothing.
/// 3. If there're other strong refs, make_mut will clone the value and make new strong ref.
#[test]
fn ut_rc_test_cowrc_clone_make_unique() {
    let mut cow0 = RcWithAlloc::new(75, Global).unwrap();
    let mut cow1 = cow0.safe_clone().unwrap();
    let mut cow2 = cow1.safe_clone().unwrap();

    assert_eq!(75, *RcWithAlloc::make_mut(&mut cow0).unwrap());
    assert_eq!(75, *RcWithAlloc::make_mut(&mut cow1).unwrap());
    assert_eq!(75, *RcWithAlloc::make_mut(&mut cow2).unwrap());

    *RcWithAlloc::make_mut(&mut cow0).unwrap() += 1;
    *RcWithAlloc::make_mut(&mut cow1).unwrap() += 2;
    *RcWithAlloc::make_mut(&mut cow2).unwrap() += 3;

    assert_eq!(76, *cow0);
    assert_eq!(77, *cow1);
    assert_eq!(78, *cow2);

    // none should point to the same backing memory
    assert_ne!(*cow0, *cow1);
    assert_ne!(*cow0, *cow2);
    assert_ne!(*cow1, *cow2);
}

/// UT test case for make_mut of RcWithAlloc.
///
/// # Brief
/// 1. Create a RcWithAlloc.
/// 2. When it is unique, make_mut will return a mut reference of the inner value and change
///    nothing.
/// 3. If there're other strong refs, make_mut will clone the value and make new strong ref.
#[test]
fn ut_rc_test_cowrc_clone_unique2() {
    let mut cow0 = RcWithAlloc::new(75, Global).unwrap();
    let cow1 = cow0.safe_clone().unwrap();
    let cow2 = cow1.safe_clone().unwrap();

    assert_eq!(75, *cow0);
    assert_eq!(75, *cow1);
    assert_eq!(75, *cow2);

    *RcWithAlloc::make_mut(&mut cow0).unwrap() += 1;

    assert_eq!(76, *cow0);
    assert_eq!(75, *cow1);
    assert_eq!(75, *cow2);

    // cow1 and cow2 should share the same contents
    // cow0 should have a unique reference
    assert_ne!(*cow0, *cow1);
    assert_ne!(*cow0, *cow2);
    assert_eq!(*cow1, *cow2);
}

/// UT test case for make_mut of WeakWithAlloc.
///
/// # Brief
/// 1. Create a RcWithAlloc.
/// 2. When it is unique, make_mut will return a mut reference of the inner value and change
///    nothing.
/// 3. If there're other weak refs, make_mut will disassociate the strong & weak relations.
#[test]
fn ut_rc_test_cowrc_clone_weak() {
    let mut cow0 = RcWithAlloc::new(75, Global).unwrap();
    let cow1_rc = RcWithAlloc::downgrade(&cow0).unwrap();

    assert_eq!(75, *cow0);
    assert_eq!(75, *cow1_rc.upgrade().unwrap().unwrap());

    *(RcWithAlloc::make_mut(&mut cow0).unwrap()) += 1;

    assert_eq!(76, *cow0);
    assert!(cow1_rc.upgrade().unwrap().is_none());
}

/// UT test case for format of RcWithAlloc.
///
/// # Brief
/// 1. Create a RcWithAlloc.
/// 2. Printing it should be the same value as the expected.
#[test]
fn ut_rc_test_show() {
    let foo = RcWithAlloc::new(75, Global).unwrap();
    assert_eq!(
        format!("{foo:?}"),
        "RcWithAlloc: {Value: 75, Allocator: Global}"
    );
}

// Keep this ut to make sure all uts from std are adapted, but edit it because `from` trait hasn't
// be implemented. It should be reverted back when this trait is implemented.
/// UT test case for creating a RcWithAlloc from a DST.
///
/// # Brief
/// 1. Create a RcWithAlloc from a DST object. Creating and cloning of it should succeed.
#[test]
fn ut_rc_test_unsized() {
    let foo = RcWithAlloc::new([1, 2, 3], Global).unwrap();
    assert_eq!(foo, foo.safe_clone().unwrap());
}

/// UT test case for creating a Weak ref.
///
/// # Brief
/// 1. Create a WeakWithAlloc. Creating should succeed.
/// 2. Upgrading it should return None.
#[test]
fn ut_rc_test_new_weak() {
    let foo: WeakWithAlloc<usize, Global> = WeakWithAlloc::new(Global);
    assert!(foo.upgrade().unwrap().is_none());
}

// Keep this ut to make sure all uts from std are adapted, but edit it because `from` trait hasn't
// be implemented. It should be reverted back when this trait is implemented.
/// UT test case for creating a RcWithAlloc from a str.
///
/// # Brief
/// 1. Create a RcWithAlloc from a str object.
/// 2. Checking its value.
#[test]
fn ut_rc_test_from_str() {
    let r = RcWithAlloc::new("foo", Global).unwrap();

    assert_eq!(&r[..], "foo");
}

/// UT test case for creating a RcWithAlloc from a slice.
///
/// # Brief
/// 1. Create a RcWithAlloc from a slice object.
/// 2. Checking its value.
#[test]
fn ut_rc_test_copy_from_slice() {
    let s: &[u32] = &[1, 2, 3];
    let r = RcWithAlloc::new(s, Global).unwrap();

    assert_eq!(&r[..], [1, 2, 3]);
}

/// UT test case for creating a RcWithAlloc from a slice.
///
/// # Brief
/// 1. Create a RcWithAlloc from a slice object.
/// 2. Checking its value.
#[test]
fn ut_rc_test_clone_from_slice() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct X(u32);

    let s: &[X] = &[X(1), X(2), X(3)];
    let r = RcWithAlloc::new(s, Global).unwrap();

    assert_eq!(&r[..], s);
}

/// UT test case for creating a RcWithAlloc from a box.
///
/// # Brief
/// 1. Create a RcWithAlloc from a box object.
/// 2. Checking its value.
#[test]
fn ut_rc_test_from_box() {
    let b: Box<u32> = Box::new(123);
    let r = RcWithAlloc::new(b, Global).unwrap();

    assert_eq!(**r, 123);
}

/// UT test case for creating a RcWithAlloc from a boxed str.
///
/// # Brief
/// 1. Create a RcWithAlloc from a boxed str object.
/// 2. Checking its value.
#[test]
fn ut_rc_test_from_box_str() {
    use std::string::String;

    let s = String::from("foo").into_boxed_str();
    let r = RcWithAlloc::new(s, Global).unwrap();

    assert_eq!(&r[..], "foo");
}

/// UT test case for creating a RcWithAlloc from a boxed slice.
///
/// # Brief
/// 1. Create a RcWithAlloc from a boxed slice object.
/// 2. Checking its value.
#[test]
fn ut_rc_test_from_box_slice() {
    let s = vec![1, 2, 3].into_boxed_slice();
    let r = RcWithAlloc::new(s, Global).unwrap();

    assert_eq!(r[..], [1, 2, 3]);
}

/// UT test case for creating a RcWithAlloc from a boxed dyn trait.
///
/// # Brief
/// 1. Create a RcWithAlloc from a boxed dyn trait object.
/// 2. Checking its value.
#[test]
fn ut_rc_test_from_box_trait() {
    use std::fmt::Display;
    use std::string::ToString;

    let b: Box<dyn Display> = Box::new(123);
    let r = RcWithAlloc::new(b, Global).unwrap();

    assert_eq!(r.to_string(), "123");
}

/// UT test case for creating a RcWithAlloc from a boxed dyn trait.
///
/// # Brief
/// 1. Create a RcWithAlloc from a boxed dyn trait and zero sized object.
/// 2. Checking its value.
#[test]
fn ut_rc_test_from_box_trait_zero_sized() {
    use std::fmt::Debug;

    let b: Box<dyn Debug> = Box::new(());
    let r = RcWithAlloc::new(b, Global).unwrap();

    assert_eq!(
        format!("{r:?}"),
        "RcWithAlloc: {Value: (), Allocator: Global}"
    );
}

/// UT test case for creating a RcWithAlloc from a vec.
///
/// # Brief
/// 1. Create a RcWithAlloc from a vec object.
/// 2. Checking its value.
#[test]
fn ut_rc_test_from_vec() {
    let v = vec![1, 2, 3];
    let r = RcWithAlloc::new(v, Global).unwrap();

    assert_eq!(&r[..], [1, 2, 3]);
}
