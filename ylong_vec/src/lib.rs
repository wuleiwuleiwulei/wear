//! This crate provides a vector named FixedCapVec, and a queue named FixedCapVecDeque, that functions mainly the same with Vector and VecDeque in the
//! Rust standard library, but differs in the following aspects:
//!
//! 1. In OOM and other cases, FixedCapVec and FixedCapVecDeque's methods won't panic but return Result. In some embedded
//! systems, recoverable errors, rather than breaking the whole process, are better for the users to
//! handle with errors.
//!
//! 2. Users need to clarify allocators and capacity when new a FixedCapVec and FixedCapVecDeque instance, and
//! the length won't grow when users insert items into it. This design helps users manage
//! the memory usage in embedded systems and disallows allocations to happen without users'
//! permission.
//!
//! # Attention
//! Because methods require specification of allocators, feature `allocator_api` and nightly
//! version of rustc is needed.

#![feature(allocator_api)] // For `Allcator` trait
#![feature(ptr_internals)] // For use `Unique<T>`
#![feature(sized_type_properties)] // For `IS_ZST` check
#![feature(unchecked_math)] // For `unchecked_mul`
#![feature(strict_provenance)] // For Raw pointer, (*const T).addr()
#![feature(exact_size_is_empty)] // For `ExactSizeIterator` trait
#![feature(ptr_sub_ptr)] // For Raw pointer, (*const T).sub_ptr()
#![feature(pointer_byte_offsets)] // For Raw pointer, (*const T).wrapping_byte_add()
#![feature(slice_pattern)] // For `as_slice()`
#![feature(dropck_eyepatch)] // For `may_dangle`
#![feature(specialization)] // For specialization
#![allow(incomplete_features)] // Mask the unstable alarm of specialization.

pub(crate) mod raw_buffer;
pub mod vec;
pub mod vec_deque;

pub use vec::FixedCapVec;
pub use vec_deque::FixedCapVecDeque;
pub use ylong_stdx_common::*;
