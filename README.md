# FreeRTOS-rust
Rust crate for FreeRTOS

## Checkout

Including submodule:

     git clone https://github.com/lobaro/FreeRTOS-rust.git --recurse-submodules

## Update Submodules

After cloning without `--recurse-submodules` the submodules must be loaded

    git submodule update --init --recursive

## Requirements

* [LLVM](https://releases.llvm.org/download.html) to build C Code
* or [LLVM Windows Snapshot](http://llvm.org/builds/)

**Issues:**

* llvm-config is missing on windows

## Generate C Bindings

C-Bindings are based on `src/bindings.h` and generated with [bindgen](https://github.com/rust-lang/rust-bindgen)




