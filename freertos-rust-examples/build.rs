use std::env;
use std::fs::copy;
use std::path::PathBuf;

fn main() {
    // Allows to show relevant environment variables for debugging purpose
    print_env();

    let target = env::var("TARGET").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut b = freertos_cargo_build::Builder::new();

    b.freertos("FreeRTOS/Source");

    // Windows example specific stuff.
    if target_family == "windows" {
        b.freertos_config("examples/win");
        // TODO: in future all FreeRTOS API should be implemented by the freertos-rust crate
        // until then, we need to compile some C code manually
        b.get_cc().file("examples/win/hooks.c");
        b.get_cc().file("examples/win/Run-time-stats-utils.c");

        if target_env == "msvc" {
            println!("cargo:rustc-link-lib=static=winmm");
        }
    }

    if target == "x86_64-unknown-linux-gnu" {
        b.freertos_config("examples/linux");

        b.get_cc().file("examples/linux/hooks.c");
        // b.get_cc().file("examples/linux/Run-time-stats-utils.c"); // Unimplemented yet..
    }

    if target == "thumbv7m-none-eabi" {
        b.freertos_config("examples/stm32-cortex-m3");
        copy(
            "examples/stm32-cortex-m3/memory.x",
            PathBuf::from(out_dir.as_str()).join("memory.x"),
        ).unwrap();
    }
    if target == "thumbv8m.main-none-eabihf" {
        b.freertos_config("examples/nrf9160");
        copy(
            "examples/nrf9160/memory.x",
            PathBuf::from(out_dir.as_str()).join("memory.x"),
        ).unwrap();
    }

    b.compile().unwrap_or_else(|e| { panic!(e.to_string()) });
}

/// Print relevant environment variables
fn print_env() {
    let env_keys = ["TARGET", "OUT_DIR", "HOST"];
    env::vars().for_each(|(key, val)| {
        if key.starts_with("CARGO") {
            println!("cargo:warning={}={}", key, val);
        } else if env_keys.contains(&key.as_str()) {
            println!("cargo:warning={}={}", key, val);
        } else {
            // println!("cargo:warning={}={}", key, val);
        }
    });
}