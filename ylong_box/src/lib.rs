//! This crate provides a box, AllocatorBox, that functions mainly the same with Box in the
//! Rust standard library, but differs in the following aspects:
//!
//! 1. In OOM and other cases, AllocatorBox's methods won't panic but return Result. In some embedded
//! systems, recoverable errors, rather than breaking the whole process, are better for the users to
//! handle with errors.
//!
//! 2. Users need to clarify allocators when new a AllocatorBox instance. This design helps users manage
//! the memory usage in embedded systems and disallows allocations to happen without users'
//! permission.
//!
//! # Attention
//! Because methods require specification of allocators, feature `allocator_api` and nightly
//! version of rustc is needed.

#![feature(ptr_internals)] // For use `Unique<T>`
#![feature(allocator_api)] // For `Allcator` trait
#![feature(sized_type_properties)] // For `IS_ZST` check
#![feature(layout_for_ptr)] //For `Layout::for_value_raw`
#![feature(dropck_eyepatch)] // For `may_dangle`
#![feature(specialization)] // For specialization
#![allow(incomplete_features)] // Mask the unstable alarm of specialization.

pub mod allocator_box;

pub use allocator_box::*;
pub use ylong_stdx_common::*;
