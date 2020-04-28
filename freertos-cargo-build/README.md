# freertos-cargo-build
Create to help with building FreeRTOS applications with Cargo and Rust

## Usage

Add dependencies to your `Cargo.toml`

```
[dependencies]
freertos_rust = "*"

[build-dependencies]
freertos-cargo-build = "*"
```

To build FreeRTOS you need to specify a path to the FreeRTOS `Source` directory and your `FreeRTOSConfig.h`.
The FreeRTOS Source directory can also be set via the environment variable `FREERTOS_SRC`. It contains files like `taks.h`
and subdirectories `include` and `portable`. The correct port is automatically detected.

The `FreeRTOSConfig.h` is usually inside your main crate to match you application and target needs.

Add this snippet to your apps `build.rs`:
```
use std::env;

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

## Used C compiler
`freertos-cargo-build` depends on the [cc crate](https://docs.rs/crate/cc). So the C compiler
used can be set by using the `CC` enviroment variable or otherwise falling back on internal 
defaults. For the ARM architecture this is the `arm-none-eabi-gcc` which can be found [here](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads).
