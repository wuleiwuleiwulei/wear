//! 测试用例0000-0010-0100-1000代表AllocatorBox申请的内存大小，单位KB

#![feature(test)]
#![feature(allocator_api)]
extern crate test;

use std::alloc::Global;
use test::{black_box, Bencher};
use ylong_box::AllocatorBox;
use ylong_stdx_common::SafeClone;

#[bench]
fn bench_new_0001(b: &mut Bencher) {
    b.iter(black_box(|| {
        AllocatorBox::new([1u8; 1000], Global).unwrap()
    }))
}

#[bench]
fn bench_new_0100(b: &mut Bencher) {
    b.iter(black_box(|| {
        AllocatorBox::new([1u8; 100000], Global).unwrap()
    }))
}

#[bench]
fn bench_new_1000(b: &mut Bencher) {
    b.iter(black_box(|| {
        AllocatorBox::new([1u8; 1000000], Global).unwrap()
    }))
}

#[bench]
fn bench_clone_0001(b: &mut Bencher) {
    let x = AllocatorBox::new([1u8; 1000], Global).unwrap();

    b.iter(black_box(|| x.safe_clone().unwrap()))
}

#[bench]
fn bench_clone_0100(b: &mut Bencher) {
    let x = AllocatorBox::new([1u8; 100000], Global).unwrap();

    b.iter(black_box(|| x.safe_clone().unwrap()))
}

#[bench]
fn bench_clone_1000(b: &mut Bencher) {
    let x = AllocatorBox::new([1u8; 1000000], Global).unwrap();

    b.iter(black_box(|| x.safe_clone().unwrap()))
}

#[bench]
fn bench_clone_from_0001(b: &mut Bencher) {
    let src = AllocatorBox::new([1u8; 1000], Global).unwrap();

    b.iter(black_box(|| {
        let mut dst = AllocatorBox::new([10u8; 1000], Global).unwrap();
        dst.safe_clone_from(&src).unwrap();
        dst
    }));
}

#[bench]
fn bench_clone_from_0100(b: &mut Bencher) {
    let src = AllocatorBox::new([1u8; 100000], Global).unwrap();

    b.iter(black_box(|| {
        let mut dst = AllocatorBox::new([10u8; 100000], Global).unwrap();
        dst.safe_clone_from(&src).unwrap();
        dst
    }));
}

#[bench]
fn bench_clone_from_1000(b: &mut Bencher) {
    let src = AllocatorBox::new([1u8; 1000000], Global).unwrap();

    b.iter(black_box(|| {
        let mut dst = AllocatorBox::new([10u8; 1000000], Global).unwrap();
        dst.safe_clone_from(&src).unwrap();
        dst
    }));
}

#[bench]
fn bench_into_inner_0001(b: &mut Bencher) {
    b.iter(black_box(|| {
        let src = AllocatorBox::new([1u8; 1000], Global).unwrap();
        AllocatorBox::into_inner(src)
    }));
}

#[bench]
fn bench_into_inner_0100(b: &mut Bencher) {
    b.iter(black_box(|| {
        let src = AllocatorBox::new([1u8; 100000], Global).unwrap();
        AllocatorBox::into_inner(src)
    }));
}

#[bench]
fn bench_into_inner_1000(b: &mut Bencher) {
    b.iter(black_box(|| {
        let src = AllocatorBox::new([1u8; 1000000], Global).unwrap();
        AllocatorBox::into_inner(src)
    }));
}

#[bench]
fn bench_from_raw_0001(b: &mut Bencher) {
    b.iter(black_box(|| unsafe {
        let src = AllocatorBox::new([1u8; 1000], Global).unwrap();
        let ptr = AllocatorBox::into_raw(src);
        AllocatorBox::from_raw(ptr, Global)
    }));
}

#[bench]
fn bench_from_raw_0100(b: &mut Bencher) {
    b.iter(black_box(|| unsafe {
        let src = AllocatorBox::new([1u8; 100000], Global).unwrap();
        let ptr = AllocatorBox::into_raw(src);
        AllocatorBox::from_raw(ptr, Global)
    }));
}

#[bench]
fn bench_from_raw_1000(b: &mut Bencher) {
    b.iter(black_box(|| unsafe {
        let src = AllocatorBox::new([1u8; 1000000], Global).unwrap();
        let ptr = AllocatorBox::into_raw(src);
        AllocatorBox::from_raw(ptr, Global)
    }));
}
