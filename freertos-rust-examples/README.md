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

    cargo build --example win
    
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

### Run Linux Demo


Prepare the build with:

    rustup default x86_64-unknown-linux-gnu
    rustup target add x86_64-unknown-linux-gnu

Run the build

    cargo run --package freertos-rust-examples --example linux --target x86_64-unknown-linux-gnu

### Run STM32 Cortex-M3 Demo

we need the nightly build for some features like allocator_api:

    rustup default nightly-x86_64-pc-windows-gnu // TODO: Build does not finish with GNU Toolchain
    rustup default nightly-x86_64-pc-windows-msvc
    rustup target add thumbv7m-none-eabi
    
Build the binary:

    cargo build --package freertos-rust-examples --example stm32-cortex-m3 --target thumbv7m-none-eabi
    
Create hex file to be flashed:

    cargo objcopy --example stm32-cortex-m3 --target thumbv7m-none-eabi -- -O ihex stm32-cortex-m3.hex

### Run nRF9160 Demo

Setup:

    rustup default nightly-x86_64-pc-windows-msvc
    rustup target add thumbv8m.main-none-eabihf
    
Build:
    
    cargo build --package freertos-rust-examples --example nrf9160 --target thumbv8m.main-none-eabihf

Create hex file to be flashed:

    cargo objcopy --example nrf9160 --target thumbv8m.main-none-eabihf -- -O ihex nrf9160.hex

**CLion Embedded GDB Server Settings** (for debugging):

* Go to: _File | Settings | Build, Execution, Deployment | Custom Build Targets_:
* Click Add
* Name: `nrf9160-example`
* Toolchain: `arm-none-eabi`

Create the Toolchain under: `File | Settings | Build, Execution, Deployment | Toolchains`

* Name: `arm-none-eabi`
* Debugger: `/path/to/arm-none-eabi-gdb.exe`

Build: 

* Name: `build-nrf9160-example`
* Programm: `cargo`
* Arguments: `build --package freertos-rust-examples --example nrf9160 --target thumbv8m.main-none-eabihf`
* Working directory: `$ProjectFileDir$`

Clean: 

* Name: `clean`
* Programm: `cargo`
* Arguments: `clean`
* Working directory: `$ProjectFileDir$`

Setup a Run Configuration: 

* Executable: `target\thumbv8m.main-none-eabihf\debug\examples\nrf9160` (only selectable after first build!)
* Download executable: `Always`
* 'target remote' args: `tcp:localhost:2331`
* GDB Server: `path/to/JLinkGDBServerCL.exe`
* GDB Server args: `-select USB -device nRF9160 -endian little -if SWD -speed 10000 -LocalhostOnly -noir`


## CLion Settings

To get proper auto completion for macros (e.g. in HAL crates for GPIOs) you have to set
*Settings | Languages & Frameworks | Rust | Expand declarative macros* to `Expand with experimental engine`.
