#![no_std]
#![no_main]

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
use cortex_m_semihosting::{hio};
use crate::hw::VolatileStruct;

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

#[entry]
fn main() -> ! {
    let rcc = unsafe { hw::RCC::from_addr(RCC_BASE) };
    let gpio_a = unsafe { hw::GPIO::from_addr(GPIOA_BASE) };


    rcc.ahbenr.write(1);
    gpio_a.moder.write(4);


    loop {
        let mut i = 0;
        for _ in 0..2_00 {
            i+=1;
        }
        set_led(gpio_a, true);
        for _ in 0..2_00 {
            i+=1;
        }
        set_led(gpio_a, false);
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
