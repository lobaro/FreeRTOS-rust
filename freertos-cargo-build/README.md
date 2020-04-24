# freertos-cargo-build
Create to help with building FreeRTOS applications with Cargo and Rust


## Usage

Add dependencies to your `Cargo.toml`

To build FreeRTOS you need to specify a path to the FreeRTOS `Source` directory and your `FreeRTOSConfig.h`.
The FreeRTOS Source directory can also be set via the environment variable `FREERTOS_SRC`. It contains files like `taks.h`
and subdirectories `include` and `portable`. The correct port is automatically detected.

The `FreeRTOSConfig.h` is usually inside your main crate to match you application and target needs.

```
    let mut b = freertos_cargo_build::Builder::new();
    b.freertos("FreeRTOS/Source");
    b.freertos_config("src/freertos");
    b.compile().unwrap_or_else(|e| {panic!(e.to_string())});
```

### Select FreeRTOS port
See: `freertos_cargo_build::Builder::freertos_port(...)`