# FreeRTOS Rust Examples


## Setup

We need to use nightly toolchain to support all examples. 
Even if some might run with the stable toolchain as well.

**GNU Toolchain** is required for debugging some examples (e.g. windows):

    rustup default nightly
    rustup toolchain install nightly-gnu
    rustup default nightly-gnu
    
_If you have issues that the build does not ends, try th MSVC Toolchain!_
    
**MSVC Toolchain** is not working for debugging:

    rustup toolchain install stable-msvc
    rustup default nightly-msvc
    
Add you target, e.g. for Cortex-M3 (ARMv7-M architecture):

    rustup target add thumbv7m-none-eabi
    
Install required/useful tooling

    cargo install cargo-binutils
    rustup component add llvm-tools-preview

### Build

    cargo build -example win
    
**Note:** An example for linux is very welcome :)
    
To see all errors use:

    cargo build -vv

### Run Windows Demo

You need to build with nightly GNU to allow debugging. 
The target must be `x86_64-pc-windows-msvc` for the FreeRTOS `MSVC-MingW` port.

Prepare the build with:

    rustup default nightly-x86_64-pc-windows-gnu
    rustup target add x86_64-pc-windows-msvc
    
Run the build

    cargo run --package freertos-rust-examples --example win --target x86_64-pc-windows-msvc
    
### Run STM32 Coretex M3 Demo

we need the nightly build for some features like allocator_api:

    rustup default nightly-x86_64-pc-windows-gnu // TODO: Build does not finish with GNU Toolchain
    rustup default nightly-x86_64-pc-windows-msvc
    rustup target add thumbv7m-none-eabi
    
Build the binary:

    cargo build --package freertos-rust-examples --example stm32-cortex-m3 --target thumbv7m-none-eabi
    
Create hex file to be flashed (also creates the build):

    cargo objcopy --example stm32-cortex-m3 --target thumbv7m-none-eabi -- -O ihex stm32-cortex-m3.hex

## CLion Settings

To get proper auto completion for macros (e.g. in HAL crates for GPIOs) you have to set
*Settings | Languages & Frameworks | Rust | Expand declarative macros* to `Expand with experimental engine`.
