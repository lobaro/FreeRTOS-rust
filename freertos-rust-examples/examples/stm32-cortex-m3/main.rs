#![no_std]
#![no_main]
// For allocator
#![feature(lang_items)]
#![feature(alloc_error_handler)]

extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use core::alloc::Layout;
use cortex_m::asm;
use cortex_m_rt::exception;
use cortex_m_rt::{entry, ExceptionFrame};
use freertos_rust::*;
use stm32l1xx_hal as hal;
use stm32l1xx_hal::gpio::gpioa::PA1;
use stm32l1xx_hal::gpio::*;
use stm32l1xx_hal::hal::digital::v2::*;

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

fn delay() {
    let mut _i = 0;
    for _ in 0..2_00 {
        _i += 1;
    }
}

fn delay_n(n: i32) {
    for _ in 0..n {
        delay();
    }
}

static mut LED: Option<PA1<Output<PushPull>>> = None;

fn set_led(on: bool) {
    unsafe {
        let mut led = LED.take().unwrap();
        if on {
            led.set_low();
        } else {
            led.set_high();
        }
    }
}

// Setup IO for the LED and blink, does not return.
fn do_blink_forever() -> ! {
    loop {
        delay();
        set_led(true);
        delay();
        set_led(false);
    }
}

#[entry]
fn main() -> ! {
    let dp = hal::stm32::Peripherals::take().unwrap();

    // Set up the LED, it's connected to pin PA1.
    let gpioa: stm32l1xx_hal::gpio::gpioa::Parts = dp.GPIOA.split();
    unsafe {
        LED = Some(gpioa.pa1.into_push_pull_output());
    }

    // Initial blink
    set_led(true);
    delay_n(10);
    set_led(false);
    delay_n(10);

    // Just blink (does NOT work!)
    do_blink_forever();

    // TODO: What comes now does not work yet!
    // Initialize Tasks and start FreeRTOS
    Task::new()
        .name("hello")
        .stack_size(128)
        .priority(TaskPriority(1))
        .start(|_this_task| {
            // Just blink
            freertos_rust::CurrentTask::delay(Duration::ms(1000));
            set_led(true);
            freertos_rust::CurrentTask::delay(Duration::ms(1000));
            set_led(false);
        })
        .unwrap();

    // TODO: Starting the scheduler fails, we need debugging to find the issue
    // Seems like we don't even get an assert
    FreeRtosUtils::start_scheduler();
}

#[exception]
fn DefaultHandler(_irqn: i16) {
    // custom default handler
    // irqn is negative for Cortex-M exceptions
    // irqn is positive for device specific (line IRQ)
    // set_led(true);(true);
    // panic!("Exception: {}", irqn);
}

#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    // Blink 3 times long when exception occures
    delay_n(10);
    for _ in 0..3 {
        set_led(true);
        delay_n(10);
        set_led(false);
        delay_n(5);
    }
    loop {}
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    set_led(true);
    asm::bkpt();
    loop {}
}
