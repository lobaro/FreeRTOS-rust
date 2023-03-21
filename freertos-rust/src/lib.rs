//! # FreeRTOS for Rust
//!
//! Rust interface for the FreeRTOS embedded operating system.
//! It is assumed that dynamic memory allocation is provided on the target system.
//! For Rust versions 1.68 and later, the stable channel can be used.
//! Prior to version 1.68, the nightly channel is required along with
//! enabling the `alloc_error_handler` feature.
//!
//! This library interfaces with FreeRTOS using a C shim library which provides function
//! wrappers for FreeRTOS macros. The compiled Rust application should be linked to the
//! base C/C++ firmware binary.
//!
//! Examples are provided inside [freertos-rust-examples](https://github.com/lobaro/FreeRTOS-rust/tree/master/freertos-rust-examples)
//!
//! For more examples, check the enclosed GCC ARM/Rust/QEMU based unit tests. The project
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
//!
//! FreeRtosUtils::start_scheduler();
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

#[cfg_attr(any(feature = "time", feature = "sync"), macro_use)]
extern crate alloc;

#[cfg(feature = "hooks")]
mod hooks;
mod prelude;
mod shim;

#[cfg(feature = "allocator")]
mod allocator;
mod base;
#[cfg(feature = "sync")]
mod critical;
#[cfg(feature = "time")]
mod delays;
#[cfg(feature = "interrupt")]
mod isr;
#[cfg(feature = "sync")]
mod mutex;
#[cfg(feature = "sync")]
mod queue;
#[cfg(feature = "sync")]
mod semaphore;
#[cfg(any(feature = "time", feature = "sync"))]
mod task;
#[cfg(feature = "time")]
mod timers;
#[cfg(any(feature = "time", feature = "sync"))]
mod units;
mod utils;

#[cfg(feature = "sync")]
pub mod patterns;

// Internal stuff that is only public for first Proof of Concept
pub use crate::base::*;
pub use crate::shim::*;
// ----------

#[cfg(feature = "allocator")]
pub use crate::allocator::*;
pub use crate::base::FreeRtosError;
#[cfg(feature = "sync")]
pub use crate::critical::*;
#[cfg(feature = "time")]
pub use crate::delays::*;
#[cfg(feature = "hooks")]
pub use crate::hooks::*;
#[cfg(feature = "interrupt")]
pub use crate::isr::*;
#[cfg(feature = "sync")]
pub use crate::mutex::*;
#[cfg(feature = "sync")]
pub use crate::queue::*;
#[cfg(feature = "sync")]
pub use crate::semaphore::*;
#[cfg(any(feature = "time", feature = "sync"))]
pub use crate::task::*;
#[cfg(feature = "time")]
pub use crate::timers::*;
#[cfg(any(feature = "time", feature = "sync"))]
pub use crate::units::*;

#[cfg(feature = "cpu_clock")]
pub use crate::utils::cpu_clock_hz;
pub use crate::utils::shim_sanity_check;
