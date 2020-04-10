extern crate bindgen;
extern crate cc;

use std::path::PathBuf;

// See: https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {

    println!("cargo:rerun-if-changed=src/freertos.c");

    // Build C Code
    cc::Build::new()
        .file("c-lib/add.c")
        .compile("libadd.a");



    // Build FreeRTOS for Windows
    let freertos_src_path = PathBuf::from("FreeRTOS/FreeRTOS/Source/");
    let freertos_plus_src_path = PathBuf::from("FreeRTOS/FreeRTOS-Plus/Source/");
    let freertos_demo_path = PathBuf::from("FreeRTOS/FreeRTOS/Demo");

    cc::Build::new()
        // TODO: This is the windows specific part that needs to be env specific
        .include(freertos_src_path.join("include"))
        //.include(freertos_demo_path.join("WIN32-MSVC"))
        .include(freertos_demo_path.join("WIN32-MSVC/Trace_Recorder_Configuration"))
        .include(freertos_plus_src_path.join("FreeRTOS-Plus-Trace/Include"))
        .include(freertos_src_path.join("portable/MSVC-MingW"))
        .file(freertos_demo_path.join("WIN32-MSVC/Run-time-stats-utils.c"))

        // Files related to port
        .include("src/ports/win")
        .file("src/ports/win/hooks.c")

        .file(freertos_plus_src_path.join("FreeRTOS-Plus-Trace/trcKernelPort.c"))
        .file(freertos_plus_src_path.join("FreeRTOS-Plus-Trace/trcSnapshotRecorder.c"))
        .file(freertos_src_path.join("croutine.c"))
        .file(freertos_src_path.join("event_groups.c"))
        .file(freertos_src_path.join("portable/MemMang/heap_5.c"))
        .file(freertos_src_path.join("stream_buffer.c"))
        .file(freertos_src_path.join("timers.c"))
        .file(freertos_src_path.join("list.c"))
        .file(freertos_src_path.join("queue.c"))
        .file(freertos_src_path.join("tasks.c"))
        .file(freertos_src_path.join("portable/MSVC-MingW/port.c"))

        //.file("src/freertos_shim.c")

        .compile("libfreertos.a");


    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/bindings.h");
    println!("cargo:rerun-if-changed=c-lib/add.c");



    //println!("FOO: -I{}", freertos_src_path.join("portable/MSVC-MingW").to_str().unwrap());
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .derive_debug(false)
        .impl_debug(false)
        .layout_tests(false)
        // The input header we would like to generate
        // bindings for.
        .header("src/bindings.h")
        //portmacro.h
        //.clang_arg(format!("-I{}", freertos_src_path.join("portable/MSVC-MingW").to_str().unwrap()))
        .clang_arg(format!("-I{}", freertos_demo_path.join("WIN32-MSVC").to_str().unwrap()))
        .clang_arg(format!("-I{}", freertos_demo_path.join("WIN32-MSVC/Trace_Recorder_Configuration").to_str().unwrap()))
        .clang_arg(format!("-I{}", freertos_plus_src_path.join("FreeRTOS-Plus-Trace/Include").to_str().unwrap()))
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
}