#![feature(test)]
#![feature(allocator_api)]
extern crate test;

use core::hash::Hash;
use std::collections::HashMap;
use test::{black_box, Bencher};
use ylong_stdx_common::SafeClone;
mod common;

fn create_map_from_iter<T: Hash + Eq + Copy, I: IntoIterator<Item = T>>(
    iter: I,
    cap: usize,
) -> HashMap<T, T> {
    let mut hm = HashMap::with_capacity(cap);

    for dat in iter {
        hm.insert(dat, dat);
    }

    hm
}

fn do_bench_new(b: &mut Bencher, src_len: usize) {
    b.bytes = src_len as u64;

    b.iter(black_box(|| HashMap::<i32, i32>::with_capacity(src_len)))
}
bench_new!();

fn do_bench_insert(b: &mut Bencher, src_len: usize) {
    b.iter(black_box(|| {
        let mut hm = HashMap::with_capacity(src_len);
        for i in 0..src_len {
            hm.insert(i, i);
        }
    }));
}
bench_insert!();

fn do_bench_remove(b: &mut Bencher, src_len: usize) {
    b.iter(black_box(|| {
        let mut hm = create_map_from_iter(0..src_len, src_len);

        for i in 0..src_len {
            hm.remove(&i).unwrap();
        }
    }));
}
bench_remove!();

fn do_bench_contains_key(b: &mut Bencher, src_len: usize) {
    let hm = create_map_from_iter(0..src_len, src_len);

    b.iter(black_box(|| {
        for i in 0..src_len {
            black_box(hm.contains_key(&i));
        }
    }));
}
bench_contains_key!();

fn do_bench_not_contains_key(b: &mut Bencher, src_len: usize) {
    let hm = create_map_from_iter(0..src_len, src_len);

    b.iter(black_box(|| {
        for i in src_len..src_len + src_len {
            black_box(hm.contains_key(&i));
        }
    }));
}
bench_not_contains_key!();

fn do_bench_get(b: &mut Bencher, src_len: usize) {
    let hm = create_map_from_iter(0..src_len, src_len);

    b.iter(black_box(|| {
        for i in 0..src_len {
            hm.get(&i).unwrap();
        }
    }));
}
bench_get!();

fn do_bench_clone(b: &mut Bencher, src_len: usize) {
    let hm = create_map_from_iter(0..src_len, src_len);

    b.bytes = src_len as u64;

    b.iter(black_box(|| hm.safe_clone().unwrap()));
}
bench_clone!();

fn do_bench_clone_from(b: &mut Bencher, src_len: usize) {
    let src = create_map_from_iter(0..src_len, src_len);

    b.bytes = src_len as u64;

    b.iter(black_box(|| {
        let mut dst = create_map_from_iter(0..0, src_len);
        dst.safe_clone_from(&src).unwrap();
    }));
}
bench_clone_from!();

fn do_bench_into_iter(b: &mut Bencher, src_len: usize) {
    b.iter(black_box(|| {
        let hm = create_map_from_iter(0..src_len, src_len);
        let mut sum = 0;
        for i in hm.into_iter() {
            sum += i.0 + i.1;
        }
        sum
    }));
}
bench_into_iter!();

fn do_bench_iter_mut(b: &mut Bencher, src_len: usize) {
    let mut hm = create_map_from_iter(0..src_len, src_len);
    b.iter(black_box(|| {
        let mut sum = 0;
        for i in hm.iter_mut() {
            sum += i.0;
            *i.1 += 1;
        }
        sum
    }));
}
bench_iter_mut!();

fn do_bench_keys(b: &mut Bencher, src_len: usize) {
    let hm = create_map_from_iter(0..src_len, src_len);

    b.iter(black_box(|| {
        let mut sum = 0;
        for i in hm.keys() {
            sum += i;
        }
        sum
    }));
}
bench_keys!();

fn do_bench_values_mut(b: &mut Bencher, src_len: usize) {
    let mut hm = create_map_from_iter(0..src_len, src_len);

    b.iter(black_box(|| {
        for i in hm.values_mut() {
            *i += 1;
        }
    }));
}
bench_values_mut!();
