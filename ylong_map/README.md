# ylong_map

## Introduction

This crate provides an enhanced version of hashmap and as a part of ylong stdx project. Currently, it provides the following feature:

- FixedCapMap, with fixed capacity, custimized allocators, and methods returning recoverable error instead of panicking when OOM or some other cases.

This crate is based on Bisheng rustc and YLong Rust Std, and nightly toolchain is necessary.

## Restrictions in usage

### Toolchain reliance
This crate and all other crates in ylong stdx project heavily depend on Bisheng rustc and YingLong Rust Std. Nightly features and rustc are necessary, so users need to make sure that the version of Rust toolchain support these features. Otherwise, please contact maintainers of Bisheng rustc or YingLong std.

For the installation of Bisheng rustc and YingLong Rust Std, please read the `README.md` in ylong_stdx_common crate.

### Panic scenerios

It is still possible that panic occurs in the following cases and other more:
- Panic happens within Allocator's methods

## Compile Build

1. Depend on ylong_map by a relative path

```toml
#[dependencies]
ylong_map = { path = "%PATH_TO_YLONG_MAP%", version = "1.21.0" }
```

## Usage

### `FixedCapMap`

```rust
#![feature(allocator_api)]
 
use std::alloc::Global;
use ylong_map::FixedCapMap;
use ylong_stdx_common::ContainerError;
 
fn main() -> Result<(), ContainerError> {
    let mut map: FixedCapVec::<u8, _> = FixedCapMap::new(3, System::default())?;
    assert_eq!(m.len(), 0);

    m.insert(1, 2)?.is_none();
    assert_eq!(m.len(), 1);
    
    let val = *m.get(&1).unwrap();
    assert_eq!(val, 2);

    assert!(m.remove(&1).is_some());
    
    // Insert until full
    m.insert(2, 3)?.is_none();
    m.insert(3, 4)?.is_none();
    
    assert!(m.insert(4, 5).is_err());
}
```