# FreeRTOS-rust
Rust crate for FreeRTOS

The crate is based on code from [freertos.rs](https://github.com/hashmismatch/freertos.rs).

It provides helper for you `build.rs` to make it easy to compile a embedded Rust application based on FreeRTOS. 
In contrast to freertos.rs the `main()` is written in Rust.

To build a project using this create see [freertos-cargo-build](freertos-cargo-build/README.md)

The runtime dependency for you FreeRTOS Rust application will be [freertos-rust](freertos-rust/README.md)

To get started there are examples in [freertos-rust-examples](freertos-rust-examples/README.md)

## Checkout

Including submodule:

     git clone https://github.com/lobaro/FreeRTOS-rust.git --recurse-submodules

### Update Submodules

After cloning without `--recurse-submodules` the submodules must be loaded

    git submodule update --init --recursive

## Requirements

* [LLVM](https://releases.llvm.org/download.html) to build C Code
* or [LLVM Windows Snapshot](http://llvm.org/builds/)

**Issues:**

* llvm-config is missing on windows


### Generate C Bindings

C-Bindings are based on `src/bindings.h` and generated with [bindgen](https://github.com/rust-lang/rust-bindgen)

`bindgen` is executed during build in `build.rs`.




