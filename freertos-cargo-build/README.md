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

    // Path to copy of the FreeRTOS kernel "C" code
    b.freertos("FreeRTOS/Source");

    // The `FreeRTOSConfig.h` is usually inside your main crate to match you application and target needs.
    b.freertos_config("src"); 

    // set the freertos port dir relativ to the FreeRTOS/Source/portable directory
    // "GCC/ARM_CM33_NTZ/non_secure"
    // If not set it will be detected based on the current build target (not many targets supported yet)
    b.freertos_port("GCC/ARM_CM33_NTZ/non_secure");

    // Additional "C" code may optionally compiled beside FreeRTOS using:
    // b.get_cc().file("optionalAdditionCode.c");

    // Compiles the FreeRTOS "C" Code
    b.compile().unwrap_or_else(|e| { panic!(e.to_string()) });
}

### Select FreeRTOS port
See: `freertos_cargo_build::Builder::freertos_port(...)`