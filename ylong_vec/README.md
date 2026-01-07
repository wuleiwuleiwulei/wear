# ylong_vec

## Introduction

This crate provides `FixedCapVec` and `FixedCapVecDeque`, a extend of `Vec` and `VecDeque`. It constructs with fixed capacity and customized Allocator.

Otherwise, `push` returned `Result` rather than `()`, so that user can deal with OOM Error. The same applies for other
methods, such as `new` and `remove`, and so on.

This crate is based on Bisheng rustc and YLong Rust Std, and nightly toolchain is necessary.

## Restrictions in usage

### Toolchain reliance
This crate and all other crates in ylong stdx project heavily depend on Bisheng rustc and YingLong Rust Std. Nightly features and rustc are necessary, so users need to make sure that the version of Rust toolchain support these features. Otherwise, please contact maintainers of Bisheng rustc or YingLong std.

For the installation of Bisheng rustc and YingLong Rust Std, please read the `README.md` in ylong_stdx_common crate.

### Panic scenerios

It is still possible that panic occurs in the following cases and other more:
- Panic happens within Allocator's methods
- Panic happens if index out of bound.

## Compile Build

1. Introduce ylong_vec in Cargo.toml with relative path.

```toml
#[dependencies]
ylong_vec = { path = "%PATH_TO_YLONG_VEC%", version = "1.21.0" }
```

## Usage

### `FixedCapVec`

```rust
#![feature(allocator_api)]

use std::alloc::Global;
use ylong_vec::FixedCapVec;

fn main() {
    let mut vec: FixedCapVec::<u8, _> = FixedCapVec::<u8, Global>::new(4, Global).unwrap();

    assert!(vec.push(1).is_ok());
    assert!(vec.push(2).is_ok());

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    vec[0] = 7;
    assert_eq!(vec[0], 7);

    assert!(vec.extend_from_slice(&[1, 2, 3]).is_ok());

    for x in &vec {
        println!("{x}");
    }
    assert_eq!(vec.as_ref(), [7, 1, 2, 3]);
}
```

### `FixedCapVecDeque`

```rust
#![feature(allocator_api)]

use std::alloc::Global;
use ylong_vec::FixedCapVecDeque;

fn main() {
    let mut deque : FixedCapVecDeque::<u8, _> = FixedCapVecDeque::<u8, Global>::new(4, Global).unwrap();

    assert!(deque.push_front(1).is_ok());
    assert!(deque.push_front(2).is_ok());

    assert_eq!(deque.len(), 2);
    assert_eq!(deque.front(), Some(&2));

    assert_eq!(deque.pop_front(), Some(2));
    assert_eq!(deque.len(), 1);

    assert!(deque.contains(&1));

    for x in &deque {
        println!("{x}");
    }
}
```