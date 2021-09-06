use cc::Build;
use std::fmt::Display;
use std::{fmt, env};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// The FREERTOS_SRC env variable must point to the FreeRTOS kernel code.
/// The Kernel can be found at Github: https://github.com/FreeRTOS/FreeRTOS-Kernel
///
/// When not set, you can use the Builder to specify the path
const ENV_KEY_FREERTOS_SRC: &str = "FREERTOS_SRC";

/// The FREERTOS_CONFIG variable must point to the directory
/// where the FreeRTOSConfig.h file is located for the current project.
///
/// When not set, you can use the Builder to specify the path
const ENV_KEY_FREERTOS_CONFIG: &str = "DEP_FREERTOS_CONFIG";

/// FreeRTOS shim.c file to enable usage of FreeRTOS with freertos-rust crate
/// This variable is set by freertos-rust build.rs
const ENV_KEY_FREERTOS_SHIM: &str = "DEP_FREERTOS_SHIM";

#[derive(Clone, Debug)]
pub struct Builder {
    freertos_dir: PathBuf,
    freertos_config_dir: PathBuf,
    freertos_shim: PathBuf,
    freertos_port: Option<String>,
    // name of the heap_?.c file
    heap_c: String,
    cc: Build,
}

pub struct Error {
    /// More explanation of error that occurred.
    message: String,
}

impl Error {
    fn new(message: &str) -> Error {
        Error {
            message: message.to_owned(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}


impl Builder {
    /// Construct a new instance of a blank set of configuration.
    ///
    /// This builder is finished with the [`compile`] function.
    ///
    /// [`compile`]: struct.Build.html#method.compile
    pub fn new() -> Builder {
        let freertos_path = env::var(ENV_KEY_FREERTOS_SRC).unwrap_or_default();
        let freertos_config_path = env::var(ENV_KEY_FREERTOS_CONFIG).unwrap_or_default();
        let freertos_shim = env::var(ENV_KEY_FREERTOS_SHIM).unwrap_or_default();

        let b = Builder {
            freertos_dir: PathBuf::from(freertos_path),
            freertos_config_dir: PathBuf::from(freertos_config_path),
            freertos_shim: PathBuf::from(freertos_shim),
            freertos_port: None,
            cc: cc::Build::new(),
            heap_c: String::from("heap_4.c"),
        };
        return b;
    }


    /// Set the path to freeRTOS source
    /// Default is loaded from ENV variable "FREERTOS_SRC"
    pub fn freertos<P: AsRef<Path>>(&mut self, path: P) {
        self.freertos_dir = path.as_ref().to_path_buf();
    }
    /// Set the path to freeRTOSConfig.h
    /// Default is loaded from ENV variable, see: ENV_KEY_FREERTOS_CONFIG
    pub fn freertos_config<P: AsRef<Path>>(&mut self, path: P) {
        self.freertos_config_dir = path.as_ref().to_path_buf();
    }

    /// Set the path to shim.c (required for freertos-rust)
    /// Default is loaded from ENV variable, see: ENV_KEY_FREERTOS_SHIM
    pub fn freertos_shim<P: AsRef<Path>>(&mut self, path: P) {
        self.freertos_shim = path.as_ref().to_path_buf();
    }

    /// Returns a list of all files in the shim folder
    fn freertos_shim_files(&self) -> Vec<String> {
        let files: Vec<_> = WalkDir::new(self.freertos_shim.as_path())
            .follow_links(false)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|entry| {
                let f_name = entry.path().to_str().unwrap();

                if f_name.ends_with(".c") {
                    return Some(String::from(entry.path().to_str().unwrap()));
                }
                return None;
            }).collect();
        files
    }

    /// Returns a list of all FreeRTOS source files
    fn freertos_files(&self) -> Vec<String> {
        let files: Vec<_> = WalkDir::new(self.freertos_dir.as_path())
            .follow_links(false)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|entry| {
                let f_name = entry.path().to_str().unwrap();

                if f_name.ends_with(".c") {
                    return Some(String::from(entry.path().to_str().unwrap()));
                }
                return None;
            }).collect();
        files
    }
    fn freertos_port_files(&self) -> Vec<String> {
        let files: Vec<_> = WalkDir::new(self.get_freertos_port_dir())
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|entry| {
                let f_name = entry.path().to_str().unwrap();

                if f_name.ends_with(".c") {
                    return Some(String::from(entry.path().to_str().unwrap()));
                }
                return None;
            }).collect();
        files
    }

    /// Set the heap_?.c file to use from the "/portable/MemMang/" folder.
    /// heap_1.c ... heap_5.c (Default: heap_4.c)
    /// see also: https://www.freertos.org/a00111.html
    pub fn heap(&mut self, file_name: String) {
        self.heap_c = file_name;
    }

    /// Access to the underlining cc::Build instance to further customize the build.
    pub fn get_cc(&mut self) -> &mut Build {
        &mut self.cc
    }

    fn freertos_include_dir(&self) -> PathBuf {
        self.freertos_dir.join("include")
    }

    /// set the freertos port dir relativ to the FreeRTOS/Source/portable directory
    /// e.g. "GCC/ARM_CM33_NTZ/non_secure"
    ///
    /// If not set it will be detected based on the current build target (not many targets supported yet).
    pub fn freertos_port(&mut self, port_dir: String) {
        self.freertos_port = Some(port_dir);
    }

    fn get_freertos_port_dir(&self) -> PathBuf {
        let base = self.freertos_dir.join("portable");
        if self.freertos_port.is_some() {
            return base.join(self.freertos_port.as_ref().unwrap());
        }

        let target = env::var("TARGET").unwrap_or_default();
        let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default(); // msvc, gnu, ...
        //let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default(); // unix, windows
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default(); // x86_64
        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default(); // none, windows, linux, macos
        let port = match (target.as_str(), target_arch.as_str(), target_os.as_str(), target_env.as_str()) {
            (_, "x86_64", "windows", _) => "MSVC-MingW",
            (_, "x86_64", "linux", "gnu") => "GCC/Linux",
            ("thumbv7m-none-eabi", _, _, _) => "GCC/ARM_CM3",
            ("thumbv7em-none-eabihf", _, _, _) => "GCC/ARM_CM4F",
            // TODO We should support feature "trustzone"
            ("thumbv8m.main-none-eabi", _, _, _) => "GCC/ARM_CM33_NTZ/non_secure",
            ("thumbv8m.main-none-eabihf", _, _, _) => "GCC/ARM_CM33_NTZ/non_secure",
            _ => {
                panic!("Unknown target: '{}', from TARGET environment variable.", target);
            }
        };
        return base.join(port);
    }

    fn heap_c_file(&self) -> PathBuf {
        self.freertos_dir.join("portable/MemMang").join(self.heap_c.as_str())
    }
    fn shim_c_file(&self) -> PathBuf {
        self.freertos_shim.join("shim.c")
    }

    /// Check that all required files and paths exist
    fn verify_paths(&self) -> Result<(), Error> {
        if !self.freertos_dir.clone().exists() {
            return Err(Error::new(&format!("Directory freertos_dir does not exist: {}", self.freertos_dir.to_str().unwrap())));
        }
        let port_dir = self.get_freertos_port_dir();
        if !port_dir.clone().exists() {
            return Err(Error::new(&format!("Directory freertos_port_dir does not exist: {}", port_dir.to_str().unwrap())));
        }

        let include_dir = self.freertos_include_dir();
        if !include_dir.clone().exists() {
            return Err(Error::new(&format!("Directory freertos_include_dir does not exist: {}", include_dir.to_str().unwrap())));
        }

        // The heap implementation
        let heap_c = self.heap_c_file();
        if !heap_c.clone().exists() || !heap_c.clone().is_file() {
            return Err(Error::new(&format!("File heap_?.c does not exist: {}", heap_c.to_str().unwrap())));
        }

        // Allows to find the FreeRTOSConfig.h
        if !self.freertos_config_dir.clone().exists() {
            return Err(Error::new(&format!("Directory freertos_config_dir does not exist: {}", self.freertos_config_dir.to_str().unwrap())));
        }

        // Add the freertos shim.c to support freertos-rust
        let shim_c = self.shim_c_file();
        if !shim_c.clone().exists() || !shim_c.clone().is_file() {
            return Err(Error::new(&format!("File freertos_shim '{}' does not exist, missing freertos-rust dependency?", shim_c.clone().to_str().unwrap())));
        }

        Ok(())
    }

    pub fn compile(&self) -> Result<(), Error> {
        let mut b = self.cc.clone();

        let path_error = self.verify_paths();
        if path_error.is_err() {
            return path_error;
        }

        // FreeRTOS header files
        b.include(self.freertos_include_dir());
        // FreeRTOS port header files (e.g. portmacro.h)
        b.include(self.get_freertos_port_dir());
        b.include(self.freertos_config_dir.clone());
        b.file(self.heap_c_file());
        self.freertos_files().iter().for_each(|f| {
            b.file(f);
        });
        self.freertos_port_files().iter().for_each(|f| {
            b.file(f);
        });
        self.freertos_shim_files().iter().for_each(|f| {
            b.file(f);
        });

        let res = b.try_compile("freertos");
        if res.is_err() {
            return Err(Error::new(&format!("{}", res.unwrap_err())));
        }

        Ok(())
    }
}

#[test]
fn test_paths() {
    env::set_var("FREERTOS_SRC", "some/path");
    env::set_var("TARGET", "thumbv8m.main-none-eabihf");
    let mut b = Builder::new();
    assert_eq!(b.freertos_dir.to_str().unwrap(), "some/path");
}
/*
#[test]
fn test_compile() {
    env::set_var("FREERTOS_SRC", "C:\\dev\\projects\\FreeRTOS\\FreeRTOS\\Source");
    env::set_var("TARGET", "thumbv8m.main-none-eabihf");
    env::set_var("OUT_DIR", "out");
    env::set_var("OPT_LEVEL", "0");
    env::set_var("HOST", "x86_64-pc-windows-gnu");
    let mut b = Builder::new();

    let res = b.compile();

    if res.is_err() {
        panic!(res.err().unwrap().message)
    }
    assert!(res.is_ok())
}*/