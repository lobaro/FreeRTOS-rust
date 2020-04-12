#![no_std]
#![no_main]
// For allocator
#![feature(lang_items)]
#![feature(alloc_error_handler)]

// Remove this later and clean up code
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_assignments)]
#[allow(unused_attributes)]
pub mod hw;

// release profile: minimize the binary size of the application
// TODO: We want some custom panic handler that logs & resets
#[cfg(not(debug_assertions))]
extern crate panic_abort;
// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
//#[cfg(debug_assertions)]
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
//#[cfg(debug_assertions)]
//extern crate panic_semihosting;
// logs messages to the host stderr; requires a debugger

// requires nightly

use core::fmt::Write;
// extern crate panic_halt; // just stop the world
// extern crate panic_itm; // logs messages over ITM; requires ITM support
use core::ptr;

use cortex_m::asm;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::exception;
use cortex_m_rt::{entry, ExceptionFrame};
use cortex_m_semihosting::hio::HStdout;
use cortex_m_semihosting::hio;
use alloc_cortex_m::CortexMHeap;
use crate::hw::VolatileStruct;
use freertos_rust::*;
use core::alloc::Layout;

const PERIPH_BASE: u32 = 0x40000000;
const AHBPERIPH_BASE: u32 = PERIPH_BASE + 0x20000;
const GPIOA_BASE: u32 = AHBPERIPH_BASE + 0x0000;
const RCC_BASE: u32 = AHBPERIPH_BASE + 0x3800;

fn set_led(gpio: &mut hw::GPIO, on: bool) {
    set_gpio(gpio, 1, on)
}

fn set_gpio(gpio: &mut hw::GPIO, pin: u8, state: bool) {
    if state {
        gpio.bssrl |= 1 << pin;
    } else {
        gpio.bssrh |= 1 << pin;
    }
}


// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 512; // in bytes

fn do_blink() {
    // Initialize LED
    let rcc = unsafe { hw::RCC::from_addr(RCC_BASE) };
    let gpio_a = unsafe { hw::GPIO::from_addr(GPIOA_BASE) };
    rcc.ahbenr.write(1);
    gpio_a.moder.write(4);

    let mut i = 0;
    loop {
        //println!("Hello from Task! {}", i);
        //CurrentTask::delay(Duration::ms(1000));
        //i = i + 1;


        let mut i = 0;
        for _ in 0..2_00 {
            i += 1;
        }
        set_led(gpio_a, true);
        for _ in 0..2_00 {
            i += 1;
        }
        set_led(gpio_a, false);
    }
}

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    //unsafe { initialiseHeap(); }

   // do_blink();

    // Initialize Tasks and start FreeRTOS

    //println!("Starting FreeRTOS app ...");
    Task::new().name("hello").stack_size(128).priority(TaskPriority(2)).start(|| {
        // Initialize LED
        let rcc = unsafe { hw::RCC::from_addr(RCC_BASE) };
        let gpio_a = unsafe { hw::GPIO::from_addr(GPIOA_BASE) };
        rcc.ahbenr.write(1);
        gpio_a.moder.write(4);

        let mut i = 0;
        loop {
            //println!("Hello from Task! {}", i);
            //CurrentTask::delay(Duration::ms(1000));
            //i = i + 1;


            let mut i = 0;
            for _ in 0..2_00 {
                i += 1;
            }
            set_led(gpio_a, true);
            for _ in 0..2_00 {
                i += 1;
            }
            set_led(gpio_a, false);
        }
    }).unwrap();
    //println!("Task registered");
    //let free = freertos_rs_xPortGetFreeHeapSize();
    // println!("Free Memory: {}!", free);
    //println!("Starting scheduler");
    FreeRtosUtils::start_scheduler();
    loop {
        //println!("Loop forever!");
    }
}

#[exception]
fn DefaultHandler(irqn: i16) {
    // custom default handler
    // irqn is negative for Cortex-M exceptions
    // irqn is positive for device specific (line IRQ)
    panic!("Exception: {}", irqn);
}

#[exception]
fn SysTick() {
    static mut COUNT: u32 = 0;
    static mut STDOUT: Option<HStdout> = None;

    *COUNT += 1;

    // Lazy initialization
    if STDOUT.is_none() {
        *STDOUT = hio::hstdout().ok();
    }

    if let Some(hstdout) = STDOUT.as_mut() {
        write!(hstdout, "{}", *COUNT).ok();
    }

    // IMPORTANT omit this `if` block if running on real hardware or your
    // debugger will end in an inconsistent state
    if *COUNT == 9 {
        // This will terminate the QEMU process
        panic!("9 is enough!");
    }
}

// TODO: Not working!!! (at least in QEMU)
#[allow(dead_code)]
fn trigger_hard_fault() {
    // read a nonexistent memory location
    unsafe {
        ptr::read_volatile(0x3FFF_FFFE as *const u32);
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    if let Ok(mut hstdout) = hio::hstdout() {
        writeln!(hstdout, "{:#?}", ef).ok();
    }

    loop {}
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}