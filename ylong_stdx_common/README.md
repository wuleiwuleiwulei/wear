# ylong_stdx_common

## Introduction

This crate is the base crate for all other crates in ylong stdx project. This project aims to provide enhancement of Rust standard library in embedded systems and other environments.

The enhancement features include:
- Fixed capacity in containers
- Customized allocators for structs
- Avoidance of panic in structs' methods, when panic cases, e.g. OOM, happen

Currently, ylong stdx project provides the following features:
- Hashmap: FixedCapMap
- Vector: FixedCapVec

## Compile Build

1. Depend on ylong_stdx by a relative path

```toml
#[dependencies]
ylong_stdx_common = { path = "%PATH_TO_YLONG_STDX_COMMON%", version = "1.21.0" }
```

## Restrictions in usage

This crate and all other crates in ylong stdx project heavily depend on Bisheng rustc and YingLong Rust Std. Nightly features and rustc are necessary, so users need to make sure that the version of Rust toolchain support these features. Otherwise, please contact maintainers of Bisheng rustc or YingLong std.

### Installation of Rust toolchain

Bisheng rustc and YingLong Rust std are documented in CMC. To install these two components and compose a full copy of Rust toolchain, please following the following steps.

#### 1.Download these two components on the target computer

Download Bisheng rustc and YingLong Rust std of the corresponding version from CMC, and put the tar file on the target computer. Std pack consists of stds of different targets. Please select the std that are needed, and others are safe to be deleted.

This document will skip this part. If users meet any problems in downloading these parts, please contact the corresponding maintainers.

#### 2. Untar each pack and run install bash script in the pack

The install bash script can run in this way:

```shell
bash install.sh --prefix="ROOT_PATH_OF_THE_TOOLCHAIN"
```

In success, the structure of the toolchain directory will be:
```
ROOT
|--- bin
|     |--- **rustc, cargo, rustdoc, cargo-clippy, Binary of other Rust tools**
|--- lib
|     |--- **librustc_driver.so, libstd.so, libtest.so**
|     |--- rustlib
|            |--- x86_64-unknown-linux-gnu
|                   |--- lib
|                         |--- **liballoc.rlib, libcore.rlib, libstd.rlib, Rlib of other Rust std deps**
|            |--- Directories of other target this Rust toolchain version supports, in the same structure of x86_64-unknown-linux-gnu
|            |--- rustc-src, if the source code component included
```

It is recommended to install rustc and std target with the same root path, for the convenience to search for the std used in building source code and running build.rs. If users want to install rustc and std in different root path, they should specify configurations listed in chapter 3.

#### 3. Use the Rust toolchain

Usually, a rust toolchain is composed with rustc and Rust std, though std can be specified by extra configurations.

If there's Rustup on the target system, users can add the customized toolchain in this way:

```shell
rustup toolchain link %NAME_OF_THE_TOOLCHAIN% %PATH_TO_THE_ROOT_PATH_OF_RUSTC%
```

Then use this toolchain with the help of Rustup. For the usage of Rustup, please refer to the community's handbook of Rustup.

If not, feel free to add the path to rustc and cargo to the environment variable `PATH`. In most cases, they should be in the same directory, in the `bin` directory of the root path to the toolchain. 

If everything is installed in the same root path, then we are done here, and we can use the toolchain to build and run Rust projects. If not, config as the following. 

##### 3.1 Specifying the location of std when building source code

Please pass the sysroot flag to cargo when building:

```shell
RUSTFLAGS="--sysroot=%ROOT_PATH_TO_STD_TOOLCHAIN%" cargo build ....
```

##### 3.2 Specifying the location of std when running build.rs

**Not recommended because of additional editions and unstable cargo feature. Please make sure the cargo used supports this feature.**

In the crate that need to run build.rs with std in a different location with rustc, add this line to the top of Cargo.toml:
```toml
cargo-features = ["profile-rustflags"]
```

Add the following configuration in Cargo.toml:
```toml
# If use in dev build
[profile.dev.build-override]
rustflags = ["--sysroot", "%ROOT_PATH_TO_STD_TOOLCHAIN%" ]

# If use in release build
[profile.release.build-override]
rustflags = ["--sysroot", "%ROOT_PATH_TO_STD_TOOLCHAIN%" ]
```

Add this line to cargo config (If globally configured, the path is `~/.cargo/config`):

```toml
[unstable]
profile-rustflags = true
```