//! Adapt tests from std and hashbrown to make sure functions of `FixedCapMap` follow the same logic
//! with `HashMap` in Rust std.

use crate::FixedCapMap;
use std::alloc::System;
use ylong_stdx_common::{ContainerError, SafeClone};

/// UT test case for zero cap of FixedCapMap.
///
/// # Brief
/// 1. Create zero cap map and assert its bucket num is zero.
/// 2. Create non-zero cap map and assert that items can be inserted, and buckets are allocated as
///    expected.
#[test]
fn ut_zero_capacities() {
    type HM = FixedCapMap<i32, i32, System>;

    let m = HM::new(0, System).unwrap();
    assert_eq!(m.bucket_num(), 0);

    let mut m = HM::new(5, System).unwrap();

    assert_eq!(m.bucket_num(), 7);
    m.insert(1, 1).unwrap();
    assert_eq!(m.bucket_num(), 7);

    m.insert(2, 2).unwrap();
    assert_eq!(m.bucket_num(), 7);

    m.remove(&1_i32);
    m.remove(&2);
    assert_eq!(m.bucket_num(), 7);
}

/// UT test case for contain function of FixedCapMap.
///
/// # Brief
/// 1. Create map and insert item into it.
/// 2. Assert calling contains_key with a key in it is true.
/// 3. Assert calling contains_key with a key not in it is false.
#[test]
fn ut_contains() {
    let mut m = FixedCapMap::new(1, System).unwrap();

    assert!(m.insert(1, 1).unwrap().is_none());
    assert!(m.contains_key(&1));
    assert!(!m.contains_key(&0));
}

/// UT test case for insert function of FixedCapMap.
///
/// # Brief
/// 1. Create map and insert items into it. This should succeed.
/// 2. Assert length is corresponding to the number of items inserted.
/// 3. Assert that when the number of items inserted exceeds the capacity passed in, the insertion
///    should fail. Besides, the capacity of the map will not grow.
#[test]
fn ut_insert() {
    let mut m = FixedCapMap::new(3, System).unwrap();
    assert_eq!(m.len(), 0);
    assert_eq!(m.bucket_num(), 3);
    assert!(m.insert(1, 2).unwrap().is_none());
    assert_eq!(m.len(), 1);
    assert!(m.insert(2, 4).unwrap().is_none());
    assert_eq!(m.len(), 2);
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert_eq!(*m.get(&2).unwrap(), 4);

    assert!(m.insert(3, 4).unwrap().is_none());
    assert_eq!(m.len(), 3);
    assert_eq!(m.bucket_num(), 3);
    assert!(m.insert(4, 4).is_err());
    assert_eq!(m.bucket_num(), 3);
}

/// UT test case for clone of FixedCapMap.
///
/// # Brief
/// 1. Create map and insert items into it. This should succeed.
/// 2. Assert cloning should succeed and the new map should be same of the old one.
#[test]
fn ut_clone() {
    let mut m = FixedCapMap::new(3, System).unwrap();
    assert_eq!(m.len(), 0);
    assert_eq!(m.bucket_num(), 3);
    assert!(m.insert(1, 2).unwrap().is_none());
    assert_eq!(m.len(), 1);
    assert!(m.insert(2, 4).unwrap().is_none());
    assert_eq!(m.len(), 2);
    assert_eq!(m.bucket_num(), 3);
    let m2 = m.safe_clone().unwrap();
    assert_eq!(*m2.get(&1).unwrap(), 2);
    assert_eq!(*m2.get(&2).unwrap(), 4);
    assert_eq!(m2.len(), 2);
    assert_eq!(m2.bucket_num(), 3);
}

/// UT test case for clone_from of FixedCapMap.
///
/// # Brief
/// 1. Create map and insert items into it. This should succeed.
/// 2. Assert cloning from a source map should succeed and the new map should be same of the old
///    one.
#[test]
fn ut_clone_from() {
    let mut map1 = FixedCapMap::new(3, System).unwrap();
    let mut map2 = FixedCapMap::new(3, System).unwrap();
    assert_eq!(map1.len(), 0);
    assert!(map1.insert(1, 2).unwrap().is_none());
    assert_eq!(map1.len(), 1);
    assert!(map1.insert(2, 4).unwrap().is_none());
    assert_eq!(map1.len(), 2);
    map2.safe_clone_from(&map1).unwrap();
    assert_eq!(*map2.get(&1).unwrap(), 2);
    assert_eq!(*map2.get(&2).unwrap(), 4);
    assert_eq!(map2.len(), 2);
}

/// UT test case for find mut of FixedCapMap.
///
/// # Brief
/// 1. Create a map and insert items into it. This should succeed.
/// 2. Get_mut and using the mut reference to change value should succeed, and when getting the
///    value again, it should be the new assigned one.
#[test]
fn ut_find_mut() {
    let mut m = FixedCapMap::new(3, System).unwrap();
    assert!(m.insert(1, 12).unwrap().is_none());
    assert!(m.insert(2, 8).unwrap().is_none());
    assert!(m.insert(5, 14).unwrap().is_none());
    let new = 100;
    match m.get_mut(&5) {
        None => panic!(),
        Some(x) => *x = new,
    }
    assert_eq!(m.get(&5), Some(&new));
}

/// UT test case for is_empty of FixedCapMap.
///
/// # Brief
/// 1. Create a map and insert items into it. Assert is_empty is false.
/// 2. Remove the item and assert is_empty is true.
#[test]
fn ut_is_empty() {
    let mut m = FixedCapMap::new(3, System).unwrap();
    assert!(m.insert(1, 2).unwrap().is_none());
    assert!(!m.is_empty());
    assert!(m.remove(&1).is_some());
    assert!(m.is_empty());
}

/// UT test case for remove of FixedCapMap.
///
/// # Brief
/// 1. Create a map and insert items into it.
/// 2. Remove the item. For the first time, remove should return Some with the value. For the second
///    time, it should return None.
#[test]
fn ut_remove() {
    let mut m = FixedCapMap::new(3, System).unwrap();
    m.insert(1, 2).unwrap();
    assert_eq!(m.remove(&1), Some(2));
    assert_eq!(m.remove(&1), None);
}

/// UT test case for get of FixedCapMap.
///
/// # Brief
/// 1. Create a map and assert that finding something not existing in the map returns None.
/// 2. Insert items into it and assert that finding that returns the value.
#[test]
fn ut_find() {
    let mut m = FixedCapMap::new(3, System).unwrap();
    assert!(m.get(&1).is_none());
    m.insert(1, 2).unwrap();
    match m.get(&1) {
        None => panic!(),
        Some(v) => assert_eq!(*v, 2),
    }
}

/// UT test case for eq of FixedCapMap.
///
/// # Brief
/// 1. Create a map, insert items into it, and create one another different.
/// 2. Assert that they are not equal.
/// 3. Insert the same items into the second one. Assert that they should be equal now.
#[test]
fn ut_eq() {
    let mut m1 = FixedCapMap::new(3, System).unwrap();
    m1.insert(1, 2).unwrap();
    m1.insert(2, 3).unwrap();
    m1.insert(3, 4).unwrap();

    let mut m2 = FixedCapMap::new(3, System).unwrap();
    m2.insert(1, 2).unwrap();
    m2.insert(2, 3).unwrap();

    assert_ne!(m1, m2);

    m2.insert(3, 4).unwrap();

    assert_eq!(m1, m2);
}

/// UT test case for print of FixedCapMap.
///
/// # Brief
/// 1. Create a map, and insert items into it.
/// 2. Assert that printing the map is the same as expected.
#[test]
fn ut_show() {
    let mut map = FixedCapMap::new(3, System).unwrap();
    let empty: FixedCapMap<i32, i32, System> = FixedCapMap::new(3, System).unwrap();

    map.insert(1, 2).unwrap();
    map.insert(3, 4).unwrap();

    let map_str = format!("{map:?}");
    let ans1 = "TryMap: {{1: 2, 3: 4}, Allocator: System, Limit: 3}";
    let ans2 = "TryMap: {{3: 4, 1: 2}, Allocator: System, Limit: 3}";
    assert!(map_str == ans1 || map_str == ans2);
    assert_eq!(
        format!("{empty:?}"),
        "TryMap: {{}, Allocator: System, Limit: 3}"
    );
}

/// UT test case for get_mut_default of FixedCapMap.
///
/// # Brief
/// 1. Create a map, insert items into it by get_mut_default method.
/// 2. Assert that the new value can be assigned through the mut reference.
#[test]
fn ut_get_mut_default() {
    let mut a = FixedCapMap::new(3, System).unwrap();
    let key = "hello there";
    let value = "value goes here";

    // Create a new one
    assert!(a.is_empty());
    let get_mut = a.get_mut_default(&key).unwrap();
    *get_mut = value;
    assert_eq!(a.len(), 1);
    assert_eq!(*a.get(&key).unwrap(), value);

    // Get it again
    let new_value = "a new value";
    let get_mut = a.get_mut_default(&key).unwrap();
    *get_mut = new_value;
    assert_eq!(a.len(), 1);
    assert_eq!(*a.get(&key).unwrap(), new_value);

    // Insert more
    assert!(a.insert("key2", "value2").unwrap().is_none());
    assert!(a.insert("key3", "value3").unwrap().is_none());

    // .. and more, should return error.
    let error_key = "err key";
    let err = a.get_mut_default(&error_key).err().unwrap();
    match err {
        ContainerError::Full => {}
        _ => panic!(),
    }
}
