use std::env;

fn main() {
    print_env();

    let mut b = freertos_cargo_build::Builder::new();
    b.freertos("FreeRTOS/Source");

    // Windows example specific stuff.
    // TODO: We need a way to do this based on the target
    b.freertos_config("examples/win");
    b.get_cc().file("examples/win/hooks.c");
    b.get_cc().file("examples/win/Run-time-stats-utils.c");


    println!("cargo:rustc-link-lib=static=winmm");
    b.compile().unwrap_or_else(|e| {panic!(e.to_string())});
}

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