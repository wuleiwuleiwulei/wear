//! Adapt tests from std and hashbrown to make sure functions of `FixedCapMap` follow the same logic
//! with `HashMap` in Rust std.

#![feature(allocator_api)]

use rand::Rng;
use std::alloc::{AllocError, Allocator, Layout, System};
use std::convert::TryInto;
use std::ptr::NonNull;
use ylong_map::FixedCapMap;
use ylong_stdx_common::{ContainerError, SafeClone};

/// SDV test case for empty remove of FixedCapMap.
///
/// # Brief
/// 1. Create an empty map and remove item from it. This should succeed but return None.
#[test]
fn sdv_empty_remove() {
    let mut m: FixedCapMap<i32, bool, System> = FixedCapMap::new(3, System).unwrap();
    assert_eq!(m.remove(&0), None);
}

/// SDV test case for lots of insertion and stability of FixedCapMap.
///
/// # Brief
/// 1. Create a map and repetitively insert, get, and remove items into and from it. All operations
///    should succeed except the case that the number of items inserted exceeds the capacity, and
///    more items are trying to be inserted.
#[test]
fn sdv_lots_of_insertions() {
    let max = 10 * 1001_usize;
    let mut m = FixedCapMap::new(max, System).unwrap();
    let cap = m.bucket_num();

    // Try this a few times to make sure we never screw up the FixedCapMap's
    // internal state.
    let loops = if cfg!(miri) { 2 } else { 10 };
    for _ in 0..loops {
        assert!(m.is_empty());

        let count = if cfg!(miri) { 101 } else { 1001 };

        for i in 1..count {
            assert!(m.insert(i, i).unwrap().is_none());
            assert_eq!(m.bucket_num(), cap);

            for j in 1..=i {
                let r = m.get(&j);
                assert_eq!(r, Some(&j));
            }

            for j in i + 1..count {
                let r = m.get(&j);
                assert_eq!(r, None);
            }
        }

        for i in count..(2 * count) {
            assert!(!m.contains_key(&i));
        }

        // remove forwards
        for i in 1..count {
            assert!(m.remove(&i).is_some());

            for j in 1..=i {
                assert!(!m.contains_key(&j));
            }

            for j in i + 1..count {
                assert!(m.contains_key(&j));
            }
        }

        for i in 1..count {
            assert!(!m.contains_key(&i));
        }

        for i in 1..count {
            assert!(m.insert(i, i).unwrap().is_none());
            assert_eq!(m.bucket_num(), cap);
        }

        // remove backwards
        for i in (1..count).rev() {
            assert!(m.remove(&i).is_some());

            for j in i..count {
                assert!(!m.contains_key(&j));
            }

            for j in 1..i {
                assert!(m.contains_key(&j));
            }
        }
    }
}

/// SDV test case for update of FixedCapMap.
///
/// # Brief
/// 1. Create a map and insert items into it. This should succeed.
/// 2. Insert with the same key. This should update the value.
#[test]
fn sdv_insert_overwrite() {
    let mut m = FixedCapMap::new(2, System).unwrap();
    assert!(m.insert(1, 2).unwrap().is_none());
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert!(m.insert(1, 3).unwrap().is_some());
    assert_eq!(*m.get(&1).unwrap(), 3);
    *m.get_mut(&1).unwrap() = 4;
    assert_eq!(*m.get(&1).unwrap(), 4);
}

/// SDV test case for insert conflicts of FixedCapMap.
///
/// # Brief
/// 1. Create a map and insert items into it. These items' hash indexes are the same and thus
///    conflict. This should succeed.
/// 2. Assert that getting values work properly.
#[test]
fn sdv_insert_conflicts() {
    let mut m = FixedCapMap::new(3, System).unwrap();
    assert!(m.insert(1, 2).unwrap().is_none());
    *m.get_mut_default(&5).unwrap() = 3;
    assert!(m.insert(9, 4).unwrap().is_none());
    assert_eq!(*m.get(&9).unwrap(), 4);
    assert_eq!(*m.get(&5).unwrap(), 3);
    assert_eq!(*m.get(&1).unwrap(), 2);
}

/// SDV test case for remove conflicts of FixedCapMap.
///
/// # Brief
/// 1. Create a map and insert items into it. These items' hash indexes are the same and thus
///    conflict. This should succeed.
/// 2. Assert that removing values work properly.
#[test]
fn sdv_conflict_remove() {
    let mut m = FixedCapMap::new(3, System).unwrap();
    assert!(m.insert(1, 2).unwrap().is_none());
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert!(m.insert(5, 3).unwrap().is_none());
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert_eq!(*m.get(&5).unwrap(), 3);
    assert!(m.insert(9, 4).unwrap().is_none());
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert_eq!(*m.get(&5).unwrap(), 3);
    assert_eq!(*m.get(&9).unwrap(), 4);
    assert!(m.remove(&1).is_some());
    assert_eq!(*m.get(&9).unwrap(), 4);
    assert_eq!(*m.get(&5).unwrap(), 3);
}

/// SDV test case for clone of FixedCapMap.
///
/// # Brief
/// 1. Create map and insert items into it. This should succeed.
/// 2. Assert cloning should succeed and the new map should be same of the old one.
/// 3. Inserting items into the old one should not influence the new one.
#[test]
fn sdv_clone() {
    let mut m = FixedCapMap::new(3, System).unwrap();
    assert!(m.insert(1, 2).unwrap().is_none());
    assert!(m.insert(2, 4).unwrap().is_none());
    let mut m2 = m.safe_clone().unwrap();
    assert_eq!(m, m2);
    assert!(m2.insert(3, 6).unwrap().is_none());
    assert_ne!(m, m2);
    m2.safe_clone_from(&m).unwrap();
    assert_eq!(m, m2);
    let mut m3 = FixedCapMap::new(4, System).unwrap();
    assert!(m3.insert(1, 2).unwrap().is_none());
    assert!(m3.insert(2, 4).unwrap().is_none());
    m2.safe_clone_from(&m3).unwrap();
    assert_eq!(m2, m3);
    assert!(m3.insert(3, 6).unwrap().is_none());
    assert!(m3.insert(4, 8).unwrap().is_none());
    assert!(m3.is_full());
    m2.safe_clone_from(&m3).unwrap();
    assert_eq!(m2, m3);
}

/// SDV test case for iterator of FixedCapMap.
///
/// # Brief
/// 1. Create a map and insert items into it.
/// 2. Iterate through the map. This should work properly.
#[test]
fn sdv_iterate() {
    let mut m = FixedCapMap::new(32, System).unwrap();
    let cap = m.bucket_num();
    for i in 0..32 {
        assert!(m.insert(i, i * 2).unwrap().is_none());
    }
    assert!(m.insert(33, 33 * 2).is_err());
    assert_eq!(m.len(), 32);
    assert_eq!(m.bucket_num(), cap);

    let mut observed: u32 = 0;

    for (k, v) in &m {
        assert_eq!(*v, *k * 2);
        observed |= 1 << *k;
    }
    assert_eq!(observed, 0xFFFF_FFFF);

    let range = 0..32;
    for k in m.keys() {
        assert!(range.contains(k));
    }

    let range: Vec<i32> = range.map(|x| x * 2).collect();
    for v in m.values() {
        assert!(range.contains(v));
    }

    for v in m.values_mut() {
        *v *= 2;
    }

    for (k, v) in m {
        assert_eq!(v, k * 4);
    }
}

/// SDV test case for iterator length of FixedCapMap.
///
/// # Brief
/// 1. Create a map and insert items into it.
/// 2. Assert that the iterator generates the same number of items in the map.
#[test]
fn sdv_iter_len() {
    let mut map = FixedCapMap::new(6, System).unwrap();
    map.insert(1, 1).unwrap();
    map.insert(2, 2).unwrap();
    map.insert(3, 3).unwrap();
    map.insert(4, 4).unwrap();
    map.insert(5, 5).unwrap();
    map.insert(6, 6).unwrap();

    let mut iter = map.iter();

    for _ in iter.by_ref().take(3) {}

    assert!(iter.next().is_some());
    assert!(iter.next().is_some());
    assert!(iter.next().is_some());
    assert!(iter.next().is_none());
}

/// SDV test case for mut iterator length of FixedCapMap.
///
/// # Brief
/// 1. Create a map and insert items into it.
/// 2. Assert that the mutable iterator generates the same number of items in the map.
#[test]
fn sdv_iter_mut_len() {
    let mut map = FixedCapMap::new(6, System).unwrap();
    map.insert(1, 1).unwrap();
    map.insert(2, 2).unwrap();
    map.insert(3, 3).unwrap();
    map.insert(4, 4).unwrap();
    map.insert(5, 5).unwrap();
    map.insert(6, 6).unwrap();

    let mut iter = map.iter_mut();

    for _ in iter.by_ref().take(3) {}

    assert!(iter.next().is_some());
    assert!(iter.next().is_some());
    assert!(iter.next().is_some());
    assert!(iter.next().is_none());
}

fn sdv_rng() -> rand_xorshift::XorShiftRng {
    use core::hash::{BuildHasher, Hash, Hasher};
    let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
    std::panic::Location::caller().hash(&mut hasher);

    let hc64 = hasher.finish();
    let mut seed_vec = vec![];
    for i in hc64.to_le_bytes() {
        seed_vec.push(i);
    }
    for i in 0..8 {
        seed_vec.push(i);
    }
    let seed: [u8; 16] = seed_vec.as_slice().try_into().unwrap();
    rand::SeedableRng::from_seed(seed)
}

/// SDV test case for stability with randomization of FixedCapMap.
///
/// # Brief
/// 1. Create a map, insert items with random values into it.
/// 2. Assert that removing items exist in the map returns Some, and otherwise None.
#[test]
fn sdv_entry_take_doesnt_corrupt() {
    // rand
    #![allow(deprecated)]
    // Test for #19292
    fn check(m: &FixedCapMap<i32, (), System>) {
        for k in m.keys() {
            assert!(m.contains_key(k), "{} is in keys() but not in the map?", k);
        }
    }

    let mut m = FixedCapMap::new(50, System).unwrap();
    let mut rng = sdv_rng();

    // Populate the map with some items.
    for _ in 0..50 {
        let x = rng.gen_range(-10..10);
        m.insert(x, ()).unwrap();
    }

    for _ in 0..1000 {
        let x = rng.gen_range(-10..10);
        let exist = m.get(&x).is_some();
        if exist {
            m.remove(&x);
        }

        check(&m);
    }
}

/// SDV test case for capacity mechanism of FixedCapMap.
///
/// # Brief
/// 1. Create a map, insert items into it.
/// 2. Assert that when length exceeds the capacity, insertion will return error.
#[test]
fn sdv_capacity_not_less_than_len() {
    let mut a = FixedCapMap::new(116, System).unwrap();
    let mut item = 0;

    for _ in 0..116 {
        a.insert(item, 0).unwrap();
        item += 1;
    }

    // Insert at capacity should cause error.
    assert!(a.insert(item, 0).is_err());
}

/// SDV test case for OOM case of FixedCapMap.
///
/// # Brief
/// 1. Create a map with a too big size. This should return Err instead of arousing panic.
#[test]
fn sdv_try_reserve() {
    const MAX_USIZE: usize = usize::MAX;

    let ret: Result<FixedCapMap<u8, u8, System>, ContainerError> =
        FixedCapMap::new(MAX_USIZE, System);
    match ret.err().unwrap() {
        ContainerError::CapacityOverflow => {}
        _ => panic!(),
    }
}

#[derive(Clone)]
struct AllocatorAlwaysFail;

unsafe impl Allocator for AllocatorAlwaysFail {
    fn allocate(&self, _layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Err(AllocError)
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
}

/// SDV test case for map with allocator that always returns error.
///
/// # Brief
/// 1. Create zero cap map with AllocatorAlwaysFail. This should be fine but the map is always
///    empty.
/// 2. Create non-zero cap map with AllocatorAlwaysFail. This should fail.
#[test]
fn sdv_new_with_alloc_always_fail() {
    type HM = FixedCapMap<i32, i32, AllocatorAlwaysFail>;

    let m = HM::new(0, AllocatorAlwaysFail).unwrap();
    assert_eq!(m.bucket_num(), 0);

    let ret = HM::new(5, AllocatorAlwaysFail).err().unwrap();
    match ret {
        ContainerError::AllocFailure(_) => {}
        _ => panic!(),
    }
}
