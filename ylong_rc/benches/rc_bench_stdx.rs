//! 测试用例0000-0010-0100-1000代表RcWithAlloc申请的内存大小，单位KB

#![feature(test)]
#![feature(allocator_api)]
extern crate test;

use std::alloc::Global;
use test::{black_box, Bencher};
use ylong_rc::RcWithAlloc;
use ylong_stdx_common::SafeClone;

#[bench]
fn bench_rc_new_0001(b: &mut Bencher) {
    b.iter(black_box(|| RcWithAlloc::new([1u8; 1000], Global).unwrap()))
}

#[bench]
fn bench_rc_new_0100(b: &mut Bencher) {
    b.iter(black_box(|| {
        RcWithAlloc::new([1u8; 100000], Global).unwrap()
    }))
}

#[bench]
fn bench_rc_new_1000(b: &mut Bencher) {
    b.iter(black_box(|| {
        RcWithAlloc::new([1u8; 1000000], Global).unwrap()
    }))
}

#[bench]
fn bench_clone_0001(b: &mut Bencher) {
    let x = RcWithAlloc::new([1u8; 1000], Global).unwrap();

    b.iter(black_box(|| x.safe_clone().unwrap()))
}

#[bench]
fn bench_clone_0100(b: &mut Bencher) {
    let x = RcWithAlloc::new([1u8; 100000], Global).unwrap();

    b.iter(black_box(|| x.safe_clone().unwrap()))
}

#[bench]
fn bench_clone_1000(b: &mut Bencher) {
    let x = RcWithAlloc::new([1u8; 1000000], Global).unwrap();

    b.iter(black_box(|| x.safe_clone().unwrap()))
}

#[bench]
fn bench_clone_from_0001(b: &mut Bencher) {
    let src = RcWithAlloc::new([1u8; 1000], Global).unwrap();

    b.iter(black_box(|| {
        let mut dst = RcWithAlloc::new([10u8; 1000], Global).unwrap();
        dst.safe_clone_from(&src).unwrap();
        dst
    }));
}

#[bench]
fn bench_clone_from_0100(b: &mut Bencher) {
    let src = RcWithAlloc::new([1u8; 100000], Global).unwrap();

    b.iter(black_box(|| {
        let mut dst = RcWithAlloc::new([10u8; 100000], Global).unwrap();
        dst.safe_clone_from(&src).unwrap();
        dst
    }));
}

#[bench]
fn bench_clone_from_1000(b: &mut Bencher) {
    let src = RcWithAlloc::new([1u8; 1000000], Global).unwrap();

    b.iter(black_box(|| {
        let mut dst = RcWithAlloc::new([10u8; 1000000], Global).unwrap();
        dst.safe_clone_from(&src).unwrap();
        dst
    }));
}

#[bench]
fn bench_into_inner_0001(b: &mut Bencher) {
    b.iter(black_box(|| {
        let src = RcWithAlloc::new([1u8; 1000], Global).unwrap();
        RcWithAlloc::into_inner(src).unwrap()
    }));
}

#[bench]
fn bench_into_inner_0100(b: &mut Bencher) {
    b.iter(black_box(|| {
        let src = RcWithAlloc::new([1u8; 100000], Global).unwrap();
        RcWithAlloc::into_inner(src).unwrap()
    }));
}

#[bench]
fn bench_into_inner_1000(b: &mut Bencher) {
    b.iter(black_box(|| {
        let src = RcWithAlloc::new([1u8; 1000000], Global).unwrap();
        RcWithAlloc::into_inner(src).unwrap()
    }));
}

#[bench]
fn bench_make_mut_0001(b: &mut Bencher) {
    let mut src = RcWithAlloc::new([1u8; 1000], Global).unwrap();

    b.iter(black_box(|| {
        *RcWithAlloc::make_mut(&mut src).unwrap() = [10u8; 1000]
    }));
}

#[bench]
fn bench_make_mut_0100(b: &mut Bencher) {
    let mut src = RcWithAlloc::new([1u8; 100000], Global).unwrap();

    b.iter(black_box(|| {
        *RcWithAlloc::make_mut(&mut src).unwrap() = [10u8; 100000]
    }));
}

#[bench]
fn bench_make_mut_1000(b: &mut Bencher) {
    let mut src = RcWithAlloc::new([1u8; 1000000], Global).unwrap();

    b.iter(black_box(|| {
        *RcWithAlloc::make_mut(&mut src).unwrap() = [10u8; 1000000]
    }));
}

#[bench]
fn bench_downgrade_0001(b: &mut Bencher) {
    let src = RcWithAlloc::new([1u8; 1000], Global).unwrap();

    b.iter(black_box(|| RcWithAlloc::downgrade(&src).unwrap()));
}

#[bench]
fn bench_downgrade_0100(b: &mut Bencher) {
    let src = RcWithAlloc::new([1u8; 100000], Global).unwrap();

    b.iter(black_box(|| RcWithAlloc::downgrade(&src).unwrap()));
}

#[bench]
fn bench_downgrade_1000(b: &mut Bencher) {
    let src = RcWithAlloc::new([1u8; 1000000], Global).unwrap();

    b.iter(black_box(|| RcWithAlloc::downgrade(&src).unwrap()));
}

#[bench]
fn bench_upgrade_0001(b: &mut Bencher) {
    let src = RcWithAlloc::new([1u8; 1000], Global).unwrap();
    let w = RcWithAlloc::downgrade(&src).unwrap();
    b.iter(black_box(|| w.upgrade().unwrap().unwrap()));
}

#[bench]
fn bench_upgrade_0100(b: &mut Bencher) {
    let src = RcWithAlloc::new([1u8; 100000], Global).unwrap();
    let w = RcWithAlloc::downgrade(&src).unwrap();
    b.iter(black_box(|| w.upgrade().unwrap().unwrap()));
}

#[bench]
fn bench_upgrade_1000(b: &mut Bencher) {
    let src = RcWithAlloc::new([1u8; 1000000], Global).unwrap();
    let w = RcWithAlloc::downgrade(&src).unwrap();
    b.iter(black_box(|| w.upgrade().unwrap().unwrap()));
}
