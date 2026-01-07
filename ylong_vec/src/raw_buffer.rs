use core::alloc::{Allocator, Layout};
use core::mem::SizedTypeProperties;
use core::ptr::{NonNull, Unique};
use std::mem;
use ylong_stdx_common::ContainerError;

// Use for memory allocate
pub(crate) struct AllocatedVecBuffer<T, A: Allocator> {
    pub(crate) ptr: Unique<T>,
    pub(crate) capacity: usize,
    pub(crate) alloc: A,
}

impl<T, A: Allocator> AllocatedVecBuffer<T, A> {
    pub(crate) fn new(allocator: A) -> Self {
        Self {
            ptr: Unique::dangling(),
            capacity: 0,
            alloc: allocator,
        }
    }

    // allocate memory for specified capacity
    pub(crate) fn allocate_in(capacity: usize, alloc: A) -> Result<Self, ContainerError> {
        // do not allocate any memory for ZST or zero capacity.
        if T::IS_ZST || capacity == 0 {
            return Ok(Self::new(alloc));
        }

        // calculate the layout of memory
        let layout = Layout::array::<T>(capacity).map_err(|_| ContainerError::LayoutError)?;
        alloc_check(layout.size())?;

        //allocate memory
        let ptr = match alloc.allocate(layout) {
            Ok(ptr) => ptr,
            Err(_) => return Err(ContainerError::AllocFailure(layout)),
        };

        Ok(Self {
            // SAFETY: ptr is allocated above so it is safe.
            ptr: unsafe { Unique::new_unchecked(ptr.cast::<T>().as_ptr()) },
            capacity,
            alloc,
        })
    }

    // get current memory layout.
    fn current_memory(&self) -> Option<(NonNull<u8>, Layout)> {
        if T::IS_ZST || self.capacity == 0 {
            return None;
        }

        // SAFETY: size and layout have been checked when allocated.
        unsafe {
            let align = mem::align_of::<T>();
            let size = mem::size_of::<T>().unchecked_mul(self.capacity);
            let layout = Layout::from_size_align_unchecked(size, align);
            Some((self.ptr.cast().into(), layout))
        }
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }

    pub(crate) fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    pub(crate) fn allocator(&self) -> &A {
        &self.alloc
    }

    pub(crate) fn capacity(&self) -> usize {
        if T::IS_ZST {
            usize::MAX
        } else {
            self.capacity
        }
    }
}

// SAFETY: Because as for one instance, when it is dropped, its inner pointer might not be dropped yet, but
// it is safe because other strong references are sharing this pointer. As for *this* instance,
// mark the trait as #[may_dangle] to clarify to the compiler that we know what we're doing, and
// it's safe for lifetime and memory management.
unsafe impl<#[may_dangle] T, A: Allocator> Drop for AllocatedVecBuffer<T, A> {
    fn drop(&mut self) {
        if let Some((ptr, layout)) = self.current_memory() {
            // SAFETY: ptr and layout are allocated before so it is safe.
            unsafe { self.alloc.deallocate(ptr, layout) }
        }
    }
}

// On 32-bit and 16-bit we need to add
// an extra check in case running on a platform which use
// all 4GB in user-space, e.g., PAE or x32.
#[inline]
pub(crate) fn alloc_check(alloc_size: usize) -> Result<(), ContainerError> {
    if usize::BITS < 64 && alloc_size > isize::MAX as usize {
        return Err(ContainerError::CapacityOverflow);
    }

    Ok(())
}
