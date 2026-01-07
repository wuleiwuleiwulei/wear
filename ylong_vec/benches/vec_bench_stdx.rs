#![feature(test)]
#![feature(allocator_api)]
extern crate test;

use std::alloc::Global;
use test::{black_box, Bencher};
use ylong_stdx_common::SafeClone;
use ylong_vec::FixedCapVec;
mod common;

fn create_capvec_from_iter<T, I: IntoIterator<Item = T>>(
    iter: I,
    cap: usize,
) -> FixedCapVec<T, Global> {
    let mut vec: FixedCapVec<T, _> = FixedCapVec::<T, Global>::new(cap, Global).unwrap();

    for dat in iter {
        vec.push(dat).unwrap();
    }

    vec
}

fn do_bench_with_capacity(b: &mut Bencher, src_len: usize) {
    b.bytes = src_len as u64;

    b.iter(black_box(|| {
        FixedCapVec::<i32, Global>::new(src_len, Global)
    }))
}
bench_with_capacity!();

fn do_bench_push(b: &mut Bencher, src_len: usize) {
    b.iter(black_box(|| {
        let mut vec: FixedCapVec<usize, _> =
            FixedCapVec::<usize, Global>::new(src_len, Global).unwrap();
        for i in 0..src_len {
            vec.push(i).unwrap();
        }
    }));
}
bench_push!();

fn do_bench_pop(b: &mut Bencher, src_len: usize) {
    b.iter(black_box(|| {
        let mut vec: FixedCapVec<usize, Global> = create_capvec_from_iter(0..src_len, src_len);
        for _ in 0..src_len {
            vec.pop().unwrap();
        }
    }));
}
bench_pop!();

fn do_bench_extend_from_slice(b: &mut Bencher, dst_len: usize, src_len: usize) {
    let src: FixedCapVec<usize, Global> =
        create_capvec_from_iter(dst_len..dst_len + src_len, src_len);

    b.bytes = src_len as u64;

    b.iter(black_box(|| {
        let mut dst: FixedCapVec<usize, Global> =
            create_capvec_from_iter(0..dst_len, dst_len + src_len);
        dst.extend_from_slice(&src).unwrap();
        dst
    }));
}
bench_extend_from_slice!();

fn do_bench_clone(b: &mut Bencher, src_len: usize) {
    let src = create_capvec_from_iter(0..src_len, src_len);

    b.bytes = src_len as u64;

    b.iter(black_box(|| src.safe_clone().unwrap()));
}
bench_clone!();

fn do_bench_clone_from(b: &mut Bencher, src_len: usize) {
    let src = create_capvec_from_iter(0..src_len, src_len);

    b.bytes = src_len as u64;

    b.iter(black_box(|| {
        let mut dst = create_capvec_from_iter(0..0, src_len);
        dst.safe_clone_from(&src).unwrap();
    }));
}
bench_clone_from!();

fn do_bench_truncate(b: &mut Bencher, src_len: usize, truncate_len: usize) {
    b.bytes = truncate_len as u64;

    b.iter(black_box(|| {
        let mut src = create_capvec_from_iter(0..src_len, src_len);
        src.truncate(truncate_len);
        src
    }));
}
bench_truncate!();

fn do_bench_into_iter(b: &mut Bencher, src_len: usize) {
    b.iter(black_box(|| {
        let src = create_capvec_from_iter(0..src_len, src_len);
        let mut sum = 0;
        for i in src.into_iter() {
            sum += i;
        }
        sum
    }));
}
bench_into_iter!();
