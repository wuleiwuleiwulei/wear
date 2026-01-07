//! Adapt tests from std and hashbrown to make sure functions of `FixedCapMap` follow the same logic
//! with `HashMap` in Rust std.

use crate::FixedCapMap;
use std::alloc::System;

/// UT test case for empty iter of FixedCapMap.
///
/// # Brief
/// 1. Create an empty map and get iterator from it. All next call on this iterator should return
///    None.
#[test]
fn ut_empty_iter() {
    let mut m: FixedCapMap<i32, bool, System> = FixedCapMap::new(3, System).unwrap();
    assert_eq!(m.len(), 0);
    assert!(m.is_empty());
    assert_eq!(m.iter().next(), None);
    assert_eq!(m.iter_mut().next(), None);
    assert_eq!(m.keys().next(), None);
    assert_eq!(m.values().next(), None);
    assert_eq!(m.values_mut().next(), None);
    assert_eq!(m.into_iter().next(), None);
}

/// UT test case for iterator of keys of FixedCapMap.
///
/// # Brief
/// 1. Create a map and insert items into it.
/// 2. Iterate through the map's keys. This should work properly and the iterator should
///    truly generates key values of the map.
#[test]
fn ut_keys() {
    let mut m = FixedCapMap::new(3, System).unwrap();
    assert!(m.insert(1, 'a').unwrap().is_none());
    assert!(m.insert(2, 'b').unwrap().is_none());
    assert!(m.insert(3, 'c').unwrap().is_none());

    let keys: Vec<_> = m.keys().cloned().collect();
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&1));
    assert!(keys.contains(&2));
    assert!(keys.contains(&3));
}

/// UT test case for iterator of values of FixedCapMap.
///
/// # Brief
/// 1. Create a map and insert items into it.
/// 2. Iterate through the map's values. This should work properly and the iterator should
///    truly generates values of the map.
#[test]
fn ut_values() {
    let mut m = FixedCapMap::new(3, System).unwrap();
    assert!(m.insert(1, 'a').unwrap().is_none());
    assert!(m.insert(2, 'b').unwrap().is_none());
    assert!(m.insert(3, 'c').unwrap().is_none());

    let values: Vec<_> = m.values().cloned().collect();
    assert_eq!(values.len(), 3);
    assert!(values.contains(&'a'));
    assert!(values.contains(&'b'));
    assert!(values.contains(&'c'));
}

/// UT test case for mut iterator of values of FixedCapMap.
///
/// # Brief
/// 1. Create a map and insert items into it.
/// 2. Mutably iterate through the map's values. This should work properly and the iterator should
///    truly generates values of the map.
/// 3. Change the raw value of the values stores in the map. Get them again and check their value.
#[test]
fn ut_values_mut() {
    let mut m = FixedCapMap::new(3, System).unwrap();
    assert!(m.insert(1, 1).unwrap().is_none());
    assert!(m.insert(2, 2).unwrap().is_none());
    assert!(m.insert(3, 3).unwrap().is_none());

    for value in m.values_mut() {
        *value *= 2
    }
    let values: Vec<_> = m.values().cloned().collect();
    assert_eq!(values.len(), 3);
    assert!(values.contains(&2));
    assert!(values.contains(&4));
    assert!(values.contains(&6));
}
