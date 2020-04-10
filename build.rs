extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {

    // Build C Code
    cc::Build::new()
        .file("c-lib/add.c")
        .compile("libadd.a");


    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/bindings.h");
    println!("cargo:rerun-if-changed=c-lib/add.c");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/bindings.h")
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