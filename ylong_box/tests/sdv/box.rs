#![feature(allocator_api)]
#![feature(ptr_internals)]
#![feature(layout_for_ptr)]

use std::alloc::{AllocError, Allocator, Global, Layout};
use std::ops::Deref;
use std::ptr::{drop_in_place, NonNull};
use ylong_box::*;
use ylong_stdx_common::SafeClone;

struct Temp {
    value: i32,
}

impl Temp {
    pub fn new(value: i32) -> Self {
        Temp { value }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn set_value(&mut self, x: i32) {
        self.value = x;
    }
}

/// SDV test cases for `deref`.
///
/// # Brief
/// 1. create a box
/// 2. get value from box auto deref
#[test]
fn sdv_box_auto_deref() {
    let t = Temp { value: 10 };
    let b = AllocatorBox::new(t, Global).unwrap();

    assert_eq!(b.get_value(), 10);
}

fn create_raw<T>() -> *mut T {
    let layout = Layout::new::<T>();
    Global.allocate(layout).unwrap().cast::<T>().as_ptr()
}

/// SDV test cases for `from_raw`.
///
/// # Brief
/// 1. create a array
/// 2. create box from_raw array ptr
/// 3. check element of box
#[test]
fn sdv_box_from_raw_unsized() {
    let ptr = create_raw::<[i32; 16]>() as *mut [i32];
    for i in 0..16 {
        unsafe {
            (&mut *ptr)[i] = 10;
        }
    }
    unsafe {
        let b = AllocatorBox::from_raw(ptr, Global);
        assert_eq!(b.len(), 16);
        assert_eq!(*b.first().unwrap(), 10);
        assert!(b.get(16).is_none());
    }
}

/// SDV test cases for `from_raw` and `into_raw`.
///
/// # Brief
/// 1. create box of struct
/// 2. box into_raw
/// 3. create box from raw again
/// 4. check drop of box
#[test]
fn sdv_box_into_and_from_raw() {
    // check no memory leak
    static mut TEST_FLAG: usize = 0;

    struct DropCounter {
        count: u32,
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            println!("drop {}", self.count);
            unsafe {
                TEST_FLAG += 1;
            }
        }
    }

    {
        let d = DropCounter { count: 123 };
        let x1 = AllocatorBox::new(d, Global).unwrap();
        let ptr = AllocatorBox::into_raw(x1);
        let _x2 = unsafe { AllocatorBox::from_raw(ptr, Global) };
    }
    unsafe { assert_eq!(TEST_FLAG, 1) };
}

/// SDV test cases for `from_raw` and `into_raw`.
///
/// # Brief
/// 1. create box of struct
/// 2. box leak
/// 3. create box from raw again
/// 4. check drop of box
#[test]
fn sdv_box_leak_and_from_raw() {
    // check no memory leak
    static mut TEST_FLAG: usize = 0;

    struct DropCounter {
        count: u32,
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            println!("drop {}", self.count);
            unsafe {
                TEST_FLAG += 1;
            }
        }
    }

    {
        let d = DropCounter { count: 123 };
        let x1 = AllocatorBox::new(d, Global).unwrap();
        let ptr = AllocatorBox::leak(x1) as *mut DropCounter;
        let _x2 = unsafe { AllocatorBox::from_raw(ptr, Global) };
    }
    unsafe { assert_eq!(TEST_FLAG, 1) };
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Dummy {
    _data: u8,
}

/// SDV test cases for `safe_clone`.
///
/// # Brief
/// 1. create box of array
/// 2. box clone
/// 3. check element of box
#[test]
fn sdv_box_array_safe_clone_and_clone_from_equivalence() {
    let one = create_raw::<[Dummy; 2]>() as *mut [Dummy];
    unsafe {
        (&mut *one)[0]._data = 42;
        (&mut *one)[1]._data = 42;
    }
    let control = unsafe { AllocatorBox::from_raw(one, Global) };

    let clone = control.safe_clone().unwrap();
    let two = create_raw::<[Dummy; 2]>() as *mut [Dummy];
    unsafe {
        (&mut *two)[0]._data = 84;
        (&mut *two)[1]._data = 84;
    }
    let mut copy = unsafe { AllocatorBox::from_raw(two, Global) };
    copy.safe_clone_from(&control).unwrap();
    assert_eq!(control.deref(), clone.deref());
    assert_eq!(control.deref(), copy.deref());
}

/// SDV test cases for `safe_clone`.
///
/// # Brief
/// 1. create box of array
/// 2. box clone
/// 3. check element of box
#[test]
fn sdv_box_safe_clone_from_ptr_stability() {
    let one = create_raw::<[Dummy; 2]>() as *mut [Dummy];
    unsafe {
        (&mut *one)[0]._data = 42;
        (&mut *one)[1]._data = 42;
    }
    let control = unsafe { AllocatorBox::from_raw(one, Global) };

    let two = create_raw::<[Dummy; 2]>() as *mut [Dummy];
    unsafe {
        (&mut *two)[0]._data = 84;
        (&mut *two)[1]._data = 84;
    }
    let mut copy = unsafe { AllocatorBox::from_raw(two, Global) };

    let copy_raw = copy.as_ptr() as usize;
    let _ = copy.safe_clone_from(&control);
    assert_eq!(copy.as_ptr() as usize, copy_raw);
}

fn create_box_str<A: Allocator>(string: &'static str, alloc: A) -> AllocatorBox<str, A> {
    let slice = string.as_bytes();
    let layout = Layout::array::<u8>(slice.len()).unwrap();
    let mem = Global.allocate(layout).unwrap();
    let ptr = mem.cast::<u8>().as_ptr();

    unsafe {
        ptr.copy_from(slice.as_ptr(), slice.len());
        AllocatorBox::from_raw(mem.as_ptr() as *mut str, alloc)
    }
}

/// SDV test cases for `safe_clone`.
///
/// # Brief
/// 1. create box of str
/// 2. box clone
/// 3. check element of box
#[test]
fn sdv_box_str() {
    let b = create_box_str("hello", Global);
    assert_eq!(b.deref(), "hello");
    let mut b2 = b.safe_clone().unwrap();
    assert_eq!(b.deref(), b2.deref());

    let _ = b2.safe_clone_from(&b);
    assert_eq!(b.deref(), b2.deref());

    let mut b3 = create_box_str("world", Global);
    assert_eq!(b3.deref(), "world");
    assert_ne!(b3.deref(), b.deref());
    let _ = b3.safe_clone_from(&b);
    assert_eq!(b3.deref(), b.deref());
}

/// SDV test cases for `deref`.
///
/// # Brief
/// 1. create box
/// 2. set box value
/// 3. check element of box
#[test]
fn sdv_box_deref_lval() {
    let mut x = Box::new(Temp::new(5));
    x.set_value(1000);
    assert_eq!(x.get_value(), 1000);
}

/// SDV test cases for `deref`.
///
/// # Brief
/// 1. create box
/// 2. check drop of box
#[test]
fn sdv_box_drop() {
    static mut TEST_FLAG: usize = 0;
    static mut TEST_FLAG2: usize = 0;
    struct DropCounter2 {
        count: u32,
    }

    impl Drop for DropCounter2 {
        fn drop(&mut self) {
            println!("drop {}", self.count);
            unsafe {
                TEST_FLAG2 += 1;
            }
        }
    }

    struct DropCounter {
        _v: DropCounter2,
        count: u32,
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            println!("drop {}", self.count);
            unsafe {
                TEST_FLAG += 1;
            }
        }
    }

    let d1 = DropCounter {
        _v: DropCounter2 { count: 0 },
        count: 0,
    };
    let b1 = AllocatorBox::new(d1, Global);
    drop(b1);

    {
        let d2 = DropCounter {
            _v: DropCounter2 { count: 0 },
            count: 0,
        };
        let _b1 = AllocatorBox::new(d2, Global);
    }

    unsafe {
        println!("TEST_FLAG {}", TEST_FLAG);
        println!("TEST_FLAG2 {}", TEST_FLAG2);
        assert_eq!(TEST_FLAG, 2);
        assert_eq!(TEST_FLAG2, 2);
    }
}

/// SDV test cases for `leak`.
///
/// # Brief
/// 1. create box
/// 2. leak box
/// 3. check drop of value
#[test]
fn sdv_box_drop_leak() {
    static mut TEST_FLAG: usize = 0;
    static mut TEST_FLAG2: usize = 0;
    struct DropCounter2 {
        count: u32,
    }

    impl Drop for DropCounter2 {
        fn drop(&mut self) {
            println!("drop {}", self.count);
            unsafe {
                TEST_FLAG2 += 1;
            }
        }
    }

    struct DropCounter {
        _v: DropCounter2,
        count: u32,
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            println!("drop {}", self.count);
            unsafe {
                TEST_FLAG += 1;
            }
        }
    }
    let leak;
    {
        let d = DropCounter {
            _v: DropCounter2 { count: 0 },
            count: 0,
        };
        let b1 = AllocatorBox::new(d, Global).unwrap();
        leak = AllocatorBox::leak(b1);
    }

    unsafe {
        println!("TEST_FLAG {}", TEST_FLAG);
        println!("TEST_FLAG2 {}", TEST_FLAG2);
        assert_eq!(TEST_FLAG, 0);
        assert_eq!(TEST_FLAG2, 0);
    }
    let layout = Layout::new::<Layout>();
    let ptr = NonNull::from(leak).cast::<u8>();
    unsafe {
        Global.deallocate(ptr, layout);
    }
}

/// SDV test cases for `leak`.
///
/// # Brief
/// 1. create box
/// 2. leak box
/// 3. check drop of value
/// 4. check no drop of allocator
#[test]
fn sdv_box_drop_leak_with_allocator() {
    static mut TEST_FLAG: usize = 0;
    static mut TEST_FLAG2: usize = 0;

    struct DropCounter {
        count: u32,
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            println!("drop {}", self.count);
            unsafe {
                TEST_FLAG += 1;
            }
        }
    }

    struct DropAllocator {
        _val: i32,
    }

    unsafe impl Allocator for DropAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            Global.allocate(layout)
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            Global.deallocate(ptr, layout)
        }
    }

    impl Drop for DropAllocator {
        fn drop(&mut self) {
            unsafe {
                TEST_FLAG2 += 1;
            }
        }
    }

    let leak;
    {
        let d = DropCounter { count: 0 };
        let alloc = DropAllocator { _val: 123 };
        let b1 = AllocatorBox::new(d, alloc).unwrap();
        leak = AllocatorBox::leak_with_allocator(b1);
    }

    unsafe {
        println!("TEST_FLAG {}", TEST_FLAG);
        println!("TEST_FLAG2 {}", TEST_FLAG2);
        assert_eq!(TEST_FLAG, 0);
        assert_eq!(TEST_FLAG2, 0);
    }

    unsafe {
        drop_in_place(leak.0 as *mut DropCounter);
        let layout = Layout::new::<Layout>();
        let ptr = NonNull::from(leak.0).cast::<u8>();
        Global.deallocate(ptr, layout);
        drop_in_place(leak.1 as *mut DropAllocator);
    }
    unsafe {
        println!("TEST_FLAG {}", TEST_FLAG);
        println!("TEST_FLAG2 {}", TEST_FLAG2);
        assert_eq!(TEST_FLAG, 1);
        assert_eq!(TEST_FLAG2, 1);
    }
}

#[derive(Debug, Clone)]
struct TempC {
    _value: i32,
}

#[derive(Clone)]
struct ConditionFailAllocator {
    cnt: *mut i32,
}

unsafe impl Allocator for ConditionFailAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, core::alloc::AllocError> {
        unsafe {
            if *self.cnt == 1 {
                Global.allocate(layout)
            } else {
                Err(core::alloc::AllocError)
            }
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe { Global.deallocate(ptr, layout) }
    }
}

/// SDV test cases for `safe_clone`.
///
/// # Brief
/// 1. create box
/// 2. safe_clone box
/// 3. check clone failed
#[test]
#[allow(unused_assignments)] // For ConditionFailAllocator, use raw ptr of x is thought unused by compiler.
fn sdv_box_safe_clone_failed() {
    let mut x = 1;
    let ptr = &mut x as *mut i32;
    let alloc = ConditionFailAllocator { cnt: ptr };

    let b = AllocatorBox::new(TempC { _value: 10 }, alloc).unwrap();
    x = 0;
    let clone = b.safe_clone();
    assert!(clone.is_err());
}

/// SDV test cases for `safe_clone`.
///
/// # Brief
/// 1. create 2 box for different length
/// 2. safe_clone box
/// 3. check clone success
#[test]
fn sdv_box_arr_safe_clone_from_diff_len() {
    let ptr1 = create_raw::<[i32; 9]>() as *mut [i32];
    let arr1: &mut [i32];
    unsafe {
        arr1 = &mut *ptr1;
        for (i, item) in arr1.iter_mut().enumerate() {
            *item = i as i32 + 1;
        }
    }
    let b1 = unsafe { AllocatorBox::from_raw(ptr1, Global) };

    let ptr2 = create_raw::<[i32; 1]>() as *mut [i32];
    let arr2: &mut [i32];
    unsafe {
        arr2 = &mut *ptr2;
        arr2[0] = 100;
    }
    let mut b2 = unsafe { AllocatorBox::from_raw(ptr2, Global) };
    assert_eq!(b2.as_ptr(), ptr2 as *const i32);
    let r = b2.safe_clone_from(&b1);
    assert!(r.is_ok());
    assert_ne!(b2.as_ptr(), ptr2 as *const i32);
    assert_eq!(b2.len(), 9);
    assert_eq!(b2.deref(), b1.deref());
}

/// SDV test cases for `safe_clone`.
///
/// # Brief
/// 1. create box for unsized array
/// 2. safe_clone box
/// 3. check clone failed
#[test]
#[allow(unused_assignments)] // For ConditionFailAllocator, use raw ptr of x is thought unused by compiler.
fn sdv_box_arr_safe_clone_failed() {
    let mut x = 1;
    let ptr = &mut x as *mut i32;
    let alloc = ConditionFailAllocator { cnt: ptr };

    let arr_ptr = create_raw::<[i32; 9]>() as *mut [i32];
    let arr: &mut [i32];
    unsafe {
        arr = &mut *arr_ptr;
        for (i, item) in arr.iter_mut().enumerate() {
            *item = i as i32 + 1;
        }
    }

    let b = unsafe { AllocatorBox::from_raw(arr_ptr, alloc) };
    x = 0;
    let clone = b.safe_clone();
    assert!(clone.is_err());
}

/// SDV test cases for `safe_clone`.
///
/// # Brief
/// 1. create box for unsized str
/// 2. safe_clone box
/// 3. check clone failed
#[test]
#[allow(unused_assignments)] // For ConditionFailAllocator, use raw ptr of x is thought unused by compiler.
fn sdv_box_str_safe_clone_failed() {
    let mut x = 1;
    let ptr = &mut x as *mut i32;
    let alloc = ConditionFailAllocator { cnt: ptr };

    let b = create_box_str("hello", alloc);
    x = 0;
    let clone = b.safe_clone();
    assert!(clone.is_err());
}

/// SDV test cases for `drop`.
///
/// # Brief
/// 1. create box for unsized array
/// 2. check drop of box
#[test]
fn sdv_box_arr_drop() {
    static mut TEST_FLAG: usize = 0;

    struct DropCounter {
        count: u32,
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            println!("drop {}", self.count);
            unsafe {
                TEST_FLAG += 1;
            }
        }
    }

    let arr_ptr = create_raw::<[DropCounter; 4]>() as *mut [DropCounter];
    let arr: &mut [DropCounter];
    unsafe {
        arr = &mut *arr_ptr;
        for (i, item) in arr.iter_mut().enumerate() {
            *item = DropCounter { count: i as u32 };
        }
        TEST_FLAG = 0;
    }
    let b1 = unsafe { AllocatorBox::from_raw(arr_ptr, Global) };
    drop(b1);

    unsafe {
        println!("TEST_FLAG {}", TEST_FLAG);
        assert_eq!(TEST_FLAG, 4);
    }
}

/// SDV test cases for `may_dangle`.
///
/// # Brief
/// 1. create box of reference
/// 2. check compile success
#[test]
fn sdv_box_may_dangle() {
    let t = Temp { value: 123 };
    impl Drop for Temp {
        fn drop(&mut self) {
            println!("value : {}", self.value);
        }
    }

    let _b = AllocatorBox::new(&t, Global);
    drop(t);
}

/// SDV test cases for `into_inner`.
///
/// # Brief
/// 1. create box
/// 2. box into inner
/// 3. create box from raw
/// 4. check drop of box value
#[test]
fn sdv_box_into_inner_drop() {
    static mut TEST_FLAG1: usize = 0;
    static mut TEST_FLAG2: usize = 0;

    #[derive(Clone)]
    struct DropAllocator {
        _val: i32,
    }

    unsafe impl Allocator for DropAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            Global.allocate(layout)
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            Global.deallocate(ptr, layout)
        }
    }

    impl Drop for DropAllocator {
        fn drop(&mut self) {
            unsafe {
                TEST_FLAG2 += 1;
            }
        }
    }

    struct DropCounter {
        count: u32,
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            println!("drop {}", self.count);
            unsafe {
                TEST_FLAG1 += 1;
            }
        }
    }

    {
        let d = DropCounter { count: 123 };
        let alloc = DropAllocator { _val: 123 };
        let b = AllocatorBox::new(d, alloc).unwrap();
        let d2 = AllocatorBox::into_inner(b);
        assert_eq!(d2.count, 123);

        unsafe {
            println!("TEST_FLAG1 {}", TEST_FLAG1);
            assert_eq!(TEST_FLAG1, 0);
            println!("TEST_FLAG2 {}", TEST_FLAG2);
            assert_eq!(TEST_FLAG2, 1);
            drop(d2);
            println!("TEST_FLAG1 {}", TEST_FLAG1);
            assert_eq!(TEST_FLAG1, 1);
        }
    }
}

struct TempAllocator;

unsafe impl Allocator for TempAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Global.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Global.deallocate(ptr, layout)
    }
}

/// SDV test cases for `into_raw`.
///
/// # Brief
/// 1. create box
/// 2. box into raw
/// 3. check allocator is dropped
#[test]
fn sdv_box_into_raw_check_allocator_drop() {
    static mut TEST_FLAG: usize = 0;

    struct DropAllocator {
        _val: i32,
    }

    unsafe impl Allocator for DropAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            Global.allocate(layout)
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            Global.deallocate(ptr, layout)
        }
    }

    impl Drop for DropAllocator {
        fn drop(&mut self) {
            unsafe {
                TEST_FLAG += 1;
            }
        }
    }

    let v;
    {
        let alloc = DropAllocator { _val: 123 };
        let b = AllocatorBox::new(456, alloc).unwrap();
        v = AllocatorBox::into_raw(b);
    }
    unsafe {
        assert_eq!(TEST_FLAG, 1);
        Global.deallocate(
            NonNull::new(v).unwrap().cast::<u8>(),
            Layout::for_value_raw(v),
        );
    }
}

/// SDV test cases for `into_raw_with_allocator`.
///
/// # Brief
/// 1. create box
/// 2. box into raw with allocator
#[test]
fn sdv_box_into_raw_with_allocator() {
    let b = AllocatorBox::new(Temp { value: 10 }, TempAllocator).unwrap();
    let (t, mut _alloc) = AllocatorBox::into_raw_with_allocator(b);
    _alloc = TempAllocator;
    unsafe {
        assert_eq!((*t).value, 10);
        (*t).value = 20;
        assert_eq!((*t).value, 20);
        TempAllocator.deallocate(
            NonNull::new(t).unwrap().cast::<u8>(),
            Layout::for_value_raw(t),
        );
    }
}

#[derive(Debug)]
enum List<T> {
    Cons(T, AllocatorBox<List<T>, Global>),
    Nil,
}

/// SDV test cases for `recursive`.
///
/// # Brief
/// 1. create box of recursive.
/// 2. check box value.
#[test]
fn sdv_box_recursive() {
    let list: List<i32> = List::Cons(
        1,
        AllocatorBox::new(
            List::Cons(2, AllocatorBox::new(List::Nil, Global).unwrap()),
            Global,
        )
        .unwrap(),
    );
    println!("{list:?}");
    match list {
        List::Cons(v, node) => {
            assert_eq!(v, 1);
            match AllocatorBox::into_inner(node) {
                List::Cons(v, node) => {
                    assert_eq!(v, 2);
                    match AllocatorBox::into_inner(node) {
                        List::Cons(_v, _node) => {
                            unreachable!()
                        }
                        List::Nil => {}
                    }
                }
                List::Nil => {
                    unreachable!()
                }
            }
        }
        List::Nil => {
            unreachable!()
        }
    }
}

/// SDV test cases for `safe_clone`.
///
/// # Brief
/// 1. create box of allocator.
/// 2. check drop of allocator.
#[test]
fn sdv_box_clone_allocator() {
    static mut TEST_FLAG: usize = 0;

    #[derive(Clone)]
    struct DropAllocator;

    unsafe impl Allocator for DropAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            Global.allocate(layout)
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            Global.deallocate(ptr, layout)
        }
    }

    impl Drop for DropAllocator {
        fn drop(&mut self) {
            unsafe {
                TEST_FLAG += 1;
            }
        }
    }

    {
        let _b = AllocatorBox::new(1, DropAllocator);
    }

    unsafe {
        assert_eq!(TEST_FLAG, 1);
    }

    {
        let b = AllocatorBox::new(1, DropAllocator).unwrap();
        let _b2 = b.safe_clone();
    }

    unsafe {
        assert_eq!(TEST_FLAG, 3);
    }
}
