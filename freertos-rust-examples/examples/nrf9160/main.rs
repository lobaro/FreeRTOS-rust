#![no_std]
#![no_main]
// For allocator
#![feature(lang_items)]
#![feature(alloc_error_handler)]


use cortex_m_rt::{entry, exception, ExceptionFrame};
use nrf9160_pac as nrf9160;
use cortex_m::asm;

use core::panic::PanicInfo;
use freertos_rust::*;
use core::alloc::Layout;

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    do_blink();
    // custom default handler
    // irqn is negative for Cortex-M exceptions
    // irqn is positive for device specific (line IRQ)
    //board_set_led(true);
    //panic!("Exception: {}", irqn);
}


#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    do_blink();
    loop {}
}

fn do_blink() {
    let periphs = nrf9160::Peripherals::take().unwrap();
    let p0 = periphs.P0_S;

    p0.dir.write(|w| w.pin31().bit(true));

    loop {
        let mut baz = 0;
        p0.outset.write(|w| w.pin31().bit(true));
        for i in 1..20000 {
            baz = i * 10;
        }
        p0.outclr.write(|w| w.pin31().bit(true));

        for i in 1..20000 {
            baz = i * 10;
        }
    }
}

#[entry]
fn main() -> ! {
    //asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    let h = Task::new().name("hello").stack_size(512).priority(TaskPriority(1)).start(|| {
        // Blink forever
        do_blink();
        loop {

        }
    }).unwrap();

    FreeRtosUtils::start_scheduler();
}

fn test_function(arg: i32) -> i32 {
    let mut temp: f64 = arg as f64;
    temp = temp * 3.1415;
    temp as i32
}

// -xc /Users/lobaro/cpath/src/github.com/lobaro/c-build/gdb/gdb_nrf9160.txt
// monitor reset
// Workaround for https://github.com/rust-embedded/cortex-m-rt/issues/139
// on stable: GDB set backtrace limit 32
// or rustup default nightly or beta (>1.43.6)

// FreeRTOS handler

#[no_mangle]
fn vApplicationMallocFailedHook() {}

#[no_mangle]
fn vApplicationIdleHook() {}

#[no_mangle]
fn vApplicationStackOverflowHook(pxTask: FreeRtosTaskHandle, pcTaskName: FreeRtosCharPtr) {}

#[no_mangle]
fn vApplicationTickHook() {}