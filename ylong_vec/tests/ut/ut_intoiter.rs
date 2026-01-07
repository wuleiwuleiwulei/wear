use super::*;
use core::slice::SlicePattern;
use std::alloc::Global;

fn create_vec_from_arr<T, const N: usize>(arr: [T; N], cap: usize) -> FixedCapVec<T, Global> {
    let mut v = FixedCapVec::new(cap, Global).unwrap();
    for e in arr {
        let _ = v.push(e);
    }
    v
}

/// UT test cases for IntoIter.
///
/// # Brief
/// 1. create IntoIter.
/// 2. use iterator.
#[test]
fn into_iter() {
    let vec = create_vec_from_arr([1, 2, 3, 4, 5], 10);

    let mut x = 0;
    let it = vec.into_iter();
    for e in it {
        x += e;
    }
    assert_eq!(x, 15);
}

/// UT test cases for IntoIter.
///
/// # Brief
/// 1. create IntoIter of ZST.
/// 2. use iterator.
#[test]
#[allow(clippy::unit_cmp)] // mask warning for zst
fn into_iter_zst() {
    let vec = create_vec_from_arr([(); 5], 10);

    let mut x = 0;
    let it = vec.into_iter();
    for e in it {
        x += 1;
        assert_eq!(e, ());
    }
    assert_eq!(x, 5)
}

/// UT test cases for IntoIter for ref vec.
///
/// # Brief
/// 1. create IntoIter of ref vec.
/// 2. use iterator.
#[test]
fn into_iter_ref_vec() {
    let mut vec = create_vec_from_arr([1, 2, 3, 4, 5], 10);

    let r = &vec;
    let it_ref = r.into_iter();
    assert_eq!(it_ref.as_slice(), vec.as_slice());

    let mr = &mut vec;

    for e in mr {
        *e = 0;
    }
    assert_eq!(vec.as_slice(), [0; 5]);
}

/// UT test cases for IntoIter `size_hint`.
///
/// # Brief
/// 1. create IntoIter.
/// 2. get size hint.
#[test]
fn into_iter_size_hint() {
    let vec = create_vec_from_arr([1, 2, 3, 4, 5], 10);

    let it = vec.into_iter();
    assert_eq!(it.size_hint(), (5, Some(5)));
}

/// UT test cases for IntoIter `size_hint`.
///
/// # Brief
/// 1. create IntoIter of ZST.
/// 2. get size hint.
#[test]
fn into_iter_size_hint_zst() {
    let vec = create_vec_from_arr([(); 5], 10);

    let it = vec.into_iter();
    assert_eq!(it.size_hint(), (5, Some(5)));
}

/// UT test cases for IntoIter `as_slice`.
///
/// # Brief
/// 1. create IntoIter.
/// 2. get slice.
/// 3. check slice is equal with iter
#[test]
fn into_iter_as_slice() {
    let vec = create_vec_from_arr([1, 2, 3, 4, 5], 10);

    let it = vec.into_iter();
    assert_eq!(it.as_slice(), [1, 2, 3, 4, 5]);
}

/// UT test cases for IntoIter `as_mut_slice`.
///
/// # Brief
/// 1. create IntoIter.
/// 2. get mut_slice.
/// 3. check slice is equal with iter
#[test]
fn into_iter_as_mut_slice() {
    let vec = create_vec_from_arr([1, 2, 3, 4, 5], 10);

    let mut it = vec.into_iter();
    assert_eq!(it.as_mut_slice(), [1, 2, 3, 4, 5]);
}

/// UT test cases for IntoIter `is_empty`.
///
/// # Brief
/// 1. create IntoIter.
/// 2. check Iter if is empty.
#[test]
fn into_iter_is_empty() {
    let vec = create_vec_from_arr([1, 2, 3, 4, 5], 10);

    let it = vec.into_iter();
    assert!(!it.is_empty());

    let vec2 = create_vec_from_arr([0; 0], 10);
    let it2 = vec2.into_iter();
    assert!(it2.is_empty());
}

/// UT test cases for IntoIter `debug`.
///
/// # Brief
/// 1. create IntoIter.
/// 2. get debug message of IntoIter.
#[test]
fn test_into_iter_debug() {
    let vec = create_vec_from_arr(['a', 'b', 'c'], 3);
    let into_iter = vec.into_iter();
    let debug = format!("{into_iter:?}");
    assert_eq!(debug, "IntoIter(['a', 'b', 'c'])");
}
