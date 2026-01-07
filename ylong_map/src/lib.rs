//! This crate provides a HashMap, FixedCapMap, that functions mainly the same with HashMap in the
//! Rust standard library, but differs in the following aspects:
//!
//! 1. In OOM and other cases, FixedCapMap's methods won't panic but return Result. In some embedded
//! systems, recoverable errors, rather than breaking the whole process, are better for the users to
//! handle with errors.
//!
//! 2. Users need to clarify allocators and capacity when new a FixedCapMap instance, and
//! FixedCapMap's length won't grow when users insert items into it. This design helps users manage
//! the memory usage in embedded systems and disallows allocations to happen without users'
//! permission.
//!
//! # Attention
//! Because methods require specification of allocators, feature `allocator_api` and nightly
//! version of rustc is needed.

// Feature for allowance of allocator traits and customized allocator
#![feature(allocator_api)]
// Feature to reuse RawTable in hashbrown from std
#![feature(rustc_private)]

mod map;
pub use map::*;
mod iter;
pub use iter::*;
