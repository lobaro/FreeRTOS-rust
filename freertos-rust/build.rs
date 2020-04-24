
use std::env;
use std::path::PathBuf;

fn print_env() {
    let env_keys =["TARGET", "OUT_DIR", "HOST"];
    env::vars().for_each(|(key, val)| {
        if key.starts_with("CARGO") {
            println!("cargo:warning={}={}", key, val);
        } else if env_keys.contains(&key.as_str()) {
            println!("cargo:warning={}={}", key, val);
        } else {
            //println!("cargo:warning={}={}", key, val);
        }
    });
}

// See: https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:SHIM={}", PathBuf::from(manifest_dir).join("src/freertos").to_str().unwrap());
}