use core::borrow::Borrow;
use core::hash::Hash;
use std::alloc::Allocator;
use std::collections::hash_map::{RandomState, RawTable};
use std::fmt::Debug;
use std::hash::{BuildHasher, Hasher};
use ylong_stdx_common::{ContainerError, SafeClone};

use crate::{IntoIter, Iter, IterMut, Keys, Values, ValuesMut};

#[cfg(test)]
mod ut_test;

/// FixedCapMap that provides functions the same as std's HashMap, but won't panic when OOM
/// and grow its capacity.
///
/// Core realization reuses RawTable, the raw and core implementation of HashMap, in hashbrown
/// 0.14.3 (from Rust 1.76.0). Here, adapt the methods and make them easier for users to use.
///
/// FixedCapMap has to require `Clone` trait bound on allocator, because RawTable in hashbrown
/// requires that.
pub struct FixedCapMap<K, V, A: Allocator + Clone> {
    table: RawTable<(K, V), A>,
    hasher: RandomState,
    limit: usize,
}

impl<K, V, A: Allocator + Clone> FixedCapMap<K, V, A> {
    /// Initializes a new instance of FixedCapMap. Users need to clarify capacity and allocator of
    /// the new instance. The allocator must implement Allocator trait in the Rust standard library,
    /// and the capacity won't grow when new items are inserted into the instance. Return error if
    /// anything fails when creating the instance, e.g. failure to allocate the memory specified.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn main() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn new(capacity: usize, allocator: A) -> Result<FixedCapMap<K, V, A>, ContainerError> {
        let raw = RawTable::<(K, V), A>::try_with_capacity_in(capacity, allocator)?;
        Ok(FixedCapMap {
            table: raw,
            hasher: RandomState::new(),
            limit: capacity,
        })
    }

    /// Clears all the contents in the map. Objects inside will be released.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn main() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     // Insert some items
    ///     map.insert(3, 6)?;
    ///
    ///     // Clear
    ///     map.clear();
    ///     assert!(map.is_empty());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn clear(&mut self) {
        self.table.clear();
    }

    /// Gets the length of the map.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn main() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     // Insert some items
    ///     map.insert(3, 6)?;
    ///
    ///     let len = map.len();
    ///     assert_eq!(len, 1);
    /// #   Ok(())
    /// # }
    /// ```
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// Gets the bucket number of the map. Bucket number is capacity of RawTable in hashbrown crate,
    /// and will be larger than the limit size set by the user.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn new_map() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     assert!(map.bucket_num() > map.capacity());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn bucket_num(&self) -> usize {
        self.table.capacity()
    }

    /// Gets the capacity of the map. Capacity is the limit size set by the user.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn new_map() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     assert_eq!(map.capacity(), size);
    ///
    ///     // Insert 5 items
    ///     map.insert(1, 1)?;
    ///     map.insert(2, 2)?;
    ///     map.insert(3, 3)?;
    ///     map.insert(4, 4)?;
    ///     map.insert(5, 5)?;
    ///
    ///     // Can't insert more because the capacity is 5
    ///     assert!(map.insert(6, 6).is_err());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn capacity(&self) -> usize {
        self.limit
    }

    /// Gets the size of the table allocated, mainly as a hint of how much memory has been consumed
    /// by the inner table. The size info is from hashbrown::RawTable::table_size_allocated. The
    /// total size of the whole instance will be bigger as besides of the table, the instance also
    /// stores hasher and limit size number.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn new_map() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     let table_size = map.table_size_allocated();
    /// #   Ok(())
    /// # }
    /// ```
    pub fn table_size_allocated(&self) -> usize {
        self.table.allocation_info().1.size()
    }

    /// Checks whether the hashmap is empty.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn main() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     assert!(map.is_empty());
    ///     // Insert some items
    ///     map.insert(3, 6)?;
    ///     assert!(!map.is_empty());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    /// Checks whether the map is full.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn new_map() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     assert!(!map.is_full());
    ///
    ///     // Insert 5 items
    ///     map.insert(1, 1)?;
    ///     map.insert(2, 2)?;
    ///     map.insert(3, 3)?;
    ///     map.insert(4, 4)?;
    ///     map.insert(5, 5)?;
    ///
    ///     // Can't insert more because the capacity is 5
    ///     assert!(map.insert(6, 6).is_err());
    ///     assert!(map.is_full());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn is_full(&self) -> bool {
        self.table.len() >= self.limit
    }

    /// Gets iterator of the map.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn new_map() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     map.insert(1, 1)?;
    ///     let mut iter = map.iter();
    ///     if let Some(pair) = iter.next() {
    ///         assert_eq!(*pair.0, 1);
    ///         assert_eq!(*pair.1, 1);
    ///     }
    /// #   else {
    /// #       panic!("Iter fails to get keypair");
    /// #   }
    /// #   Ok(())
    /// # }
    /// ```
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            // SAFETY:
            // Safe because lifetime mechanism will restrict that `RawTable` outlives the `RawIter`
            iter: unsafe { self.table.iter() },
            phantom: Default::default(),
        }
    }

    /// Gets mut iterator of the map.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn new_map() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     map.insert(1, 1)?;
    ///     let mut iter = map.iter_mut();
    ///     if let Some(pair) = iter.next() {
    ///         assert_eq!(*pair.0, 1);
    ///         assert_eq!(*pair.1, 1);
    ///         *pair.1 = 2;
    ///     }
    /// #   else {
    /// #       panic!("Iter fails to get keypair");
    /// #   }
    ///     if let Some(val) = map.get(&1) {
    ///         assert_eq!(*val, 2);
    ///     }
    /// #   else {
    /// #       panic!("Iter fails to get keypair");
    /// #   }
    /// #   Ok(())
    /// # }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut {
            // SAFETY:
            // Safe because lifetime mechanism will restrict that `RawTable` outlives the `RawIter`
            iter: unsafe { self.table.iter() },
            phantom: Default::default(),
        }
    }

    /// Gets an iterator of keys in the map.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn new_map() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     map.insert(1, 0)?;
    ///     let mut keys = map.keys();
    ///     if let Some(key) = keys.next() {
    ///         assert_eq!(*key, 1);
    ///     }
    /// #   else {
    /// #       panic!("Iter fails to get keypair");
    /// #   }
    /// #   Ok(())
    /// # }
    /// ```
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys { iter: self.iter() }
    }

    /// Gets an iterator of values in the map.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn new_map() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     map.insert(1, 0)?;
    ///     let mut values = map.values();
    ///     if let Some(value) = values.next() {
    ///         assert_eq!(*value, 0);
    ///     }
    /// #   else {
    /// #       panic!("Iter fails to get keypair");
    /// #   }
    /// #   Ok(())
    /// # }
    /// ```
    pub fn values(&self) -> Values<'_, K, V> {
        Values { iter: self.iter() }
    }

    /// Gets a mut iterator of values in the map.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn new_map() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     map.insert(1, 0)?;
    ///     let mut values = map.values_mut();
    ///     if let Some(value) = values.next() {
    ///         assert_eq!(*value, 0);
    ///         *value = 2;
    ///     }
    /// #   else {
    /// #       panic!("Iter fails to get keypair");
    /// #   }
    ///     if let Some(val) = map.get(&1) {
    ///         assert_eq!(*val, 2);
    ///     }
    /// #   else {
    /// #       panic!("Iter fails to get keypair");
    /// #   }
    /// #   Ok(())
    /// # }
    /// ```
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut {
            iter: self.iter_mut(),
        }
    }
}

impl<K, V, A: Allocator + Clone> IntoIterator for FixedCapMap<K, V, A> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V, A>;

    fn into_iter(self) -> IntoIter<K, V, A> {
        IntoIter {
            iter: self.table.into_iter(),
        }
    }
}

impl<'a, K, V, A: Allocator + Clone> IntoIterator for &'a FixedCapMap<K, V, A> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Iter<'a, K, V> {
        self.iter()
    }
}

#[inline]
fn eq_f_borrow<K: Eq + Borrow<T>, T: Eq, V>(k: &T) -> impl FnMut(&(K, V)) -> bool + '_ {
    move |x: &(K, V)| *k == *x.0.borrow()
}

#[inline]
fn eq_f<K: Eq, V>(k: &K) -> impl FnMut(&(K, V)) -> bool + '_ {
    move |x: &(K, V)| k.eq(&x.0)
}

impl<K: Hash + Eq, V, A: Allocator + Clone> FixedCapMap<K, V, A> {
    fn hash<T: Hash>(&self, k: &T) -> u64 {
        let mut hasher = self.hasher.build_hasher();
        k.hash(&mut hasher);
        hasher.finish()
    }

    /// Gets a reference of a value in the map, according to the key. Return None if the pair is not
    /// in the map.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn new_map() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     map.insert(1, 0)?;
    ///     if let Some(val) = map.get(&1) {
    ///         assert_eq!(*val, 0);
    ///     }
    /// #   else {
    /// #       panic!("Iter fails to get keypair");
    /// #   }
    ///     assert!(map.get(&2).is_none());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn get<T: Hash + Eq>(&self, k: &T) -> Option<&V>
    where
        K: Borrow<T>,
    {
        let hash = self.hash(k);
        let pair = self.table.get(hash, eq_f_borrow(k))?;
        Some(&pair.1)
    }

    /// Gets a mut reference of a value in the map, according to the key. Return None if the pair is
    /// not in the map.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn new_map() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     map.insert(1, 0)?;
    ///     if let Some(val) = map.get_mut(&1) {
    ///         assert_eq!(*val, 0);
    ///         *val = 2;
    ///     }
    /// #   else {
    /// #       panic!("Iter fails to get keypair");
    /// #   }
    ///     if let Some(val) = map.get_mut(&1) {
    ///         assert_eq!(*val, 2);
    ///     }
    /// #   else {
    /// #       panic!("Iter fails to get keypair");
    /// #   }
    ///     assert!(map.get(&2).is_none());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn get_mut<T: Hash + Eq>(&mut self, k: &T) -> Option<&mut V>
    where
        K: Borrow<T>,
    {
        let hash = self.hash(k);
        let pair = self.table.get_mut(hash, eq_f_borrow(k))?;
        Some(&mut pair.1)
    }

    /// Checks whether a key is in a map instance.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn new_map() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     map.insert(1, 0)?;
    ///     assert!(map.contains_key(&1));
    ///     assert!(!map.contains_key(&2));
    /// #   Ok(())
    /// # }
    /// ```
    pub fn contains_key<T: Hash + Eq>(&self, k: &T) -> bool
    where
        K: Borrow<T>,
    {
        self.get(k).is_some()
    }

    /// Inserts a key value pair into a map instance. This won't cause the map to expand if there's
    /// no more room for this new pair. If that happens, an error will be returned.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn new_map() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     map.insert(1, 0)?;
    ///     assert!(map.contains_key(&1));
    ///     // Insert 4 more and make the map full
    ///     map.insert(2, 1)?;
    ///     map.insert(3, 2)?;
    ///     map.insert(4, 3)?;
    ///     map.insert(5, 4)?;
    ///     // Insert again and trigger an error
    ///     assert!(map.insert(6, 5).is_err());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn insert(&mut self, k: K, v: V) -> Result<Option<V>, ContainerError> {
        if self.is_full() {
            return Err(ContainerError::Full);
        }
        let hash = self.hash(&k);
        // SAFETY:
        // Safe because finding the slot first, so the slot must exist in the RawTable.
        let ret = self
            .table
            .find(hash, eq_f(&k))
            .map(|bucket| unsafe { self.table.remove(bucket).0 .1 });
        self.table
            .try_insert_no_grow(hash, (k, v))
            .map_err(|_| ContainerError::Full)?;
        Ok(ret)
    }

    /// Removes a key value pair from a map instance. If the pair is in the map, return the value.
    /// If not, return None.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn new_map() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     map.insert(1, 0)?;
    ///     assert!(map.remove(&2).is_none());
    ///     if let Some(value) = map.remove(&1) {
    ///         assert_eq!(value, 0);
    ///     }
    /// #   else {
    /// #       panic!("Key pair should in the map.");
    /// #   }
    /// #   Ok(())
    /// # }
    /// ```
    pub fn remove<T: Hash + Eq>(&mut self, k: &T) -> Option<V>
    where
        K: Borrow<T>,
    {
        let hash = self.hash(k);
        // SAFETY:
        // Safe because finding the slot first, so the slot must exist in the RawTable.
        self.table
            .find(hash, eq_f_borrow(k))
            .map(|bucket| unsafe { self.table.remove(bucket).0 .1 })
    }
}

impl<K: Hash + Eq + Borrow<K> + SafeClone, V: Default, A: Allocator + Clone> FixedCapMap<K, V, A> {
    /// Gets a mut reference of a value in the map, according to the key. If the key is not in the
    /// map, call value struct default method to initialize it, insert the pair, and return a mut
    /// reference to the new default value.
    ///
    /// This won't cause the map to expand if there's no more room for this new pair. If that
    /// happens, an error will be returned.
    ///
    /// # Examples
    /// ```rust
    /// use ylong_map::FixedCapMap;
    /// # use ylong_stdx_common::ContainerError;
    /// use std::alloc::System;
    ///
    /// # fn new_map() -> Result<(), ContainerError> {
    ///     let size = 5;
    ///     let mut map: FixedCapMap<i32, i32, System> = FixedCapMap::new(size, System::default())?;
    ///     if let Ok(val) = map.get_mut_default(&1) {
    ///         *val = 0;
    ///     }
    /// #   else {
    /// #       panic!("Should initialize a new pair");
    /// #   }
    ///     if let Some(val) = map.get(&1) {
    ///         assert_eq!(*val, 0);
    ///     }
    /// #   else {
    /// #       panic!("Iter fails to get keypair");
    /// #   }
    /// #   Ok(())
    /// # }
    /// ```
    pub fn get_mut_default(&mut self, k: &K) -> Result<&mut V, ContainerError> {
        let hash = self.hash(k);
        let bucket = match self.table.find(hash, eq_f(k)) {
            Some(ret) => ret,
            None => {
                if self.is_full() {
                    return Err(ContainerError::Full);
                }
                self.table
                    .try_insert_no_grow(hash, (k.safe_clone()?, V::default()))
                    .map_err(|_| ContainerError::Full)?
            }
        };
        // SAFETY:
        // Safe because checking and inserting before getting a mutable reference.
        Ok(unsafe { &mut bucket.as_mut().1 })
    }
}

impl<K: SafeClone + Hash + Eq, V: SafeClone, A: Allocator + Clone> FixedCapMap<K, V, A> {
    fn update_table_from(&mut self, src: &FixedCapMap<K, V, A>) -> Result<(), ContainerError> {
        for (k, v) in src {
            let _ = self.insert(k.safe_clone()?, v.safe_clone()?)?;
        }
        Ok(())
    }
}

impl<K: SafeClone + Hash + Eq, V: SafeClone, A: Allocator + Clone> SafeClone
    for FixedCapMap<K, V, A>
{
    fn safe_clone(&self) -> Result<Self, ContainerError> {
        let allocator = self.table.allocator().safe_clone()?;
        let limit = self.limit;
        let table = RawTable::<(K, V), A>::try_with_capacity_in(limit, allocator)?;
        let mut ret = FixedCapMap {
            table,
            hasher: self.hasher.clone(),
            limit,
        };
        ret.update_table_from(self)?;
        Ok(ret)
    }

    fn safe_clone_from(&mut self, source: &Self) -> Result<(), ContainerError> {
        *self = source.safe_clone()?;
        Ok(())
    }
}

impl<K: Debug, V: Debug, A: Allocator + Clone + Debug> Debug for FixedCapMap<K, V, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TryMap: {")?;
        f.debug_map().entries(self.iter()).finish()?;
        f.write_fmt(format_args!(
            ", Allocator: {:?}, Limit: {}}}",
            self.table.allocator(),
            self.limit
        ))
    }
}

impl<K: Hash + Eq, V: PartialEq, A: Allocator + Clone> PartialEq for FixedCapMap<K, V, A> {
    fn eq(&self, other: &Self) -> bool {
        if self.limit != other.limit || self.len() != other.len() {
            return false;
        }
        self.iter()
            .all(|x| other.get(x.0).map_or(false, |v| x.1 == v))
    }
}
