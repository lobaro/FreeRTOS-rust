# FreeRTOS-rust

This project is based on code from [freertos.rs](https://github.com/hashmismatch/freertos.rs) and own additions to
 simplify the usage of [FreeRTOS](https://github.com/FreeRTOS/FreeRTOS-Kernel) in embedded applications written
 in Rust.

But in contrast to freertos.rs this crate differs in these points:

- The application `main()` entry point is written in Rust.
- The FreeRTOS scheduler can be started from Rust.
- The FreeRTOS heap `MemMang/heap/heap_x.c`is used as global memory allocator for Rust
- No need for a Clang skeleton project

## Usage

Add dependencies to your apps `Cargo.toml`

```
[dependencies]
freertos_rust = "*"

[build-dependencies]
freertos-cargo-build = "*"
```

To build FreeRTOS you need to specify a path to the [FreeRTOS](https://github.com/FreeRTOS/FreeRTOS-Kernel) `Source` directory and your `FreeRTOSConfig.h`.
The `freertos-cargo-build` build dependency takes care of compiling FreeRTOS using the
[cc crate](https://crates.io/crates/cc). You have to specify location of the [FreeRTOS kernel](https://github.com/FreeRTOS/FreeRTOS-Kernel)
code and your project specific `FreeRTOSConfig.h`. 

Add this snippet to your apps `build.rs`:
```
use std::env;

fn main() {
    let mut b = freertos_cargo_build::Builder::new();

    // Path to copy of the FreeRTOS kernel "C" code
    b.freertos("FreeRTOS/Source");

    // The `FreeRTOSConfig.h` is usually inside your main crate to match you application and target needs.
    b.freertos_config("src"); 

    // Additional "C" code may optionally compiled beside FreeRTOS using:
    // b.get_cc().file("optionalAdditionCode.c");

    // Compiles the FreeRTOS "C" Code
    b.compile().unwrap_or_else(|e| { panic!(e.to_string()) });
}

```
The correct port is automatically detected.

## Structure

* To build a project using this create see [freertos-cargo-build](freertos-cargo-build/README.md)
* The runtime dependency for you FreeRTOS Rust application will be [freertos-rust](freertos-rust/README.md)
* To get started there are examples in [freertos-rust-examples](freertos-rust-examples/README.md)



