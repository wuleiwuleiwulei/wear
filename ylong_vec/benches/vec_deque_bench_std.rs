#![feature(test)]
#![feature(allocator_api)]
extern crate test;

use std::alloc::Global;
use std::collections::VecDeque;
use test::{black_box, Bencher};
use ylong_stdx_common::SafeClone;
mod common;

fn create_vecdeque_from_iter<T, I: IntoIterator<Item = T>>(iter: I, cap: usize) -> VecDeque<T> {
    let mut deq = VecDeque::with_capacity_in(cap, Global);

    for dat in iter {
        deq.push_back(dat);
    }

    deq
}

fn do_bench_with_capacity(b: &mut Bencher, src_len: usize) {
    b.bytes = src_len as u64;

    b.iter(black_box(|| {
        VecDeque::<i32>::with_capacity_in(src_len, Global)
    }))
}
bench_with_capacity!();

fn do_bench_push_back(b: &mut Bencher, src_len: usize) {
    b.iter(black_box(|| {
        let mut deq = VecDeque::with_capacity_in(src_len, Global);
        for i in 0..src_len {
            deq.push_back(i);
        }
    }));
}
bench_push_back!();

fn do_bench_push_front(b: &mut Bencher, src_len: usize) {
    b.iter(black_box(|| {
        let mut deq = VecDeque::with_capacity_in(src_len, Global);
        for i in 0..src_len {
            deq.push_front(i);
        }
    }));
}
bench_push_front!();

fn do_bench_pop_back(b: &mut Bencher, src_len: usize) {
    b.iter(black_box(|| {
        let mut deq = create_vecdeque_from_iter(0..src_len, src_len);
        for _ in 0..src_len {
            deq.pop_back().unwrap();
        }
    }));
}
bench_pop_back!();

fn do_bench_pop_front(b: &mut Bencher, src_len: usize) {
    b.iter(black_box(|| {
        let mut deq = create_vecdeque_from_iter(0..src_len, src_len);
        for _ in 0..src_len {
            deq.pop_front().unwrap();
        }
    }));
}
bench_pop_front!();

fn do_bench_clone(b: &mut Bencher, src_len: usize) {
    let deq = create_vecdeque_from_iter(0..src_len, src_len);

    b.bytes = src_len as u64;

    b.iter(black_box(|| deq.safe_clone().unwrap()));
}
bench_clone!();

fn do_bench_clone_from(b: &mut Bencher, src_len: usize) {
    let src = create_vecdeque_from_iter(0..src_len, src_len);

    b.bytes = src_len as u64;

    b.iter(black_box(|| {
        let mut dst = create_vecdeque_from_iter(0..0, src_len);
        dst.safe_clone_from(&src).unwrap();
    }));
}
bench_clone_from!();

fn do_bench_into_iter(b: &mut Bencher, src_len: usize) {
    b.iter(black_box(|| {
        let src = create_vecdeque_from_iter(0..src_len, src_len);
        let mut sum = 0;
        for i in src.into_iter() {
            sum += i;
        }
        sum
    }));
}
bench_into_iter!();
