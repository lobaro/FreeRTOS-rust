# FreeRTOS-rust

This project is based on code from [freertos.rs](https://github.com/hashmismatch/freertos.rs) and some additions to
 simplify the usage of [FreeRTOS](https://github.com/FreeRTOS/FreeRTOS-Kernel) in embedded applications written
 in Rust.

In contrast to freertos.rs this crate differs in these points:

- The application `main()` entry point is written in Rust.
- The FreeRTOS scheduler can be started from Rust.
- The FreeRTOS heap `MemMang/heap/heap_x.c`is used as global memory allocator for Rust
- No need for a Clang skeleton project

## How it works

The `freertos-cargo-build` build-dependency compiles the FreeRTOS code from its original "C" sources files into an 
archive to be linked against your Rust app. Internally it uses the [cc crate](https://docs.rs/crate/cc) and some meta 
info provided by your apps `build.rs`:
 
 1. A path to the [FreeRTOS](https://github.com/FreeRTOS/FreeRTOS-Kernel) `Sources`
 1. A path to the app specific `FreeRTOSConfig.h`
 1. A relative path to the `FreeRTOS port` to be used, e.g. for ARM Cortex-M3 cores.
 1. Optional: Additional C code to be compiled
 
 The `freertos_rust` dependency then provides a shim to access all FreeRTOS functionality from your (embedded) 
 Rust app.
 
 ## Usage

1. Checkout FreeRTOS: https://github.com/FreeRTOS/FreeRTOS-Kernel   

1. Add dependencies to your Rust apps `Cargo.toml`

    ```
    [dependencies]
    freertos_rust = "*"
    
    [build-dependencies]
    freertos-cargo-build = "*"
    ```
    
1. Add this snippet to your apps `build.rs`:
    ```
    fn main() {
        let mut b = freertos_cargo_build::Builder::new();
    
        b.freertos("FreeRTOS-Kernel");  // Path to copy of the FreeRTOS kernel
        b.freertos_config("src");       // Location of `FreeRTOSConfig.h` 
        b.freertos_port("GCC/ARM_CM3"); // Port dir relativ to 'FreeRTOS-Kernel/portable' 
        b.heap("heap4.c");              // Set the heap_?.c allocator to use from 
                                        // 'FreeRTOS-Kernel/portable/MemMang'       
   
        // b.get_cc().file("More.c");   // Optional additional C-Code to be compiled
    
        b.compile().unwrap_or_else(|e| { panic!(e.to_string()) });
    }
    ```   

### Used C compiler
`freertos-cargo-build` depends on the [cc crate](https://docs.rs/crate/cc). So the C compiler
used can be set by using the `CC` enviroment variable or otherwise defined by internal 
defaults. For the ARM architecture this is the `arm-none-eabi-gcc` which can be found [here](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads).

## Examples
To get started there are examples in [freertos-rust-examples](freertos-rust-examples) for:

* Cortex M33 (nRF9160)
* Cortex M3 (STM32L151CBU6A)
* Windows
* ...more to follow...

## Project Crates
* To build a project using this create see [freertos-cargo-build](freertos-cargo-build)
* The runtime dependency for you FreeRTOS Rust application will be [freertos-rust](freertos-rust)




