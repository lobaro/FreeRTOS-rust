extern crate bindgen;
extern crate cc;

use std::env;
use std::path::{PathBuf};

#[allow(unused_variables)]
// See: https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    println!("run build.rs");

    // ENV variables:
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    let target = env::var("TARGET").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap(); // msvc, gnu, ...
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap(); // x86_64
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap(); // none, windows, linux, macos
    println!("cargo:warning=Target is '{}', ARCH = {}, ENV = {}, OS = {}",
             target, target_arch, target_env, target_os);

    println!("cargo:warning=CARGO_MANIFEST_DIR '{}'", env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:warning=OUT_DIR '{}'", env::var("OUT_DIR").unwrap());
    println!("cargo:warning=CARGO_PKG_NAME '{}'", env::var("CARGO_PKG_NAME").unwrap());

    println!("cargo:rerun-if-changed=always");

    build_linker_script("examples/stm32-cortex-m3/layout.ld");

    // Build FreeRTOS for Windows
    let freertos_src_path = PathBuf::from("FreeRTOS/FreeRTOS/Source/");
    let freertos_plus_src_path = PathBuf::from("FreeRTOS/FreeRTOS-Plus/Source/");
    let freertos_demo_path = PathBuf::from("FreeRTOS/FreeRTOS/Demo");


    // TODO: remove? We do not use anything from the demo dir anymore
    let demo = match target.as_str() {
        "x86_64-pc-windows-msvc" => "WIN32-MSVC",
        "x86_64-pc-windows-gnu" => "WIN32-MingW",
        "thumbv7m-none-eabi" => "",
        _ => ""
    };

    // TODO: We have to make this more fine grained as more ports are added
    let port = match target_arch.as_str() {
        "x86_64" => "MSVC-MingW",
        "arm" => "GCC/ARM_CM3",
        _ => {
            println!("cargo:warning=Unknown arch: '{}'", target_arch);
            "MSVC-MingW"
        }
    };

    // For GNU compilation we need the winmm library
    if target_env.as_str() == "gnu" {
        println!("cargo:rustc-link-lib=static=winmm");
    }
    let mut build = &mut cc::Build::new();
    build = build
        .define("projCOVERAGE_TEST", "0")
        //.static_flag(true)
        //.shared_flag(true)
        // Files related to port
        //.include("src/freertos/ports/win/")
        // TODO: This is the windows specific part that needs to be env specific
        // FreeRTOS.h and modules (Task, Queues, etc.)
        .include(freertos_src_path.join("include"))
        // portmacro.h
        .include(freertos_src_path.join("portable").join(port));
    // Tracing from Demo TODO: Get rid of this
    //.include(freertos_demo_path.join(demo).join("Trace_Recorder_Configuration"))
    //.include(freertos_plus_src_path.join("FreeRTOS-Plus-Trace/Include"))

    // TODO: find a better way to find the correct FreeRTOSConfig.h
    // FreeRTOSConfig.h
    if target.as_str() == "thumbv7m-none-eabi" {
        build = build.include("examples/stm32-cortex-m3");
    } else {
        //.include(freertos_demo_path.join(demo))
        build= build.include("src/freertos/ports/win")
    }

    if target_os.as_str() == "windows" {
        build = build.file("src/freertos/ports/win/Run-time-stats-utils.c")
            .file("src/freertos/ports/win/hooks.c")
            .file("src/freertos/ports/win/heap.c")
            .file("src/freertos/shim.c") // TODO: make separate lib file for shim?
    }

    // FreeRTOS Plus Trace is needed for windows Demo
    //.file(freertos_plus_src_path.join("FreeRTOS-Plus-Trace/trcKernelPort.c"))
    //.file(freertos_plus_src_path.join("FreeRTOS-Plus-Trace/trcSnapshotRecorder.c"))

    // FreeRTOS
    build = build.file(freertos_src_path.join("croutine.c"))
        .file(freertos_src_path.join("event_groups.c"))
        .file(freertos_src_path.join("portable/MemMang/heap_5.c"))
        .file(freertos_src_path.join("stream_buffer.c"))
        .file(freertos_src_path.join("timers.c"))
        .file(freertos_src_path.join("list.c"))
        .file(freertos_src_path.join("queue.c"))
        .file(freertos_src_path.join("tasks.c"))
        .file(freertos_src_path.join("portable").join(port).join("port.c"));

    build.compile("freertos");


    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/bindings.h");

    //return; // TODO: Some flags are missing for minGW bindgen to solve: 'x86intrin.h' file not found
    // Does not work:
    //std::env::set_var("RUST_LOG", "debug");
    //println!("cargo:rustc-env=RUST_LOG=debug");
    //println!("FOO: -I{}", freertos_src_path.join("portable/MSVC-MingW").to_str().unwrap());
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .derive_debug(false)
        .impl_debug(false)
        .layout_tests(false)
        .detect_include_paths(false)
        // The input header we would like to generate
        // bindings for.
        .header("src/bindings.h")
        //.trust_clang_mangling(false)

        //.header("src/freertos/freertos_shim.h")
        //portmacro.h
        //.clang_arg(format!("-I{}", freertos_src_path.join("portable/MSVC-MingW").to_str().unwrap()))
        // FreeRTOS.h
        //.clang_arg(format!("-I{}", freertos_src_path.join("include").to_str().unwrap()))
        // FreeRTOSConfig.h
        //.clang_arg("-Isrc/freertos/ports/win")
        //.clang_arg(format!("-I{}", freertos_demo_path.join(demo).to_str().unwrap()))
        // trcRecorder.h
        //.clang_arg(format!("-I{}", freertos_plus_src_path.join("FreeRTOS-Plus-Trace/Include").to_str().unwrap()))
        // trcConfig.h
        //.clang_arg(format!("-I{}", freertos_demo_path.join(demo).join("Trace_Recorder_Configuration").to_str().unwrap()))
        // Make the generated code #![no_std] compatible
        .use_core()
        // Tip from https://rust-embedded.github.io/book/interoperability/c-with-rust.html
        //.ctypes_prefix("cty")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    //let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = PathBuf::from("src/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");


    // TODO: create hex file:
    // Use?: https://github.com/rust-embedded/cargo-binutils
    // cargo objcopy -O ihex
    /*
        add_custom_command(TARGET ${TARGET} POST_BUILD
            COMMAND ${CMAKE_OBJCOPY} -Oihex $<TARGET_FILE:${TARGET}> ${HEX_FILE}
            COMMENT "Building ${HEX_FILE}")

    add_custom_command(TARGET ${TARGET} POST_BUILD
            COMMAND ${CMAKE_OBJCOPY} -Obinary $<TARGET_FILE:${TARGET}> ${BIN_FILE}
            COMMENT "Building ${BIN_FILE}")

    add_custom_command(TARGET ${TARGET} POST_BUILD
            COMMAND arm-none-eabi-size --format=berkeley --totals "$<TARGET_FILE:${TARGET}>"
            COMMENT "Invoking: Cross ARM GNU Print Size")
    */
}

fn build_linker_script(path: &str) {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    std::fs::copy(path, out.join("memory.x"))
        .expect("failed to copy linker script");
    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed={}", path);
}