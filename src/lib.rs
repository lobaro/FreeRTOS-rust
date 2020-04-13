#![feature(allocator_api)]
//! # FreeRTOS for Rust
//!
//! Rust interface for the FreeRTOS embedded operating system. Requires beta Rust.
//! It is assumed that dynamic memory allocation is provided on the target system.
//!
//! This library interfaces with FreeRTOS using a C shim library which provides function
//! wrappers for FreeRTOS macros. The compiled Rust application should be linked to the
//! base C/C++ firmware binary. Check the subdirectory ``shim``. Copy the source file to
//! your firmware's sources directory and modify it to include the appropriate headers for
//! target your system.
//!
//! For a complete example, check the enclosed GCC ARM/Rust/QEMU based unit tests. The project
//! ``qemu_runner`` cross-compiles this library, compiles the main firmware using GCC ARM and links
//! in the appropriate entry points for unit tests. [GNU ARM Eclipse QEMU](http://gnuarmeclipse.github.io/qemu/)
//! is used to run the test binaries.
//!
//! Be sure to check the [FreeRTOS documentation](http://www.freertos.org/RTOS.html).
//!
//! # Samples
//!
//! Spawning a new task
//!
//! ```rust
//! # use freertos_rs::*;
//! Task::new().name("hello").stack_size(128).start(|| {
//! 	loop {
//! 		println!("Hello world!");
//! 		CurrentTask::delay(Duration::infinite());
//! 	}
//! }).unwrap();
//! ```
//!
//! Queue
//!
//! ```rust
//! # use freertos_rs::*;
//! let q = Queue::new(10).unwrap();
//! q.send(10, Duration::ms(5)).unwrap();
//! q.receive(Duration::infinite()).unwrap();
//! ```
//!
//! Mutex
//!
//! ```rust
//! # use freertos_rs::*;
//! let m = Mutex::new(0).unwrap();
//! {
//! 	let mut v = m.lock(Duration::infinite()).unwrap();
//! 	*v += 1;
//! }
//! ```

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate alloc;

//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!("bindings.rs");

mod hooks;
mod prelude;
mod shim;

mod base;
mod task;
mod timers;
mod queue;
mod semaphore;
mod mutex;
mod units;
mod utils;
mod isr;
mod delays;
mod critical;
mod allocator;

pub mod patterns;

pub use crate::base::FreeRtosError;
pub use crate::task::*;
pub use crate::queue::*;
pub use crate::units::*;
pub use crate::mutex::*;
pub use crate::semaphore::*;
pub use crate::isr::*;
pub use crate::delays::*;
pub use crate::timers::*;
pub use crate::critical::*;
pub use crate::hooks::*;
pub use crate::allocator::*;

pub use crate::utils::shim_sanity_check;
