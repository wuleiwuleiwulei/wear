use core::iter::Iterator;
use std::alloc::Allocator;
use std::collections::hash_map::{RawIntoIter, RawIter};
use std::marker::PhantomData;

#[cfg(test)]
mod ut_test;

/// Iterator over the map.
pub struct Iter<'a, K, V> {
    pub(crate) iter: RawIter<(K, V)>,
    pub(crate) phantom: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        // SAFETY:
        // Safe because checking before getting a reference.
        let tuple = self.iter.next().map(|bucket| unsafe { bucket.as_ref() })?;
        Some((&tuple.0, &tuple.1))
    }
}

/// Mutable iterator over the map.
pub struct IterMut<'a, K, V> {
    pub(crate) iter: RawIter<(K, V)>,
    pub(crate) phantom: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        // SAFETY:
        // Safe because checking before getting a mutable reference.
        let tuple = self.iter.next().map(|bucket| unsafe { bucket.as_mut() })?;
        Some((&tuple.0, &mut tuple.1))
    }
}

/// Into iterator of the map.
pub struct IntoIter<K, V, A: Allocator + Clone> {
    pub(crate) iter: RawIntoIter<(K, V), A>,
}

impl<K, V, A: Allocator + Clone> Iterator for IntoIter<K, V, A> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        self.iter.next()
    }
}

/// Iterator of keys in the map.
pub struct Keys<'a, K, V> {
    pub(crate) iter: Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<&'a K> {
        self.iter.next().map(|x| x.0)
    }
}

/// Into iterator of keys in the map.
pub struct IntoKeys<K, V, A: Allocator + Clone> {
    pub(crate) iter: IntoIter<K, V, A>,
}

impl<K, V, A: Allocator + Clone> Iterator for IntoKeys<K, V, A> {
    type Item = K;

    fn next(&mut self) -> Option<K> {
        self.iter.next().map(|x| x.0)
    }
}

/// Iterator of values in the map.
pub struct Values<'a, K, V> {
    pub(crate) iter: Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<&'a V> {
        self.iter.next().map(|x| x.1)
    }
}

/// Mutable iterator of values in the map.
pub struct ValuesMut<'a, K, V> {
    pub(crate) iter: IterMut<'a, K, V>,
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<&'a mut V> {
        self.iter.next().map(|x| x.1)
    }
}

/// Into iterator of values in the map.
pub struct IntoValues<K, V, A: Allocator + Clone> {
    pub(crate) iter: IntoIter<K, V, A>,
}

impl<K, V, A: Allocator + Clone> Iterator for IntoValues<K, V, A> {
    type Item = V;

    fn next(&mut self) -> Option<V> {
        self.iter.next().map(|x| x.1)
    }
}
