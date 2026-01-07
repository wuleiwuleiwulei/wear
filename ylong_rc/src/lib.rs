//! This crate provides a Rc, RcWithAlloc, that functions mainly the same with Rc in the
//! Rust standard library, but differs in the following aspects:
//!
//! 1. Users need to clarify allocators for RcWithAlloc. This design helps users manage
//! the memory usage in embedded systems.
//!
//! 2. In OOM and other cases, RcWithAlloc's methods won't panic but return Result. In some embedded
//! systems, recoverable errors, rather than breaking the whole process, are better for the users to
//! handle with errors.
//!
//! # Attention
//! Because methods require specification of allocators, feature `allocator_api` and nightly
//! version of rustc is needed.

// Feature for allowance of allocator traits and customized allocator
#![feature(allocator_api)]
// Generate layout by ptr
#![feature(layout_for_ptr)]
// May dangle
#![feature(dropck_eyepatch)]
// Enable to use default fn implementation for traits
#![feature(specialization)]
// Enable to build invalid pointer
#![feature(strict_provenance)]
// Mask out warning on feature specialization
#![allow(incomplete_features)]

// To use alloc, we must declare it explicitly
extern crate alloc;

mod rc;
pub use rc::*;
mod weak;
pub use weak::*;
