# FreeRTOS-rust
Rust crate for FreeRTOS

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

## Setup

For debugging the GNU Toolchain is required:

    rustup default stable
    rustup toolchain install stable-x86_64-pc-windows-gnu
    rustup default stable-x86_64-pc-windows-gnu
    
MSVC Toolchain is not working for debugging:

    rustup toolchain install stable-msvc
    rustup default stable-msvc
    
Cortex-M3 (ARMv7-M architecture):

    rustup target add thumbv7m-none-eabi
    
    
Add some tooling

    cargo install cargo-binutils
    rustup component add llvm-tools-preview

### Build

    cargo build
    
To see all errors use:

    cargo build -vv

### Run Windows Demo

    cargo run --example win --target x86_64-pc-windows-gnu
    
### Run STM32 Coretex M3 Demo

    rustup target add thumbv7m-none-eabi
    
we need the nightly build for some features like allocator_api:

    rustup default nightly
    
Create hex file

    cargo objcopy --example stm32-cortex-m3 --target thumbv7m-none-eabi -- -O ihex app.hex

### Generate C Bindings

C-Bindings are based on `src/bindings.h` and generated with [bindgen](https://github.com/rust-lang/rust-bindgen)

`bindgen` is executed during build in `build.rs`.




