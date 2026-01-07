use super::*;
use std::alloc::Global;
use std::ptr::NonNull;

#[derive(Clone)]
struct FailAllocator;

unsafe impl Allocator for FailAllocator {
    fn allocate(&self, _layout: Layout) -> Result<NonNull<[u8]>, core::alloc::AllocError> {
        Err(core::alloc::AllocError)
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
}

#[derive(Debug)]
struct Temp {
    value: i32,
}

impl fmt::Display for Temp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Temp {{ value: {:} }}", self.value)
    }
}

impl Temp {
    pub fn get(&self) -> i32 {
        self.value
    }
}

#[derive(Debug)]
struct ZSTTemp;

#[derive(Debug, Clone)]
struct ZSTTempC;

/// UT test cases for `new`.
///
/// # Brief
/// 1. create Box success.
/// 2. create Box failed.
#[test]
fn ut_box_new() {
    let b = AllocatorBox::new(1, Global);
    assert!(b.is_ok());

    let b = AllocatorBox::new(1, FailAllocator);
    assert!(b.is_err());
}

/// UT test cases for `new`.
///
/// # Brief
/// 1. create zst Box success.
/// 2. create zst Box failed.
#[test]
fn ut_box_new_zst() {
    let b = AllocatorBox::new(ZSTTemp, Global);
    assert!(b.is_ok());

    let b = AllocatorBox::new(ZSTTemp, FailAllocator);
    assert!(b.is_ok())
}

/// UT test cases for `into_inner`.
///
/// # Brief
/// 1. create Box.
/// 2. get inner of box.
#[test]
fn ut_box_into_inner() {
    let b = AllocatorBox::new(Temp { value: 10 }, Global).unwrap();
    let t = AllocatorBox::into_inner(b);
    assert_eq!(t.value, 10);
}

/// UT test cases for `into_inner`.
///
/// # Brief
/// 1. create zst Box.
/// 2. get inner of box.
#[test]
fn ut_box_into_inner_zst() {
    let b = AllocatorBox::new(ZSTTemp, Global).unwrap();
    let _t: ZSTTemp = AllocatorBox::into_inner(b);
}

fn create_raw<T>() -> *mut T {
    let layout = Layout::new::<T>();
    Global.allocate(layout).unwrap().cast::<T>().as_ptr()
}

/// UT test cases for `from_raw`.
///
/// # Brief
/// 1. create raw ptr.
/// 2. create box from raw.
#[test]
fn ut_box_from_raw() {
    let ptr1 = create_raw::<Temp>();
    unsafe {
        (*ptr1).value = 10;
    }
    let b = unsafe { AllocatorBox::from_raw(ptr1, Global) };
    assert_eq!(b.get(), 10);
}

/// UT test cases for `into_raw`.
///
/// # Brief
/// 1. create box.
/// 2. get raw ptr from box.
#[test]
fn ut_box_into_raw() {
    let b = AllocatorBox::new(Temp { value: 10 }, Global).unwrap();
    let t = AllocatorBox::into_raw(b);
    unsafe {
        assert_eq!((*t).value, 10);
        (*t).value = 20;
        assert_eq!((*t).value, 20);
        Global.deallocate(
            NonNull::new(t).unwrap().cast::<u8>(),
            Layout::for_value_raw(t),
        );
    }
}

/// UT test cases for `into_raw_with_allocator`.
///
/// # Brief
/// 1. create box.
/// 2. get raw ptr and allocator from box.
#[test]
fn ut_box_into_raw_with_allocator() {
    let b = AllocatorBox::new(Temp { value: 10 }, Global).unwrap();
    let (t, mut _alloc) = AllocatorBox::into_raw_with_allocator(b);
    _alloc = Global;
    unsafe {
        assert_eq!((*t).value, 10);
        (*t).value = 20;
        assert_eq!((*t).value, 20);
        Global.deallocate(
            NonNull::new(t).unwrap().cast::<u8>(),
            Layout::for_value_raw(t),
        );
    }
}

/// UT test cases for `allocator`.
///
/// # Brief
/// 1. create box.
/// 2. get allocator from box.
#[test]
fn ut_box_allocator() {
    let b = AllocatorBox::new(Temp { value: 10 }, Global).unwrap();
    let _alloc: &Global = AllocatorBox::allocator(&b);
}

/// UT test cases for `as_ref`.
///
/// # Brief
/// 1. create box.
/// 2. get ref of box.
#[test]
fn ut_box_as_ref() {
    let b = AllocatorBox::new(Temp { value: 10 }, Global).unwrap();
    let r: &Temp = b.as_ref();
    assert_eq!(r.get(), 10);
}

/// UT test cases for `as_mut`.
///
/// # Brief
/// 1. create box.
/// 2. get mut ref of box.
#[test]
fn ut_box_as_mut() {
    let mut b = AllocatorBox::new(Temp { value: 10 }, Global).unwrap();
    let r: &mut Temp = b.as_mut();
    assert_eq!(r.get(), 10);
    r.value = 20;
    assert_eq!(r.get(), 20);
}

/// UT test cases for `deref`.
///
/// # Brief
/// 1. create box.
/// 2. get deref of box.
#[test]
fn ut_box_deref() {
    let b = AllocatorBox::new(Temp { value: 10 }, Global).unwrap();
    let r: &Temp = b.deref();
    assert_eq!(r.get(), 10);
}

/// UT test cases for `deref_mut`.
///
/// # Brief
/// 1. create box.
/// 2. get mut deref of box.
#[test]
fn ut_box_deref_mut() {
    let mut b = AllocatorBox::new(Temp { value: 10 }, Global).unwrap();
    let r: &mut Temp = b.deref_mut();
    assert_eq!(r.get(), 10);
    r.value = 20;
    assert_eq!(r.get(), 20);
}

#[derive(Debug, Clone)]
struct TempC {
    value: i32,
}

/// UT test cases for `safe_clone`.
///
/// # Brief
/// 1. create box.
/// 2. safe_clone box.
#[test]
fn ut_box_safe_clone() {
    let mut b = AllocatorBox::new(TempC { value: 10 }, Global).unwrap();
    let mut clone = b.safe_clone().unwrap();
    assert_eq!(clone.value, 10);
    b.value = 20;
    let _ = clone.safe_clone_from(&b);
    assert_eq!(clone.value, 20);
}

/// UT test cases for `safe_clone`.
///
/// # Brief
/// 1. create zst box.
/// 2. safe_clone box.
#[test]
fn ut_box_safe_clone_zst() {
    let b = AllocatorBox::new(ZSTTempC, FailAllocator).unwrap();
    let clone = b.safe_clone();
    assert!(clone.is_ok());
}

/// UT test cases for `safe_clone`.
///
/// # Brief
/// 1. create unsized array box.
/// 2. safe_clone box.
#[test]
fn ut_box_arr_safe_clone() {
    let ptr = create_raw::<[i32; 9]>() as *mut [i32];
    let arr: &mut [i32];
    unsafe {
        arr = &mut *ptr;
        for (i, item) in arr.iter_mut().enumerate() {
            *item = i as i32 + 1;
        }
    }

    let b = unsafe { AllocatorBox::from_raw(ptr, Global) };
    let mut b2 = b.safe_clone().unwrap();

    assert_eq!(*b2.first().unwrap(), 1);
    assert_eq!(*b2.get(8).unwrap(), 9);
    assert!(b2.get(9).is_none());
    arr[0] = 10;
    assert_eq!(*b2.first().unwrap(), 1);

    let ptr2 = b2.as_ptr();
    let _ = b2.safe_clone_from(&b);
    assert_eq!(*b2.first().unwrap(), 10);
    assert_eq!(b2.as_ptr(), ptr2);
}

/// UT test cases for `safe_clone`.
///
/// # Brief
/// 1. create unsized array zst box.
/// 2. safe_clone box.
#[test]
fn ut_box_arr_safe_clone_zst() {
    let mut arr: [i32; 0] = [];
    let ptr = arr.as_mut() as *mut [i32];
    let b = unsafe { AllocatorBox::from_raw(ptr, Global) };
    let b2 = b.safe_clone();
    assert!(b2.is_ok());

    let b = unsafe { AllocatorBox::from_raw(ptr, FailAllocator) };
    let b2 = b.safe_clone();
    assert!(b2.is_ok());

    let mut arr2 = [ZSTTempC, ZSTTempC, ZSTTempC];
    let ptr2 = arr2.as_mut() as *mut [ZSTTempC];
    let b = unsafe { AllocatorBox::from_raw(ptr2, FailAllocator) };
    let b2 = b.safe_clone();
    assert!(b2.is_ok());
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

/// UT test cases for `safe_clone`.
///
/// # Brief
/// 1. create unsized str box.
/// 2. safe_clone box.
#[test]
fn ut_box_str_safe_clone() {
    let b = create_box_str("hello", Global);

    let b2 = b.safe_clone().unwrap();
    assert_eq!(b2.deref(), "hello");
}

/// UT test cases for `safe_clone`.
///
/// # Brief
/// 1. create unsized str zst box.
/// 2. safe_clone box.
#[test]
fn ut_box_str_safe_clone_zst() {
    let b = create_box_str("", FailAllocator);

    let b2 = b.safe_clone().unwrap();
    assert_eq!(b2.deref(), "");
}

/// UT test cases for `debug`.
///
/// # Brief
/// 1. create box.
/// 2. show debug info of box.
#[test]
fn ut_box_debug() {
    let b = AllocatorBox::new([1, 2, 3, 4], Global).unwrap();
    println!("{:?}", b);
    assert_eq!("[1, 2, 3, 4]", format!("{:?}", b));
}

/// UT test cases for `display`.
///
/// # Brief
/// 1. create box.
/// 2. show display info of box.
#[test]
fn ut_box_display() {
    let b = AllocatorBox::new(Temp { value: 10 }, Global).unwrap();
    println!("{}", b);
    assert_eq!("Temp { value: 10 }", format!("{:?}", b));
}
