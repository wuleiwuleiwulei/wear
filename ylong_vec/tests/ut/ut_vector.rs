use super::*;
#[cfg(target_pointer_width = "32")]
use crate::raw_buffer::alloc_check;
use std::alloc::Global;

fn create_vec_from_arr<T, const N: usize>(arr: [T; N], cap: usize) -> FixedCapVec<T, Global> {
    let mut v = FixedCapVec::new(cap, Global).unwrap();
    for e in arr {
        let _ = v.push(e);
    }
    v
}

/// UT test cases for vec `push`.
///
/// # Brief
/// 1. push vec, within capacaity, success
/// 2. push vec, over capacaity, fail
/// 3. push zst, over capacaity, success
#[test]
fn vec_push() {
    let mut v = FixedCapVec::<u32, Global>::new(0, Global).unwrap();

    assert!(v.push(1).is_err());

    let mut v = FixedCapVec::<u32, Global>::new(1, Global).unwrap();
    assert!(v.push(1).is_ok());
    assert!(v.push(2).is_err());
    assert_eq!(v.pop(), Some(1));

    let mut v = create_vec_from_arr([0; 1000], 1000);
    assert!(v.push(1).is_err());

    let mut v = FixedCapVec::<(), Global>::new(1, Global).unwrap();
    assert!(v.push(()).is_ok());
    assert!(v.push(()).is_ok());
    assert_eq!(v.pop(), Some(()));
}

/// UT test cases for vec `pop`.
///
/// # Brief
/// 1. create vec
/// 2. pop for Some and None
#[test]
fn vec_pop() {
    let mut v = FixedCapVec::<u32, Global>::new(1, Global).unwrap();
    assert!(v.push(1).is_ok());
    assert!(v.push(2).is_err());
    assert_eq!(v.pop(), Some(1));
    assert_eq!(v.pop(), None);
}

/// UT test cases for vec `is_full`.
///
/// # Brief
/// 1. create a vec empty
/// 2. push element
/// 3. check whether is full
#[test]
fn vec_is_full() {
    let mut v = FixedCapVec::<u32, Global>::new(2, Global).unwrap();

    let _ = v.push(1);
    assert!(!v.is_full());
    let _ = v.push(2);
    assert!(v.is_full());
}

/// UT test cases for vec `is_full`.
///
/// # Brief
/// 1. create a vec of zst
/// 2. push element
/// 3. `is_full` always return false
#[test]
fn vec_is_full_zst() {
    let mut v = FixedCapVec::<(), Global>::new(1, Global).unwrap();

    assert!(!v.is_full());
    let _ = v.push(());
    assert!(!v.is_full());
}

/// UT test cases for vec `is_empty`.
///
/// # Brief
/// 1. create a vec empty
/// 2. push element
/// 3. check whether is empty
#[test]
fn vec_is_empty() {
    let mut v = FixedCapVec::<(), Global>::new(2, Global).unwrap();

    assert!(v.is_empty());
    let _ = v.push(());
    assert!(!v.is_empty());
}

/// UT test cases for vec `as_mut_slice`.
///
/// # Brief
/// 1. create a vec
/// 2. get as_mut_slice
/// 3. modify element
#[test]
fn vec_as_mut_slice() {
    let mut v = create_vec_from_arr([1, 2, 3], 3);

    let s = v.as_mut_slice();
    s[0] = 0;
    assert_eq!(v.as_slice(), [0, 2, 3]);
}

/// UT test cases for vec `truncate`.
///
/// # Brief
/// 1. create a vec.
/// 2. truncate vec.
#[test]
fn vec_truncate() {
    let mut v = create_vec_from_arr([1, 2, 3, 4, 5], 5);
    assert_eq!(v.as_ref(), [1, 2, 3, 4, 5]);

    v.truncate(6);
    assert_eq!(v.as_ref(), [1, 2, 3, 4, 5]);
    v.truncate(5);
    assert_eq!(v.as_ref(), [1, 2, 3, 4, 5]);
    v.truncate(4);
    assert_eq!(v.as_ref(), [1, 2, 3, 4]);
    v.truncate(0);
    assert_eq!(v.as_ref(), []);
}

/// UT test cases for vec `remove`.
///
/// # Brief
/// 1. remove, valid index, succuess.
/// 2. remove, invalid index, fail.
#[test]
fn vec_remove() {
    let mut v1 = create_vec_from_arr([1, 2, 3], 3);
    assert_eq!(v1.remove(0).unwrap(), 1);
    assert_eq!(v1.len(), 2);

    assert!(v1.remove(2).is_err());
}

/// UT test cases for vec `extend_from_slice`.
///
/// # Brief
/// 1. extend_from_slice, valid len, succuess.
/// 2. extend_from_slice, invalid len, fail.
#[test]
fn vec_extend_from_slice() {
    let mut v1 = create_vec_from_arr([1, 2, 3], 6);

    assert!(v1.extend_from_slice(&[4, 5, 6]).is_ok());
    assert_eq!(v1.len(), 6);

    assert!(v1.extend_from_slice(&[]).is_ok());
    assert!(v1.extend_from_slice(&[7]).is_err());

    assert_eq!(v1.len(), 6);
}

/// UT test cases for vec `clear`.
///
/// # Brief
/// 1. clear, drop each element.
#[test]
fn vec_clear() {
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
    let mut v1 = FixedCapVec::<DropCounter, Global>::new(10, Global).unwrap();

    let _ = v1.push(d1);
    let _ = v1.push(d2);
    let _ = v1.push(d3);

    v1.clear();
    unsafe {
        assert_eq!(TEST_FLAG, 3);
    }
}

/// UT test cases for vec `as_ref`.
///
/// # Brief
/// 1. craete a vec.
/// 2. get ref of vec
#[test]
fn vec_as_ref() {
    let v1 = create_vec_from_arr([1, 2, 3], 6);

    let r: &FixedCapVec<i32, _> = v1.as_ref();
    assert_eq!(r.as_ptr(), v1.as_ptr());
}

/// UT test cases for vec `as_mut`.
///
/// # Brief
/// 1. craete a vec.
/// 2. get mut ref of vec
#[test]
fn vec_as_mut() {
    let mut v1 = create_vec_from_arr([1, 2, 3], 6);

    let r: &mut FixedCapVec<i32, _> = v1.as_mut();
    assert_eq!(r.as_ptr(), v1.as_ptr());
}

/// UT test cases for vec `debug`.
///
/// # Brief
/// 1. create vec.
/// 2. print vec debug message.
#[test]
fn test_debug_fmt() {
    let vec1 = FixedCapVec::<isize, Global>::new(0, Global).unwrap();
    assert_eq!("[]", format!("{:?}", vec1));

    let mut vec2 = FixedCapVec::<i32, Global>::new(2, Global).unwrap();
    let _ = vec2.push(1);
    let _ = vec2.push(2);
    assert_eq!("[1, 2]", format!("{:?}", vec2));

    let slice: &[isize] = &[3, 4];
    assert_eq!("[3, 4]", format!("{slice:?}"));
}

/// UT test cases for vec `safe_clone`.
///
/// # Brief
/// 1. clone vec
/// 2. check it will call clone of element
#[test]
fn vec_safe_clone() {
    #[derive(Clone)]
    struct Temp {
        i: u8,
    }

    let temp = Temp { i: 1 };

    let mut v1 = FixedCapVec::<Temp, Global>::new(10, Global).unwrap();
    assert!(v1.push(temp).is_ok());

    let v2 = v1.safe_clone().unwrap();
    assert_eq!(v2[0].i, 1);
}

/// SDV test cases for vec `safe_clone_from`.
///
/// # Brief
/// 1. clone_from vec
/// 2. check it will call clone_from of element
#[test]
fn test_safe_clone_from() {
    let mut v: FixedCapVec<_, Global> = create_vec_from_arr([], 3);
    let three = create_vec_from_arr([Box::new(1), Box::new(2), Box::new(3)], 3);
    let two = create_vec_from_arr([Box::new(4), Box::new(5)], 3);
    // zero, long
    let _ = v.safe_clone_from(&three);
    assert_eq!(v.as_slice(), three.as_slice());

    // equal
    let _ = v.safe_clone_from(&three);
    assert_eq!(v.as_slice(), three.as_slice());

    // long, short
    let _ = v.safe_clone_from(&two);
    assert_eq!(v.as_slice(), two.as_slice());

    // short, long
    let _ = v.safe_clone_from(&three);
    assert_eq!(v.as_slice(), three.as_slice());

    let four = create_vec_from_arr([Box::new(1), Box::new(2), Box::new(3), Box::new(4)], 4);
    let r = v.safe_clone_from(&four);
    assert!(r.is_err());
    assert_eq!(v.as_slice(), three.as_slice());
}

/// UT test cases for vec `alloc_check`.
///
/// # Brief
/// 1. 3GB allocation in 32bits
/// 2. check failed
#[test]
#[cfg(target_pointer_width = "32")]
fn test_alloc_check() {
    let r = alloc_check(isize::MAX as usize + 1);
    assert!(r.is_err());
}
