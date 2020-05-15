# freertos-cargo-build
Helper crate for building FreeRTOS applications with Cargo and Rust using a `build.rs`.

To build an embedded application with FreeRTOS please refer 
to [freertos-rust home](https://github.com/lobaro/FreeRTOS-rust).


## Usage

The crate is published on [crates.io](https://crates.io/crates/freertos-cargo-build)

    [build-dependencies]
    freertos-cargo-build = "*"
    
Create a `build.rs` file to build FreeRTOS and other C code. See [freertos-rust home](https://github.com/lobaro/FreeRTOS-rust) for an initial example.
