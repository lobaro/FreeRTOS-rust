extern crate bindgen;
extern crate cc;

use std::path::PathBuf;

// See: https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    println!("run build.rs");
    //println!("cargo:warning=Test Warning");

    println!("cargo:rerun-if-changed=always");



    // Build C Code
    cc::Build::new()
        .file("c-lib/add.c")
        .compile("libadd.a");

    // Build FreeRTOS for Windows
    let freertos_src_path = PathBuf::from("FreeRTOS/FreeRTOS/Source/");
    let freertos_plus_src_path = PathBuf::from("FreeRTOS/FreeRTOS-Plus/Source/");
    let freertos_demo_path = PathBuf::from("FreeRTOS/FreeRTOS/Demo");

    let demo = "WIN32-MingW";
    //let demo = "WIN32-MSVC";

    let port = "MSVC-MingW";

    // For GNU compilation we need the winmm library
    println!("cargo:rustc-link-lib=static=winmm");
    cc::Build::new()
        //.cpp_link_stdlib("stdc++")
        //.flag("-DprojCOVERAGE_TEST=0")
        .define("projCOVERAGE_TEST", "0")
        .static_flag(true)
        .shared_flag(true)
        // Files related to port
        //.include("src/freertos/ports/win/")
        // TODO: This is the windows specific part that needs to be env specific
        // FreeRTOS.h and modules (Task, Queues, etc.)
        .include(freertos_src_path.join("include"))
        // FreeRTOSConfig.h
        //.include(freertos_demo_path.join(demo))
        .include("src/freertos/ports/win")
        // portmacro.h
        .include(freertos_src_path.join("portable").join(port))
        // Tracing from Demo TODO: Get rid of this
        //.include(freertos_demo_path.join(demo).join("Trace_Recorder_Configuration"))
        //.include(freertos_plus_src_path.join("FreeRTOS-Plus-Trace/Include"))



        // TODO: Make runtime stats not needed
        .file(freertos_demo_path.join(demo).join("Run-time-stats-utils.c"))
        .file("src/freertos/ports/win/hooks.c")
        .file("src/freertos/ports/win/heap.c")
        .file("src/freertos/shim.c") // TODO: make separate lib file for shim?

        // FreeRTOS Plus Trace is needed for windows Demo
        //.file(freertos_plus_src_path.join("FreeRTOS-Plus-Trace/trcKernelPort.c"))
        //.file(freertos_plus_src_path.join("FreeRTOS-Plus-Trace/trcSnapshotRecorder.c"))

        // FreeRTOS
        .file(freertos_src_path.join("croutine.c"))
        .file(freertos_src_path.join("event_groups.c"))
        .file(freertos_src_path.join("portable/MemMang/heap_5.c"))
        .file(freertos_src_path.join("stream_buffer.c"))
        .file(freertos_src_path.join("timers.c"))
        .file(freertos_src_path.join("list.c"))
        .file(freertos_src_path.join("queue.c"))
        .file(freertos_src_path.join("tasks.c"))
        .file(freertos_src_path.join("portable").join(port).join("port.c"))

        .compile("freertos");


    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/bindings.h");
    println!("cargo:rerun-if-changed=c-lib/add.c");

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
        .clang_arg(format!("-I{}", freertos_src_path.join("include").to_str().unwrap()))
        // FreeRTOSConfig.h
        .clang_arg("-Isrc/freertos/ports/win")
        //.clang_arg(format!("-I{}", freertos_demo_path.join(demo).to_str().unwrap()))
        // trcRecorder.h
        .clang_arg(format!("-I{}", freertos_plus_src_path.join("FreeRTOS-Plus-Trace/Include").to_str().unwrap()))
        // trcConfig.h
        .clang_arg(format!("-I{}", freertos_demo_path.join(demo).join("Trace_Recorder_Configuration").to_str().unwrap()))
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