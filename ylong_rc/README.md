# ylong_rc

## Introduction

This crate provides an enhanced version of Rc and as a part of ylong stdx project. Currently, it provides the following feature:

- RcWithAlloc and WeakWithAlloc, with custimized allocators, and methods returning recoverable error instead of panicking when OOM or some other cases.

This crate is based on Bisheng rustc and YLong Rust Std, and nightly toolchain is necessary.

## Restrictions in usage

### Toolchain reliance
This crate and all other crates in ylong stdx project heavily depend on Bisheng rustc and YingLong Rust Std. Nightly features and rustc are necessary, so users need to make sure that the version of Rust toolchain support these features. Otherwise, please contact maintainers of Bisheng rustc or YingLong std.

For the installation of Bisheng rustc and YingLong Rust Std, please read the `README.md` in ylong_stdx_common crate.

### Panic scenerios

It is still possible that panic occurs in the following cases and other more:
- Panic happens within Allocator's methods

## Compile Build

1. Depend on ylong_rc by a relative path

```toml
#[dependencies]
ylong_rc = { path = "%PATH_TO_YLONG_RC%", version = "1.21.0" }
```

## Usage

### `RcWithAlloc`

```rust
#![feature(allocator_api)]

use std::alloc::System;
use ylong_rc::{RcWithALloc};
use ylong_stdx_common::{ContainerError, SafeClone};

fn main() -> Result<(), ContainerError> {
    let mut rc = RcWithAlloc::new(3, System::default())?;
    assert_eq!(*rc, 3);
    assert_eq!(RcWithAlloc::strong_count(&rc), 1);
    assert_eq!(RcWithAlloc::weak_count(&rc), 0);
    
    // Update the inner value
    // It is unlikely because here we only have one strong ref. The error should be mapped depending on the 
    // scenario.
    *RcWithAlloc::get_mut(&mut rc).ok_or(ContainerError::Unlikely)? = 4;
    assert_eq!(*rc, 4);

    // Clone it to have another strong ref
    let rc2 = rc.safe_clone()?;
    assert_eq!(*rc2, 4);
    assert_eq!(RcWithAlloc::strong_count(&rc2), 2);
    assert_eq!(RcWithAlloc::weak_count(&rc2), 0);
    
    // If wanting a weak ref
    let weak = RcWithAlloc::downgrade(&rc)?;
    assert_eq!(RcWithAlloc::strong_count(&rc2), 2);
    assert_eq!(RcWithAlloc::weak_count(&rc2), 1);
    
    // Upgrade the weak
    // It is unlikely because here the strong ref lives. The error should be mapped depending on the scenario.
    let rc3 = weak.upgrade()?..ok_or(ContainerError::Unlikely)?;
    assert_eq!(RcWithAlloc::strong_count(&rc), 3);
    assert_eq!(RcWithAlloc::weak_count(&rc), 1);
    
    // Unwrap and get inner value
    drop(rc2);
    drop(rc3);
    drop(weak);
    // Unwrapping will fail and return None if there're other refs. It is unlikely because here we drop all the refs 
    // before. The error should be mapped depending on the scenario.
    let value = RcWithAlloc::into_inner(rc)..ok_or(ContainerError::Unlikely)?;
    assert_eq!(value, 4);
}
```