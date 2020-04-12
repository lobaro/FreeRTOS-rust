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
    rustup toolchain install stable-gnu
    rustup default stable-gnu
    
    // Alternatively use the nightly
    rustup default nightly-gnu
    
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

we need the nightly build for some features like allocator_api:

    rustup default nightly-x86_64-pc-windows-gnu // TODO: Build does not finish with GNU Toolchain
    rustup default nightly-x86_64-pc-windows-msvc
    rustup target add thumbv7m-none-eabi
    
Build the binary:

    cargo build --example stm32-cortex-m3 --target thumbv7m-none-eabi
    
Create hex file to be flashed (also creates the build):

    cargo objcopy --example stm32-cortex-m3 --target thumbv7m-none-eabi -- -O ihex app.hex

### Generate C Bindings

C-Bindings are based on `src/bindings.h` and generated with [bindgen](https://github.com/rust-lang/rust-bindgen)

`bindgen` is executed during build in `build.rs`.




