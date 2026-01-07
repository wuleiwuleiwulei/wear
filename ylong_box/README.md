# ylong_box

## Introduction

This crate provides `AllocatorBox`, an extension of `Box`. It constructs with customized Allocator.

Otherwise, `new` returned `Result` rather than `()`, so that user can deal with OOM Error.

This crate is based on Bisheng rustc and YLong Rust Std, and nightly toolchain is necessary.

## Restrictions in usage

### Toolchain reliance
This crate and all other crates in ylong stdx project heavily depend on Bisheng rustc and YingLong Rust Std. Nightly features and rustc are necessary, so users need to make sure that the version of Rust toolchain support these features. Otherwise, please contact maintainers of Bisheng rustc or YingLong std.

For the installation of Bisheng rustc and YingLong Rust Std, please read the `README.md` in ylong_stdx_common crate.

### Panic scenerios

It is still possible that panic occurs in the following cases and other more:
- Panic happens within Allocator's methods
- Panic happens within unsafe method.

## Compile Build

1. Introduce ylong_box in Cargo.toml with relative path.

```toml
#[dependencies]
ylong_box = { path = "%PATH_TO_YLONG_BOX%", version = "1.21.0" }
```

## Usage

### `AllocatorBox`

```rust
#![feature(allocator_api)]

use std::alloc::Global;
use ylong_box::AllocatorBox;

fn main() {
    let five = AllocatorBox::new(5, Global);
}
```

## Differences

Note that there are some different features below between `Box` and `AllocatorBox`.

1. The dereference operator for `Box<T>` produces a place which can be moved from. This means that the `*` operator and the destructor of `Box<T>` are built-in to the language. But not for `AllocatorBox`.
```rust
struct Bar {
    val: i32,
}

fn test() {
    let boxed: Box<_> = Box::new(Bar { val: 123 });
    let val = *boxed;
}
```
This feature is known to Rust compiler. So it doesn't work for `AllocatorBox`.
```rust
struct Bar {
    val: i32,
}

fn test() {
    let boxed: AllocatorBox<_, _> = AllocatorBox::new(Bar { val: 123 }, Global).unwrap();
    let val = *boxed; // Error: move occurs because value has type `Bar`, which does not implement the `Copy` trait
}
```

2. Methods can take `Box<Self>` as a receiver.

3. A trait may be implemented for `Box<T>` in the same crate as `T`, which the orphan rules prevent for other generic types.