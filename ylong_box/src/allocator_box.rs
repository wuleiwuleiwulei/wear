use core::alloc::{Allocator, Layout};
use core::mem::{forget, SizedTypeProperties};
use core::ops::{Deref, DerefMut};
use core::ptr::{drop_in_place, read, slice_from_raw_parts_mut, write, NonNull, Unique};
use core::{fmt, mem};
use ylong_stdx_common::{ContainerError, SafeClone};

/// A pointer type that uniquely owns a heap allocation of type `T`.
/// AllocatorBox is an extension of `AllocatorBox` with customized Allocator.
/// Otherwise, some method such as `new`, returned Result rather than `()`.
pub struct AllocatorBox<T: ?Sized, A: Allocator> {
    ptr: Unique<T>,
    alloc: A,
}

impl<T, A: Allocator> AllocatorBox<T, A> {
    /// Allocates memory on the heap and then places `x` into it.
    ///
    /// This doesn't actually allocate memory if `T` is zero-sized.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use ylong_box::AllocatorBox;
    /// use std::alloc::Global;
    ///
    /// let five = AllocatorBox::new(5, Global);
    /// ```
    pub fn new(x: T, alloc: A) -> Result<Self, ContainerError> {
        if T::IS_ZST {
            return Ok(Self {
                ptr: Unique::dangling(),
                alloc,
            });
        }
        // compiler will check the size is not more than isize::MAX
        let layout = Layout::new::<T>();
        let mem = match alloc.allocate(layout) {
            Ok(ptr) => ptr,
            Err(_) => return Err(ContainerError::AllocFailure(layout)),
        };
        let ptr;
        // SAFETY: mem is NonNull and valid
        unsafe {
            mem.cast::<T>().as_ptr().write(x);
            ptr = Unique::new_unchecked(mem.cast::<T>().as_ptr());
        }

        Ok(Self { ptr, alloc })
    }
}

/// Consumes the `AllocatorBox`, returning the wrapped value.
///
/// # Examples
///
/// ```
/// #![feature(allocator_api)]
/// use ylong_box::AllocatorBox;
/// use std::alloc::Global;
///
/// let five = AllocatorBox::new(5, Global).unwrap();
///
/// assert_eq!(AllocatorBox::into_inner(five), 5);
/// ```
impl<T, A: Allocator> AllocatorBox<T, A> {
    pub fn into_inner(boxed: Self) -> T {
        // SAFETY: boxed is non null and valid
        let t = unsafe { boxed.ptr.as_ptr().read() };
        // SAFETY: boxed is non null and valid
        unsafe {
            // drop this allocator
            let _alloc = read(&boxed.alloc as *const A);
        }
        boxed.free_mem();
        forget(boxed); // note forget
        t
    }
}

impl<T: ?Sized, A: Allocator> AllocatorBox<T, A> {
    /// Constructs a box from a raw pointer.
    ///
    /// After calling this function, the raw pointer is owned by the
    /// resulting `AllocatorBox`. Specifically, the `AllocatorBox` destructor will call
    /// the destructor of `T` and free the allocated memory. For this
    /// to be safe, the memory must have been allocated in accordance
    /// with the `Layout` in `core` used by `AllocatorBox` .
    ///
    /// # Safety
    ///
    /// Casting a box from a raw pointer is unsafe: invalid pointer might cause memory problems.
    /// E.g. if a raw pointer is casted twice to two boxes, when the compiler destroies these two
    /// boxes, there might be double-free issue.
    ///
    /// # Examples
    /// Manually create a `AllocatorBox` from scratch by using the global allocator:
    /// ```
    /// #![feature(allocator_api)]
    /// use ylong_box::AllocatorBox;
    /// use std::alloc::{alloc, Layout, Global};
    ///
    /// unsafe {
    ///     let ptr = alloc(Layout::new::<i32>()) as *mut i32;
    ///     // Just write something for this i32. Write function might not work for other
    ///     // data type; avoid to use it as possible.
    ///     ptr.write(5);
    ///     let x = AllocatorBox::from_raw(ptr, Global);
    /// }
    /// ```
    pub unsafe fn from_raw(raw: *mut T, alloc: A) -> Self {
        Self {
            ptr: Unique::new_unchecked(raw),
            alloc,
        }
    }

    /// Consumes the `AllocatorBox`, returning a wrapped raw pointer.
    ///
    /// Note: This method returns `T`, and `Allocator` is dropped.
    ///
    /// After calling this function, the caller is responsible for the
    /// memory previously managed by the `AllocatorBox`. In particular, the
    /// caller should properly destroy `T` and release the memory, taking
    /// into account the `Layout` used by `AllocatorBox`. The easiest way to
    /// do this is to convert the raw pointer back into a `AllocatorBox` with the
    /// `AllocatorBox::from_raw` function, allowing the `AllocatorBox` destructor to perform
    /// the cleanup.
    ///
    /// # Examples
    /// Manual cleanup by explicitly running the destructor and deallocating
    /// the memory:
    /// ```
    /// #![feature(allocator_api)]
    /// use ylong_box::AllocatorBox;
    /// use std::alloc::{Allocator, Layout, Global};
    /// use std::ptr::{drop_in_place, NonNull};
    ///
    /// let x = AllocatorBox::new(String::from("Hello"), Global).unwrap();
    /// let p = AllocatorBox::into_raw(x);
    /// unsafe {
    ///     drop_in_place(p);
    ///     Global.deallocate(NonNull::new(p as *mut u8).unwrap(), Layout::new::<String>());
    /// }
    /// ```
    pub fn into_raw(b: Self) -> *mut T {
        Self::into_raw_with_allocator(b).0
    }

    /// Consumes the `AllocatorBox`, returning a wrapped raw pointer and the allocator.
    ///
    /// After calling this function, the caller is responsible for the
    /// memory previously managed by the `AllocatorBox`. In particular, the
    /// caller should properly destroy `T` and release the memory, taking
    /// into account the `Layout` used by `AllocatorBox`. The easiest way to
    /// do this is to convert the raw pointer back into a `AllocatorBox` with the
    /// `AllocatorBox::from_raw` function, allowing the `AllocatorBox` destructor to perform
    /// the cleanup.
    ///
    /// # Examples
    /// Manual cleanup by explicitly running the destructor and deallocating
    /// the memory:
    /// ```
    /// #![feature(allocator_api)]
    /// use ylong_box::AllocatorBox;
    /// use std::alloc::{Allocator, Layout, Global};
    /// use std::ptr::{drop_in_place, NonNull};
    ///
    /// let x = AllocatorBox::new(String::from("Hello"), Global).unwrap();
    /// let p = AllocatorBox::into_raw(x);
    /// unsafe {
    ///     drop_in_place(p);
    ///     Global.deallocate(NonNull::new(p as *mut u8).unwrap(), Layout::new::<String>());
    /// }
    /// ```
    pub fn into_raw_with_allocator(b: Self) -> (*mut T, A) {
        let raw = b.ptr.as_ptr();
        // SAFETY: b is non null and valid
        let alloc = unsafe { read(&b.alloc) };
        AllocatorBox::leak(b);
        (raw, alloc)
    }

    /// Consumes and leaks the `AllocatorBox`, returning a mutable reference,
    /// `&'a mut T`. Note that the type `T` must outlive the chosen lifetime
    /// `'a`. If the type has only static references, or none at all, then this
    /// may be chosen to be `'static`.
    ///
    /// Note: `T` and `Allocator` are both leaked.
    ///
    /// This function is mainly useful for data that lives for the remainder of
    /// the program's life. Dropping the returned reference will cause a memory
    /// leak. If this is not acceptable, the reference should first be wrapped
    /// with the `AllocatorBox::from_raw` function producing a `AllocatorBox`. This `AllocatorBox` can
    /// then be dropped which will properly destroy `T` and release the
    /// allocated memory.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use ylong_box::AllocatorBox;
    /// use std::alloc::Global;
    ///
    /// let x = AllocatorBox::new(41, Global).unwrap();
    /// let static_ref: &'static mut usize = AllocatorBox::leak(x);
    /// *static_ref += 1;
    /// assert_eq!(*static_ref, 42);
    /// ```
    pub fn leak<'a>(b: Self) -> &'a mut T
    where
        A: 'a,
    {
        // SAFETY: b is non null and valid
        unsafe { &mut *mem::ManuallyDrop::new(b).ptr.as_ptr() }
    }

    /// Consumes and leaks the `AllocatorBox`, returning two mutable references,
    /// `&'a mut T` and `&'a mut Allocator`. Note that the type `T` must outlive the chosen lifetime
    /// `'a`. If the type has only static references, or none at all, then this
    /// may be chosen to be `'static`.
    ///
    /// Note: `T` and `Allocator` are both leaked.
    ///
    /// This function is mainly useful for data that lives for the remainder of
    /// the program's life. Dropping the returned reference will cause a memory
    /// leak. If this is not acceptable, the reference should first be wrapped
    /// with the `AllocatorBox::from_raw` function producing a `AllocatorBox`. This `AllocatorBox` can
    /// then be dropped which will properly destroy `T` and release the
    /// allocated memory.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use ylong_box::AllocatorBox;
    /// use std::alloc::Global;
    ///
    /// let x = AllocatorBox::new(41, Global).unwrap();
    /// let (static_ref, _alloc): (&'static mut usize, &'static mut Global) = AllocatorBox::leak_with_allocator(x);
    /// *static_ref += 1;
    /// assert_eq!(*static_ref, 42);
    /// ```
    pub fn leak_with_allocator<'a>(b: Self) -> (&'a mut T, &'a mut A)
    where
        A: 'a,
    {
        // SAFETY: b is non null and valid
        unsafe {
            let mut leak = mem::ManuallyDrop::new(b);
            let t = &mut *leak.ptr.as_ptr();
            let alloc = &mut *(&mut leak.alloc as *mut A);
            (t, alloc)
        }
    }

    /// Returns a reference to the underlying allocator.
    pub fn allocator(b: &Self) -> &A {
        &b.alloc
    }

    fn free_mem(&self) {
        // SAFETY: mem is non null and valid
        unsafe {
            let layout = Layout::for_value_raw(self.ptr.as_ptr());
            if layout.size() != 0 {
                self.alloc.deallocate(From::from(self.ptr.cast()), layout);
            }
        }
    }
}

// SAFETY: Because as for one instance, when it is dropped, its inner pointer might not be dropped yet, but
// it is safe because other strong references are sharing this pointer. As for *this* instance,
// mark the trait as #[may_dangle] to clarify to the compiler that we know what we're doing, and
// it's safe for lifetime and memory management.
unsafe impl<#[may_dangle] T: ?Sized, A: Allocator> Drop for AllocatorBox<T, A> {
    #[inline]
    fn drop(&mut self) {
        let ptr = self.ptr;
        // SAFETY: ptr is non null and valid. Drop for referece does nothing
        unsafe {
            drop_in_place(ptr.as_ptr());
        }

        self.free_mem()
    }
}

impl<T: ?Sized, A: Allocator> AsRef<T> for AllocatorBox<T, A> {
    fn as_ref(&self) -> &T {
        // SAFETY: ptr is non null and valid.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized, A: Allocator> AsMut<T> for AllocatorBox<T, A> {
    fn as_mut(&mut self) -> &mut T {
        // SAFETY: ptr is non null and valid.
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized, A: Allocator> Deref for AllocatorBox<T, A> {
    type Target = T;

    fn deref(&self) -> &T {
        // SAFETY: ptr is non null and valid.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized, A: Allocator> DerefMut for AllocatorBox<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: ptr is non null and valid.
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: SafeClone, A: Allocator + SafeClone> SafeClone for AllocatorBox<T, A> {
    /// Safe clones self, and returns Err if failing in allocation or other operations.
    ///
    /// Note that both contents and allocator of AllocatorBox will be cloned.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_box::{AllocatorBox, SafeClone};
    ///
    /// let mut b = AllocatorBox::new(10, Global).unwrap();
    ///
    /// let clone = b.safe_clone().unwrap();
    /// assert_eq!(clone.as_ref(), b.as_ref());
    /// ```
    fn safe_clone(&self) -> Result<Self, ContainerError> {
        let alloc = self.alloc.safe_clone()?;
        if T::IS_ZST {
            return Ok(Self {
                ptr: Unique::dangling(),
                alloc,
            });
        }
        let layout = Layout::new::<T>();
        let mem = match alloc.allocate(layout) {
            Ok(ptr) => ptr,
            Err(_) => return Err(ContainerError::AllocFailure(layout)),
        };

        let x = (**self).safe_clone()?;
        // SAFETY: mem is non null and valid.
        unsafe {
            write(mem.cast::<T>().as_ptr(), x);
        }

        Ok(Self {
            // SAFETY: mem is allocated above so it's safe.
            ptr: unsafe { Unique::new_unchecked(mem.cast::<T>().as_ptr()) },
            alloc,
        })
    }

    /// Safe clones self from an other instance, and returns Err if failing in allocation or other
    /// operations.
    ///
    /// This method will not clone allocator or reallocate memory. It only clone contents of AllocatorBox.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::Global;
    /// use ylong_box::{AllocatorBox, SafeClone};
    ///
    /// let b1 = AllocatorBox::new(10, Global).unwrap();
    /// let mut b2 = AllocatorBox::new(20, Global).unwrap();
    ///
    /// b2.safe_clone_from(&b1);
    /// assert_eq!(b1.as_ref(), b2.as_ref());
    /// ```
    fn safe_clone_from(&mut self, source: &Self) -> Result<(), ContainerError> {
        (**self).safe_clone_from(&(**source))
    }
}

/// Now unsized type AllocatorBox only can be created by `from_raw` method.
impl<T: SafeClone, A: Allocator + SafeClone> SafeClone for AllocatorBox<[T], A> {
    /// Safe clones self, and returns Err if failing in allocation or other operations.
    ///
    /// This method is for unsized array AllocatorBox type.
    /// Note that both contents and allocator of AllocatorBox will be cloned.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::{Global, Allocator, Layout};
    /// use ylong_box::{AllocatorBox, SafeClone};
    ///
    /// let layout = Layout::new::<[i32;16]>();
    /// let ptr = Global.allocate(layout).unwrap().cast::<[i32;16]>().as_ptr();
    ///
    /// let mut b1: AllocatorBox<[i32], _> = unsafe {
    ///     AllocatorBox::from_raw(ptr, Global)
    /// };
    /// b1[0] = 1;
    /// let b2 = b1.safe_clone().unwrap();
    /// assert_eq!(b1.as_ref(), b2.as_ref());
    /// ```
    fn safe_clone(&self) -> Result<Self, ContainerError> {
        let alloc = self.alloc.safe_clone()?;
        let len = self.len();
        if len == 0 || T::IS_ZST {
            return Ok(Self {
                // SAFETY: ZST do not actually read ptr.
                ptr: unsafe {
                    Unique::new_unchecked(slice_from_raw_parts_mut(
                        NonNull::<T>::dangling().as_ptr(),
                        len,
                    ))
                },
                alloc,
            });
        }

        let layout = match Layout::array::<T>(len) {
            Ok(layout) => layout,
            Err(_) => return Err(ContainerError::LayoutError),
        };

        let ptr = match alloc.allocate(layout) {
            Ok(ptr) => ptr,
            Err(_) => return Err(ContainerError::AllocFailure(layout)),
        };
        let mut raw_ptr = ptr.cast::<T>().as_ptr();
        let head_ptr = raw_ptr;

        // SAFETY: mem is non null and valid.
        unsafe {
            for v in self.deref() {
                raw_ptr.write(v.safe_clone()?);
                raw_ptr = raw_ptr.add(1);
            }
        }

        Ok(Self {
            // SAFETY: mem is allocated above so it's safe.
            ptr: unsafe { Unique::new_unchecked(slice_from_raw_parts_mut(head_ptr, len)) },
            alloc,
        })
    }

    /// Safe clones self from an other instance, and returns Err if failing in allocation or other
    /// operations.
    ///
    /// This method is for unsized array AllocatorBox type.
    ///
    /// Note: If the length of destination is the same as source, this method will not clone allocator or reallocate memory.
    /// It only clone contents of AllocatorBox. Otherwise, this method will both clone contents and allocator of AllocatorBox.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::{Global, Allocator, Layout};
    /// use ylong_box::{AllocatorBox, SafeClone};
    ///
    /// let layout = Layout::new::<[i32;16]>();
    /// let ptr1 = Global.allocate(layout).unwrap().cast::<[i32;16]>().as_ptr();
    /// let ptr2 = Global.allocate(layout).unwrap().cast::<[i32;16]>().as_ptr();
    ///
    /// let mut b1: AllocatorBox<[i32], _> = unsafe { AllocatorBox::from_raw(ptr1, Global) };
    /// let mut b2: AllocatorBox<[i32], _> = unsafe { AllocatorBox::from_raw(ptr2, Global) };
    /// b1[0] = 1;
    /// b2.safe_clone_from(&b1);
    /// assert_eq!(b1.as_ref(), b2.as_ref());
    /// ```
    fn safe_clone_from(&mut self, source: &Self) -> Result<(), ContainerError> {
        if self.len() == source.len() {
            let len = self.len();
            for i in 0..len {
                self[i].safe_clone_from(&source[i])?;
            }
        } else {
            *self = source.safe_clone()?;
        }
        Ok(())
    }
}

/// Now unsized type AllocatorBox only can be created by `from_raw` method.
impl<A: Allocator + SafeClone> SafeClone for AllocatorBox<str, A> {
    /// Safe clones self, and returns Err if failing in allocation or other operations.
    ///
    /// This method is for unsized str AllocatorBox type.
    /// Note that both contents and allocator of AllocatorBox will be cloned.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::{Global, Allocator, Layout};
    /// use ylong_box::{AllocatorBox, SafeClone};
    ///
    /// let slice = "hello".as_bytes();
    /// let layout = Layout::array::<u8>(slice.len()).unwrap();
    ///
    /// let mem = Global.allocate(layout).unwrap();
    /// let ptr = mem.cast::<u8>().as_ptr();
    /// let b1 = unsafe {
    ///     ptr.copy_from(slice.as_ptr(), slice.len());
    ///     AllocatorBox::from_raw(mem.as_ptr() as *mut str, Global)
    /// };
    /// let b2 = b1.safe_clone().unwrap();
    /// assert_eq!(b1.as_ref(), b2.as_ref());
    /// ```
    fn safe_clone(&self) -> Result<Self, ContainerError> {
        let alloc = self.alloc.safe_clone()?;
        let slice = self.as_bytes();
        let len = slice.len();
        if len == 0 {
            return Ok(Self {
                // SAFETY: ZST do not actually read ptr.
                ptr: unsafe {
                    Unique::new_unchecked(slice_from_raw_parts_mut(
                        NonNull::<u8>::dangling().as_ptr(),
                        len,
                    ) as *mut str)
                },
                alloc,
            });
        }

        let layout = match Layout::array::<u8>(len) {
            Ok(layout) => layout,
            Err(_) => return Err(ContainerError::LayoutError),
        };

        let ptr = match alloc.allocate(layout) {
            Ok(ptr) => ptr,
            Err(_) => return Err(ContainerError::AllocFailure(layout)),
        };
        let raw_ptr = ptr.cast::<u8>().as_ptr();

        // SAFETY: mem is non null and valid.
        unsafe {
            raw_ptr.copy_from(slice.as_ptr(), len);
        }

        Ok(Self {
            // SAFETY: mem is allocated above so it's safe.
            ptr: unsafe {
                Unique::new_unchecked(slice_from_raw_parts_mut(raw_ptr, len) as *mut str)
            },
            alloc,
        })
    }

    /// Safe clones self from an other instance, and returns Err if failing in allocation or other
    /// operations.
    ///
    /// This method is for unsized str AllocatorBox type.
    /// Note that both contents and allocator of AllocatorBox will be cloned.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    /// use std::alloc::{Global, Allocator, Layout};
    /// use ylong_box::{AllocatorBox, SafeClone};
    ///
    /// let slice1 = "hello".as_bytes();
    /// let slice2 = "world".as_bytes();
    ///
    /// let mem1 = Global.allocate(Layout::array::<u8>(slice1.len()).unwrap()).unwrap();
    /// let mem2 = Global.allocate(Layout::array::<u8>(slice2.len()).unwrap()).unwrap();
    /// let ptr1 = mem1.cast::<u8>().as_ptr();
    /// let ptr2 = mem2.cast::<u8>().as_ptr();
    /// let b1 = unsafe {
    ///     ptr1.copy_from(slice1.as_ptr(), slice1.len());
    ///     AllocatorBox::from_raw(mem1.as_ptr() as *mut str, Global)
    /// };
    /// let mut b2 = unsafe {
    ///     ptr2.copy_from(slice2.as_ptr(), slice2.len());
    ///     AllocatorBox::from_raw(mem2.as_ptr() as *mut str, Global)
    /// };
    /// b2.safe_clone_from(&b1);
    /// assert_eq!(b1.as_ref(), b2.as_ref());
    /// ```
    fn safe_clone_from(&mut self, source: &Self) -> Result<(), ContainerError> {
        *self = source.safe_clone()?;
        Ok(())
    }
}

impl<T: fmt::Debug + ?Sized, A: Allocator> fmt::Debug for AllocatorBox<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: fmt::Display + ?Sized, A: Allocator> fmt::Display for AllocatorBox<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

#[cfg(test)]
mod tests {
    include!("../tests/ut/ut_allocator_box.rs");
}
