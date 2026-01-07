#![feature(allocator_api)]
#![feature(slice_pattern)]
#![feature(iter_advance_by)]
#![feature(strict_provenance)]
#![allow(clippy::reversed_empty_ranges)]
#![allow(clippy::vtable_address_comparisons)]

use core::slice::SlicePattern;
use std::alloc::{Allocator, Global, Layout, System};
use std::cell::Cell;
use std::mem::size_of;
use std::num::NonZeroUsize;
use std::panic::catch_unwind;
use std::ptr::NonNull;
use std::{hint, mem};
use ylong_vec::vec::{intoiter, FixedCapVec};

struct DropCounter<'a> {
    count: &'a mut u32,
}

impl Drop for DropCounter<'_> {
    fn drop(&mut self) {
        *self.count += 1;
    }
}

fn create_vec_from_slice<T>(arr: &[T], cap: usize) -> FixedCapVec<T, Global>
where
    T: Clone,
{
    let mut v = FixedCapVec::new(cap, Global).unwrap();
    for e in arr {
        let _ = v.push(e.clone());
    }
    v
}

fn create_vec_from_arr<T, const N: usize>(arr: [T; N], cap: usize) -> FixedCapVec<T, Global> {
    let mut v = FixedCapVec::new(cap, Global).unwrap();
    for e in arr {
        let _ = v.push(e);
    }
    v
}

/// SDV test cases for vec size.
///
/// # Brief
/// 1. the size of vec is always 3 * usize.
#[test]
fn test_small_vec_struct() {
    assert_eq!(size_of::<FixedCapVec<u8, Global>>(), size_of::<usize>() * 3);
}

/// SDV test cases for vec size.
///
/// # Brief
/// 1. create 2 vec, and drop them
/// 2. check elements of the 2 vecs are dropped.
#[test]
fn test_double_drop() {
    struct TwoVec<T> {
        x: FixedCapVec<T, Global>,
        y: FixedCapVec<T, Global>,
    }

    let (mut count_x, mut count_y) = (0, 0);
    {
        let mut tv = TwoVec {
            x: FixedCapVec::new(1, Global).unwrap(),
            y: FixedCapVec::new(1, Global).unwrap(),
        };
        let _ = tv.x.push(DropCounter {
            count: &mut count_x,
        });
        let _ = tv.y.push(DropCounter {
            count: &mut count_y,
        });

        drop(tv.x);

        // Here tv goes out of scope, tv.y should be dropped, but not tv.x.
    }

    assert_eq!(count_x, 1);
    assert_eq!(count_y, 1);
}

/// SDV test cases for vec `capacity`.
///
/// # Brief
/// 1. create vec of ZST.
/// 2. the capacity of vec of ZST is always usize::MAX.
#[test]
fn test_zst_capacity() {
    assert_eq!(
        FixedCapVec::<(), Global>::new(1, Global)
            .unwrap()
            .capacity(),
        usize::MAX
    );
}

/// SDV test cases for vec `index`.
///
/// # Brief
/// 1. create vec.
/// 2. use index to access element.
#[test]
fn test_indexing() {
    let mut v = FixedCapVec::<isize, Global>::new(2, Global).unwrap();
    let _ = v.push(10);
    let _ = v.push(20);
    assert_eq!(v[0], 10);
    assert_eq!(v[1], 20);
    let mut x: usize = 0;
    assert_eq!(v[x], 10);
    assert_eq!(v[x + 1], 20);
    x += 1;
    assert_eq!(v[x], 20);
    assert_eq!(v[x - 1], 10);
}

/// SDV test cases for vec `debug`.
///
/// # Brief
/// 1. create vec.
/// 2. print vec debug message.
#[test]
fn test_debug_fmt_std() {
    let vec1 = FixedCapVec::<isize, Global>::new(0, Global).unwrap();
    assert_eq!("[]", format!("{:?}", vec1));

    let mut vec2 = FixedCapVec::<i32, Global>::new(2, Global).unwrap();
    let _ = vec2.push(0);
    let _ = vec2.push(1);
    assert_eq!("[0, 1]", format!("{:?}", vec2));

    let slice: &[isize] = &[4, 5];
    assert_eq!("[4, 5]", format!("{slice:?}"));
}

/// SDV test cases for vec `push`.
///
/// # Brief
/// 1. create vec.
/// 2. push any element.
#[test]
fn test_push_std() {
    let mut v = FixedCapVec::<i32, Global>::new(3, Global).unwrap();
    let _ = v.push(1);
    assert_eq!(v.as_slice(), [1]);
    let _ = v.push(2);
    assert_eq!(v.as_slice(), [1, 2]);
    let _ = v.push(3);
    assert_eq!(v.as_slice(), [1, 2, 3]);
}

/// SDV test cases for vec `extend_from_slice`.
///
/// # Brief
/// 1. create vec.
/// 2. vec extend_from_slice.
#[test]
fn test_extend_from_slice() {
    let a = create_vec_from_slice(&[1, 2, 3, 4, 5], 10);
    let b = create_vec_from_slice(&[6, 7, 8, 9, 0], 5);

    let mut v = a;

    let _ = v.extend_from_slice(&b);

    assert_eq!(v.as_slice(), [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
}

/// SDV test cases for vec `ref`.
///
/// # Brief
/// 1. create vec.
/// 2. get slice of vec.
#[test]
fn test_slice_from_ref() {
    let values = create_vec_from_slice(&[1, 2, 3, 4, 5], 5);
    let slice = &values[1..3];

    assert_eq!(slice, [2, 3]);
}

/// SDV test cases for vec `mut_ref`.
///
/// # Brief
/// 1. create vec.
/// 2. get mut slice of vec.
#[test]
fn test_slice_from_mut() {
    let mut values = create_vec_from_slice(&[1, 2, 3, 4, 5], 5);
    {
        let slice = &mut values[2..];
        assert!(slice == [3, 4, 5]);
        for p in slice {
            *p += 2;
        }
    }

    assert_eq!(values.as_slice(), [1, 2, 5, 6, 7]);
}

/// SDV test cases for vec `mut_ref`.
///
/// # Brief
/// 1. create vec.
/// 2. get mut slice of vec.
#[test]
fn test_slice_to_mut() {
    let mut values = create_vec_from_slice(&[1, 2, 3, 4, 5], 5);
    {
        let slice = &mut values[..2];
        assert!(slice == [1, 2]);
        for p in slice {
            *p += 1;
        }
    }

    assert_eq!(values.as_slice(), [2, 3, 3, 4, 5]);
}

/// SDV test cases for vec `split_at_mut`.
///
/// # Brief
/// 1. create vec.
/// 2. split_at_mut vec.
#[test]
fn test_split_at_mut() {
    let mut values = create_vec_from_slice(&[1, 2, 3, 4, 5], 5);
    {
        let (left, right) = values.split_at_mut(2);
        {
            let left: &[_] = left;
            assert!(left[..left.len()] == [1, 2]);
        }
        for p in left {
            *p += 1;
        }

        {
            let right: &[_] = right;
            assert!(right[..right.len()] == [3, 4, 5]);
        }
        for p in right {
            *p += 2;
        }
    }

    assert_eq!(values.as_slice(), [2, 3, 5, 6, 7]);
}

/// SDV test cases for vec zst.
///
/// # Brief
/// 1. create vec of zst.
/// 2. push zst
/// 3. pop zst
/// 4. get len of zst vec
/// 5. iter of zst
#[test]
fn zero_sized_values() {
    let mut v = FixedCapVec::new(2, Global).unwrap();
    assert_eq!(v.len(), 0);
    let _ = v.push(());
    assert_eq!(v.len(), 1);
    let _ = v.push(());
    assert_eq!(v.len(), 2);
    assert_eq!(v.pop(), Some(()));
    assert_eq!(v.pop(), Some(()));
    assert_eq!(v.pop(), None);

    assert_eq!(v.iter().count(), 0);
    let _ = v.push(());
    assert_eq!(v.iter().count(), 1);
    let _ = v.push(());
    assert_eq!(v.iter().count(), 2);

    for &() in &v {}

    // ZST capacity is always usize::MAX
    assert_eq!(v.iter_mut().count(), 2);
    let _ = v.push(());
    assert_eq!(v.iter_mut().count(), 3);
    let _ = v.push(());
    assert_eq!(v.iter_mut().count(), 4);

    for &mut () in &mut v {}
}

/// SDV test cases for vec cmp.
///
/// # Brief
/// 1. create vec.
/// 2. cmp vec with slice
#[test]
fn test_cmp() {
    let x: &[isize] = &[1, 2, 3, 4, 5, 6];
    let cmp: &[isize] = &[1, 2, 3, 4, 5, 6];
    assert_eq!(x, cmp);
    let cmp: &[isize] = &[3, 4, 5, 6];
    assert_eq!(&x[2..], cmp);
    let cmp: &[isize] = &[1, 2, 3];
    assert_eq!(&x[..3], cmp);
    let cmp: &[isize] = &[2, 3, 4];
    assert_eq!(&x[1..4], cmp);

    let x: FixedCapVec<isize, _> = create_vec_from_slice(&[1, 2, 3, 4, 5, 6], 6);
    let cmp: &[isize] = &[1, 2, 3, 4, 5, 6];
    assert_eq!(&x[..], cmp);
    let cmp: &[isize] = &[3, 4, 5, 6];
    assert_eq!(&x[2..], cmp);
    let cmp: &[isize] = &[1, 2, 3];
    assert_eq!(&x[..3], cmp);
    let cmp: &[isize] = &[2, 3, 4];
    assert_eq!(&x[1..4], cmp);
}

/// SDV test cases for vec index.
///
/// # Brief
/// 1. create vec.
/// 2. access vec with index.
#[test]
fn test_index() {
    let vec = create_vec_from_slice(&[1, 2, 3], 3);
    assert!(vec[1] == 2);
}

/// SDV test cases for vec index.
///
/// # Brief
/// 1. create vec.
/// 2. access vec with index out of bound.
#[test]
#[should_panic]
fn test_index_out_of_bounds() {
    let vec = create_vec_from_slice(&[1, 2, 3], 3);
    let _ = vec[3];
}

/// SDV test cases for vec index.
///
/// # Brief
/// 1. create vec.
/// 2. access vec with index out of bound.
#[test]
#[should_panic]
fn test_slice_out_of_bounds_1() {
    let x = create_vec_from_slice(&[1, 2, 3, 4, 5], 5);
    let _ = &x[!0..];
}

/// SDV test cases for vec index.
///
/// # Brief
/// 1. create vec.
/// 2. access vec with index out of bound.
#[test]
#[should_panic]
fn test_slice_out_of_bounds_2() {
    let x = create_vec_from_slice(&[1, 2, 3, 4, 5], 5);
    let _ = &x[..6];
}

/// SDV test cases for vec index.
///
/// # Brief
/// 1. create vec.
/// 2. access vec with index out of bound.
#[test]
#[should_panic]
fn test_slice_out_of_bounds_3() {
    let x = create_vec_from_slice(&[1, 2, 3, 4, 5], 5);
    let _ = &x[!0..4];
}

/// SDV test cases for vec index.
///
/// # Brief
/// 1. create vec.
/// 2. access vec with index out of bound.
#[test]
#[should_panic]
fn test_slice_out_of_bounds_4() {
    let x = create_vec_from_slice(&[1, 2, 3, 4, 5], 5);
    let _ = &x[1..6];
}

/// SDV test cases for vec index.
///
/// # Brief
/// 1. create vec.
/// 2. access vec with index out of bound.
#[test]
#[should_panic]
fn test_slice_out_of_bounds_5() {
    let x = create_vec_from_slice(&[1, 2, 3, 4, 5], 5);
    let _ = &x[3..2];
}

/// SDV test cases for vec.
///
/// # Brief
/// 1. create 2 vec.
/// 2. move element from one vec to another vec.
#[test]
fn test_move_items() {
    let vec = create_vec_from_slice(&[1, 2, 3], 3);
    let mut vec2 = create_vec_from_slice(&[], 3);
    for i in vec {
        let _ = vec2.push(i);
    }
    assert_eq!(vec2.as_slice(), [1, 2, 3]);
}

/// SDV test cases for vec.
///
/// # Brief
/// 1. create 2 vec of zst.
/// 2. move element from one vec to another vec.
#[test]
fn test_move_items_zero_sized() {
    let vec = create_vec_from_slice(&[(), (), ()], 3);
    let mut vec2 = create_vec_from_slice(&[], 3);
    for i in vec {
        let _ = vec2.push(i);
    }
    assert_eq!(vec2.as_slice(), [(), (), ()]);
}

/// SDV test cases for IntoIter `as_slice`.
///
/// # Brief
/// 1. create IntoIter.
/// 2. get slice of IntoIter.
#[test]
fn test_into_iter_as_slice() {
    let vec = create_vec_from_slice(&['a', 'b', 'c'], 3);
    let mut into_iter = vec.into_iter();
    assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);
    let _x = into_iter.next().unwrap();
    assert_eq!(into_iter.as_slice(), &['b', 'c']);
    let _x = into_iter.next().unwrap();
    let _x = into_iter.next().unwrap();
    assert_eq!(into_iter.as_slice(), &[]);
}

/// SDV test cases for IntoIter `as_mut_slice`.
///
/// # Brief
/// 1. create IntoIter.
/// 2. get mut slice of IntoIter.
#[test]
fn test_into_iter_as_mut_slice() {
    let vec = create_vec_from_slice(&['a', 'b', 'c', 'd'], 4);
    let mut into_iter = vec.into_iter();
    assert_eq!(into_iter.as_slice(), &['a', 'b', 'c', 'd']);
    into_iter.as_mut_slice()[0] = 'x';
    into_iter.as_mut_slice()[1] = 'y';
    assert_eq!(into_iter.next().unwrap(), 'x');
    assert_eq!(into_iter.as_slice(), &['y', 'c', 'd']);
}

/// SDV test cases for IntoIter `count`.
///
/// # Brief
/// 1. create IntoIter.
/// 2. get count of IntoIter.
#[test]
fn test_into_iter_count() {
    assert_eq!([1, 2, 3].into_iter().count(), 3);

    let vec = create_vec_from_slice(&['a', 'b', 'c'], 3);
    let into_iter = vec.into_iter();
    assert_eq!(into_iter.count(), 3);
}

/// SDV test cases for IntoIter 'drop'.
///
/// # Brief
/// 1. create IntoIter.
/// 2. drop IntoIter.
#[test]
#[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
fn test_into_iter_leak() {
    static mut DROPS: i32 = 0;

    struct Dtemp(bool);

    impl Drop for Dtemp {
        fn drop(&mut self) {
            println!("drop!!!!");
            unsafe {
                DROPS += 1;
            }

            if self.0 {
                panic!("panic in `drop`");
            }
        }
    }

    let v = create_vec_from_arr::<Dtemp, 3>([Dtemp(false), Dtemp(true), Dtemp(false)], 3);
    catch_unwind(move || drop(v.into_iter())).ok();

    assert_eq!(unsafe { DROPS }, 3);
}

/// SDV test cases for IntoIter 'advance_by'.
///
/// # Brief
/// 1. create IntoIter.
/// 2. advance_by IntoIter.
#[test]
fn test_into_iter_advance_by() {
    // the advance here is default implement
    let mut i = create_vec_from_slice(&[1, 2, 3, 4, 5], 5).into_iter();

    assert_eq!(i.advance_by(0), Ok(()));
    assert_eq!(i.as_slice(), [1, 2, 3, 4, 5]);

    assert_eq!(i.advance_by(1), Ok(()));
    assert_eq!(i.as_slice(), [2, 3, 4, 5]);

    assert_eq!(
        i.advance_by(usize::MAX),
        Err(NonZeroUsize::new(usize::MAX - 4).unwrap())
    );

    assert_eq!(i.advance_by(0), Ok(()));

    assert_eq!(i.len(), 0);
}

/// SDV test cases for IntoIter 'drop'.
///
/// # Brief
/// 1. create IntoIter with a ReferenceCountedAllocator.
/// 2. drop IntoIter, Allocator is dropped too.
#[test]
fn test_into_iter_drop_allocator() {
    struct ReferenceCountAllocator<'a>(DropCounter<'a>);

    unsafe impl Allocator for ReferenceCountAllocator<'_> {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, core::alloc::AllocError> {
            System.allocate(layout)
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            unsafe { System.deallocate(ptr, layout) }
        }
    }

    let mut drop_count = 0;

    let allocator = ReferenceCountAllocator(DropCounter {
        count: &mut drop_count,
    });
    let _ = FixedCapVec::<u32, _>::new(1, allocator);
    assert_eq!(drop_count, 1);

    let allocator = ReferenceCountAllocator(DropCounter {
        count: &mut drop_count,
    });
    let _ = FixedCapVec::<u32, _>::new(1, allocator).into_iter();
    assert_eq!(drop_count, 2);
}

/// SDV test cases for IntoIter.
///
/// # Brief
/// 1. create IntoIter of ZST.
/// 2. drop IntoIter.
#[test]
fn test_into_iter_zst() {
    #[derive(Debug, Clone)]
    struct AlignedZstWithDrop([u64; 0]);
    impl Drop for AlignedZstWithDrop {
        fn drop(&mut self) {
            let addr = self as *mut _ as usize;
            assert!(hint::black_box(addr) % mem::align_of::<u64>() == 0);
        }
    }

    const C: AlignedZstWithDrop = AlignedZstWithDrop([0u64; 0]);

    for _ in create_vec_from_arr([C], 0) {}
    for _ in create_vec_from_arr([C; 5], 5).into_iter() {}

    let mut it = create_vec_from_arr([C, C], 2).into_iter();
    assert_eq!(it.advance_by(1), Ok(()));
    drop(it);
}

/// SDV test cases for IntoIter 'covariance'.
///
/// # Brief
/// 1. covariance lifetime of IntoIter.
/// 2. not run, just compilation.
#[allow(dead_code)]
fn assert_covariance() {
    fn into_iter<'new>(
        i: intoiter::IntoIter<&'static str, Global>,
    ) -> intoiter::IntoIter<&'new str, Global> {
        i
    }
}

/// SDV test cases for vec.
///
/// # Brief
/// 1. create vec.
/// 2. check address of vec is aligned.
#[test]
fn overaligned_allocations() {
    #[repr(align(256))]
    struct Foo(usize);
    for i in 0..0x1000 {
        let v = create_vec_from_arr([Foo(273)], i + 1);
        assert!(v[0].0 == 273);
        assert!(v.as_ptr() as usize & 0xff == 0);
    }
}

/// SDV test cases for vec.
///
/// # Brief
/// 1. create vec.
/// 2. adding and removing elements does not invalidate references into the vector.
#[test]
fn test_stable_pointers() {
    let mut v = FixedCapVec::new(128, Global).unwrap();
    assert!(v.push(13).is_ok());

    let v0 = &mut v[0];
    let v0 = unsafe { &mut *(v0 as *mut _) };

    assert!(v.push(1).is_ok());
    assert!(v.push(2).is_ok());

    assert_eq!(*v0, 13);
    assert!(v.remove(1).is_ok());
    v.pop().unwrap();
    assert_eq!(*v0, 13);
    assert!(v.push(1).is_ok());
    assert_eq!(v.len(), 2);
    assert_eq!(*v0, 13);

    // Extending
    assert!(v.extend_from_slice(&[1, 2]).is_ok());
    assert_eq!(*v0, 13);

    *v0 -= 13;
    assert_eq!(v[0], 0);
}

/// SDV test cases for vec.
///
/// # Brief
/// 1. create vec.
/// 2. push null_raw_fat_pointer.
#[test]
fn vec_macro_repeating_null_raw_fat_pointer() {
    let raw_dyn_ptr = &mut (|| ()) as &mut dyn Fn() as *mut dyn Fn();
    let vtable = dbg!(ptr_metadata(raw_dyn_ptr));
    let null_raw_dyn_ptr = ptr_from_raw_parts(std::ptr::null_mut(), vtable);
    assert!(null_raw_dyn_ptr.is_null());

    let vec = create_vec_from_arr([null_raw_dyn_ptr; 1], 1);
    dbg!(ptr_metadata(vec[0]));
    assert!(vec[0] == null_raw_dyn_ptr);

    fn ptr_from_raw_parts(data: *mut (), vtable: *mut ()) -> *mut dyn Fn() {
        unsafe { std::mem::transmute::<DynRepr, *mut dyn Fn()>(DynRepr { data, vtable }) }
    }

    fn ptr_metadata(ptr: *mut dyn Fn()) -> *mut () {
        unsafe { std::mem::transmute::<*mut dyn Fn(), DynRepr>(ptr).vtable }
    }

    #[repr(C)]
    struct DynRepr {
        data: *mut (),
        vtable: *mut (),
    }
}

/// SDV test cases for vec.
///
/// # Brief
/// 1. create vec.
/// 2. push and set Cell.
#[test]
fn test_vec_cycle() {
    #[derive(Debug)]
    struct C<'a> {
        v: FixedCapVec<Cell<Option<&'a C<'a>>>, Global>,
    }

    impl<'a> C<'a> {
        fn new() -> C<'a> {
            C {
                v: FixedCapVec::new(6, Global).unwrap(),
            }
        }
    }

    let mut cc1 = C::new();
    let mut cc2 = C::new();
    let mut cc3 = C::new();

    // Push
    let _ = cc1.v.push(Cell::new(None));
    let _ = cc1.v.push(Cell::new(None));

    let _ = cc2.v.push(Cell::new(None));
    let _ = cc2.v.push(Cell::new(None));

    let _ = cc3.v.push(Cell::new(None));
    let _ = cc3.v.push(Cell::new(None));

    // Set
    cc1.v[0].set(Some(&cc2));
    cc1.v[1].set(Some(&cc3));

    cc2.v[0].set(Some(&cc2));
    cc2.v[1].set(Some(&cc3));

    cc3.v[0].set(Some(&cc1));
    cc3.v[1].set(Some(&cc2));
}

/// SDV test cases for vec.
///
/// # Brief
/// 1. create vec.
/// 2. push and set Refs that cycle wrapped.
#[test]
fn test_vec_cycle_wrapped() {
    struct Refs<'a> {
        v: FixedCapVec<Cell<Option<&'a Cyc<'a>>>, Global>,
    }

    struct Cyc<'a> {
        refs: Refs<'a>,
    }

    impl<'a> Refs<'a> {
        fn new() -> Refs<'a> {
            Refs {
                v: FixedCapVec::new(6, Global).unwrap(),
            }
        }
    }

    impl<'a> Cyc<'a> {
        fn new() -> Cyc<'a> {
            Cyc { refs: Refs::new() }
        }
    }

    let mut cyc1 = Cyc::new();
    let mut cyc2 = Cyc::new();
    let mut cyc3 = Cyc::new();

    let _ = cyc1.refs.v.push(Cell::new(None));
    let _ = cyc1.refs.v.push(Cell::new(None));
    let _ = cyc2.refs.v.push(Cell::new(None));
    let _ = cyc2.refs.v.push(Cell::new(None));
    let _ = cyc3.refs.v.push(Cell::new(None));
    let _ = cyc3.refs.v.push(Cell::new(None));

    cyc1.refs.v[0].set(Some(&cyc2));
    cyc1.refs.v[1].set(Some(&cyc3));
    cyc2.refs.v[0].set(Some(&cyc2));
    cyc2.refs.v[1].set(Some(&cyc3));
    cyc3.refs.v[0].set(Some(&cyc1));
    cyc3.refs.v[1].set(Some(&cyc2));
}

/// SDV test cases for vec.
///
/// # Brief
/// 1. create vec.
/// 2. capacity of zst is usize::MAX.
#[test]
fn test_zero_sized_capacity() {
    for len in [0, 1, 2, 4, 8, 16, 32, 64, 128, 256] {
        let v = FixedCapVec::<(), Global>::new(len, Global).unwrap();
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), usize::MAX);
    }
}

/// SDV test cases for vec.
///
/// # Brief
/// 1. create vec.
/// 2. push zst.
#[test]
fn test_zero_sized_vec_push() {
    const N: usize = 8;

    for len in 0..N {
        let mut tester = FixedCapVec::<(), Global>::new(len, Global).unwrap();
        assert_eq!(tester.len(), 0);
        assert!(tester.capacity() >= len);
        for _ in 0..len {
            assert!(tester.push(()).is_ok());
        }
        assert_eq!(tester.len(), len);
        assert_eq!(tester.iter().count(), len);
        tester.clear();
    }
}

/// SDV test cases for vec.
///
/// # Brief
/// 1. create vec of zst.
/// 2. track ZST allocations and ensure that they all have a matching free.
#[test]
fn test_box_zero_allocator() {
    use core::{alloc::AllocError, cell::RefCell};
    use std::collections::HashSet;

    struct ZstTracker {
        state: RefCell<(HashSet<usize>, usize)>,
    }
    unsafe impl Allocator for ZstTracker {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            let ptr = if layout.size() == 0 {
                let mut state = self.state.borrow_mut();
                let addr = state.1;
                assert!(state.0.insert(addr));
                state.1 += 1;
                std::println!("allocating {addr}");
                std::ptr::invalid_mut(addr)
            } else {
                unsafe { std::alloc::alloc(layout) }
            };
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new(ptr).ok_or(AllocError)?,
                layout.size(),
            ))
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            if layout.size() == 0 {
                let addr = ptr.as_ptr() as usize;
                let mut state = self.state.borrow_mut();
                std::println!("freeing {addr}");
                assert!(state.0.remove(&addr), "ZST free that wasn't allocated");
            } else {
                unsafe { std::alloc::dealloc(ptr.as_ptr(), layout) }
            }
        }
    }

    // start from 100
    let alloc = ZstTracker {
        state: RefCell::new((HashSet::new(), 100)),
    };

    {
        let _v1: FixedCapVec<u8, _> = FixedCapVec::new(100, &alloc).unwrap();
    }

    // Ensure all ZSTs have been freed.
    assert!(alloc.state.borrow().0.is_empty());
}
