//! 测试用例0000-0010-0100-1000代表创建的Vec的长度

#[macro_export]
macro_rules! bench_with_capacity {
    () => {
        #[bench]
        fn bench_with_capacity_0000(b: &mut Bencher) {
            do_bench_with_capacity(b, 0)
        }

        #[bench]
        fn bench_with_capacity_0010(b: &mut Bencher) {
            do_bench_with_capacity(b, 10)
        }

        #[bench]
        fn bench_with_capacity_0100(b: &mut Bencher) {
            do_bench_with_capacity(b, 100)
        }

        #[bench]
        fn bench_with_capacity_1000(b: &mut Bencher) {
            do_bench_with_capacity(b, 1000)
        }
    };
}

#[macro_export]
macro_rules! bench_push {
    () => {
        #[bench]
        fn bench_push_0010(b: &mut Bencher) {
            do_bench_push(b, 10);
        }

        #[bench]
        fn bench_push_0100(b: &mut Bencher) {
            do_bench_push(b, 100);
        }

        #[bench]
        fn bench_push_1000(b: &mut Bencher) {
            do_bench_push(b, 1000);
        }
    };
}

#[macro_export]
macro_rules! bench_pop {
    () => {
        #[bench]
        fn bench_pop_0010(b: &mut Bencher) {
            do_bench_pop(b, 10);
        }

        #[bench]
        fn bench_pop_0100(b: &mut Bencher) {
            do_bench_pop(b, 100);
        }

        #[bench]
        fn bench_pop_1000(b: &mut Bencher) {
            do_bench_pop(b, 1000);
        }
    };
}

#[macro_export]
macro_rules! bench_extend_from_slice {
    () => {
        #[bench]
        fn bench_extend_from_slice_0000_0000(b: &mut Bencher) {
            do_bench_extend_from_slice(b, 0, 0)
        }

        #[bench]
        fn bench_extend_from_slice_0000_0010(b: &mut Bencher) {
            do_bench_extend_from_slice(b, 0, 10)
        }

        #[bench]
        fn bench_extend_from_slice_0000_1000(b: &mut Bencher) {
            do_bench_extend_from_slice(b, 0, 1000)
        }

        #[bench]
        fn bench_extend_from_slice_0010_0010(b: &mut Bencher) {
            do_bench_extend_from_slice(b, 10, 10)
        }

        #[bench]
        fn bench_extend_from_slice_0100_0100(b: &mut Bencher) {
            do_bench_extend_from_slice(b, 100, 100)
        }

        #[bench]
        fn bench_extend_from_slice_1000_1000(b: &mut Bencher) {
            do_bench_extend_from_slice(b, 1000, 1000)
        }
    };
}

#[macro_export]
macro_rules! bench_clone {
    () => {
        #[bench]
        fn bench_clone_0000(b: &mut Bencher) {
            do_bench_clone(b, 0)
        }

        #[bench]
        fn bench_clone_0010(b: &mut Bencher) {
            do_bench_clone(b, 10)
        }

        #[bench]
        fn bench_clone_0100(b: &mut Bencher) {
            do_bench_clone(b, 100)
        }

        #[bench]
        fn bench_clone_1000(b: &mut Bencher) {
            do_bench_clone(b, 1000)
        }
    };
}

#[macro_export]
macro_rules! bench_clone_from {
    () => {
        #[bench]
        fn bench_clone_from_0000(b: &mut Bencher) {
            do_bench_clone_from(b, 0)
        }

        #[bench]
        fn bench_clone_from_0010(b: &mut Bencher) {
            do_bench_clone_from(b, 10)
        }

        #[bench]
        fn bench_clone_from_0100(b: &mut Bencher) {
            do_bench_clone_from(b, 100)
        }

        #[bench]
        fn bench_clone_from_1000(b: &mut Bencher) {
            do_bench_clone_from(b, 1000)
        }
    };
}

#[macro_export]
macro_rules! bench_truncate {
    () => {
        #[bench]
        fn bench_truncate_0000_0000(b: &mut Bencher) {
            do_bench_truncate(b, 0, 0);
        }

        #[bench]
        fn bench_truncate_0010_0001(b: &mut Bencher) {
            do_bench_truncate(b, 10, 1);
        }

        #[bench]
        fn bench_truncate_0100_0010(b: &mut Bencher) {
            do_bench_truncate(b, 100, 10);
        }

        #[bench]
        fn bench_truncate_1000_0100(b: &mut Bencher) {
            do_bench_truncate(b, 1000, 100);
        }
    };
}

#[macro_export]
macro_rules! bench_push_front {
    () => {
        #[bench]
        fn bench_push_front_0010(b: &mut Bencher) {
            do_bench_push_front(b, 10);
        }

        #[bench]
        fn bench_push_front_0100(b: &mut Bencher) {
            do_bench_push_front(b, 100);
        }

        #[bench]
        fn bench_push_front_1000(b: &mut Bencher) {
            do_bench_push_front(b, 1000);
        }
    };
}

#[macro_export]
macro_rules! bench_push_back {
    () => {
        #[bench]
        fn bench_push_back_0010(b: &mut Bencher) {
            do_bench_push_back(b, 10);
        }

        #[bench]
        fn bench_push_back_0100(b: &mut Bencher) {
            do_bench_push_back(b, 100);
        }

        #[bench]
        fn bench_push_back_1000(b: &mut Bencher) {
            do_bench_push_back(b, 1000);
        }
    };
}

#[macro_export]
macro_rules! bench_pop_back {
    () => {
        #[bench]
        fn bench_pop_back_0010(b: &mut Bencher) {
            do_bench_pop_back(b, 10);
        }

        #[bench]
        fn bench_pop_back_0100(b: &mut Bencher) {
            do_bench_pop_back(b, 100);
        }

        #[bench]
        fn bench_pop_back_1000(b: &mut Bencher) {
            do_bench_pop_back(b, 1000);
        }
    };
}

#[macro_export]
macro_rules! bench_pop_front {
    () => {
        #[bench]
        fn bench_pop_front_0010(b: &mut Bencher) {
            do_bench_pop_front(b, 10);
        }

        #[bench]
        fn bench_pop_front_0100(b: &mut Bencher) {
            do_bench_pop_front(b, 100);
        }

        #[bench]
        fn bench_pop_front_1000(b: &mut Bencher) {
            do_bench_pop_front(b, 1000);
        }
    };
}

#[macro_export]
macro_rules! bench_into_iter {
    () => {
        #[bench]
        fn bench_into_iter_0000(b: &mut Bencher) {
            do_bench_into_iter(b, 0);
        }

        #[bench]
        fn bench_into_iter_0010(b: &mut Bencher) {
            do_bench_into_iter(b, 10);
        }

        #[bench]
        fn bench_into_iter_0100(b: &mut Bencher) {
            do_bench_into_iter(b, 100);
        }

        #[bench]
        fn bench_into_iter_1000(b: &mut Bencher) {
            do_bench_into_iter(b, 1000);
        }
    };
}
