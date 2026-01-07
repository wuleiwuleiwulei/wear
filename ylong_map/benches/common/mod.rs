//! 测试用例0000-0010-0100-1000代表创建的HashMap的长度

#[macro_export]
macro_rules! bench_new {
    () => {
        #[bench]
        fn bench_new_0000(b: &mut Bencher) {
            do_bench_new(b, 0)
        }

        #[bench]
        fn bench_new_0010(b: &mut Bencher) {
            do_bench_new(b, 10)
        }

        #[bench]
        fn bench_new_0100(b: &mut Bencher) {
            do_bench_new(b, 100)
        }

        #[bench]
        fn bench_new_1000(b: &mut Bencher) {
            do_bench_new(b, 1000)
        }
    };
}

#[macro_export]
macro_rules! bench_insert {
    () => {
        #[bench]
        fn bench_insert_0000(b: &mut Bencher) {
            do_bench_insert(b, 0)
        }

        #[bench]
        fn bench_insert_0010(b: &mut Bencher) {
            do_bench_insert(b, 10)
        }

        #[bench]
        fn bench_insert_0100(b: &mut Bencher) {
            do_bench_insert(b, 100)
        }

        #[bench]
        fn bench_insert_1000(b: &mut Bencher) {
            do_bench_insert(b, 1000)
        }
    };
}

#[macro_export]
macro_rules! bench_remove {
    () => {
        #[bench]
        fn bench_remove_0000(b: &mut Bencher) {
            do_bench_remove(b, 0)
        }

        #[bench]
        fn bench_remove_0010(b: &mut Bencher) {
            do_bench_remove(b, 10)
        }

        #[bench]
        fn bench_remove_0100(b: &mut Bencher) {
            do_bench_remove(b, 100)
        }

        #[bench]
        fn bench_remove_1000(b: &mut Bencher) {
            do_bench_remove(b, 1000)
        }
    };
}

#[macro_export]
macro_rules! bench_contains_key {
    () => {
        #[bench]
        fn bench_contains_key_0000(b: &mut Bencher) {
            do_bench_contains_key(b, 0)
        }

        #[bench]
        fn bench_contains_key_0010(b: &mut Bencher) {
            do_bench_contains_key(b, 10)
        }

        #[bench]
        fn bench_contains_key_0100(b: &mut Bencher) {
            do_bench_contains_key(b, 100)
        }

        #[bench]
        fn bench_contains_key_1000(b: &mut Bencher) {
            do_bench_contains_key(b, 1000)
        }
    };
}

#[macro_export]
macro_rules! bench_not_contains_key {
    () => {
        #[bench]
        fn bench_not_contains_key_0000(b: &mut Bencher) {
            do_bench_not_contains_key(b, 0)
        }

        #[bench]
        fn bench_not_contains_key_0010(b: &mut Bencher) {
            do_bench_not_contains_key(b, 10)
        }

        #[bench]
        fn bench_not_contains_key_0100(b: &mut Bencher) {
            do_bench_not_contains_key(b, 100)
        }

        #[bench]
        fn bench_not_contains_key_1000(b: &mut Bencher) {
            do_bench_not_contains_key(b, 1000)
        }
    };
}

#[macro_export]
macro_rules! bench_get {
    () => {
        #[bench]
        fn bench_get_0000(b: &mut Bencher) {
            do_bench_get(b, 0)
        }

        #[bench]
        fn bench_get_0010(b: &mut Bencher) {
            do_bench_get(b, 10)
        }

        #[bench]
        fn bench_get_0100(b: &mut Bencher) {
            do_bench_get(b, 100)
        }

        #[bench]
        fn bench_get_1000(b: &mut Bencher) {
            do_bench_get(b, 1000)
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

#[macro_export]
macro_rules! bench_iter_mut {
    () => {
        #[bench]
        fn bench_iter_mut_0000(b: &mut Bencher) {
            do_bench_iter_mut(b, 0);
        }

        #[bench]
        fn bench_iter_mut_0010(b: &mut Bencher) {
            do_bench_iter_mut(b, 10);
        }

        #[bench]
        fn bench_iter_mut_0100(b: &mut Bencher) {
            do_bench_iter_mut(b, 100);
        }

        #[bench]
        fn bench_iter_mut_1000(b: &mut Bencher) {
            do_bench_iter_mut(b, 1000);
        }
    };
}

#[macro_export]
macro_rules! bench_keys {
    () => {
        #[bench]
        fn bench_keys_0000(b: &mut Bencher) {
            do_bench_keys(b, 0);
        }

        #[bench]
        fn bench_keys_0010(b: &mut Bencher) {
            do_bench_keys(b, 10);
        }

        #[bench]
        fn bench_keys_0100(b: &mut Bencher) {
            do_bench_keys(b, 100);
        }

        #[bench]
        fn bench_keys_1000(b: &mut Bencher) {
            do_bench_keys(b, 1000);
        }
    };
}

#[macro_export]
macro_rules! bench_values_mut {
    () => {
        #[bench]
        fn bench_values_mut_0000(b: &mut Bencher) {
            do_bench_values_mut(b, 0);
        }

        #[bench]
        fn bench_values_mut_0010(b: &mut Bencher) {
            do_bench_values_mut(b, 10);
        }

        #[bench]
        fn bench_values_mut_0100(b: &mut Bencher) {
            do_bench_values_mut(b, 100);
        }

        #[bench]
        fn bench_values_mut_1000(b: &mut Bencher) {
            do_bench_values_mut(b, 1000);
        }
    };
}
