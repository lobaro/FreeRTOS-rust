#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::exception;
use cortex_m_rt::{entry, ExceptionFrame};
use embedded_hal::digital::OutputPin;
use freertos_rust::*;
use stm32f4xx_hal::gpio::*;

use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

#[entry]
fn main() -> ! {
    Task::new()
        .name("default")
        .stack_size(1000)
        .start(move |_| {
            app_main();
        })
        .unwrap();
    FreeRtosUtils::start_scheduler();
}

fn app_main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(100.MHz()).freeze();

    let gpioc = dp.GPIOC.split();
    let mut device = MyDevice::from_pins(gpioc.pc13.into_open_drain_output());
    device.set_led(false);
    Task::new()
        .name("hello")
        .stack_size(128)
        .priority(TaskPriority(2))
        .start(move |_| loop {
            CurrentTask::delay(Duration::ms(1000));
            device.set_led(true);
            CurrentTask::delay(Duration::ms(1000));
            device.set_led(false);
        })
        .unwrap();

    loop {
        CurrentTask::delay(Duration::ms(1000));
    }
}

pub struct MyDevice<D1: OutputPin> {
    d1: D1,
}

impl<D1: OutputPin> MyDevice<D1> {
    pub fn from_pins(d1: D1) -> MyDevice<D1> {
        MyDevice { d1 }
    }

    pub fn set_led(&mut self, on: bool) {
        if on {
            self.d1.set_low().ok();
        } else {
            self.d1.set_high().ok();
        }
    }
}

#[allow(non_snake_case)]
#[exception]
unsafe fn DefaultHandler(_irqn: i16) {
    // custom default handler
    // irqn is negative for Cortex-M exceptions
    // irqn is positive for device specific (line IRQ)
    // set_led(true);(true);
    // panic!("Exception: {}", irqn);
    asm::bkpt();
    loop {}
}

#[allow(non_snake_case)]
#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();
    loop {}
}

// We no longer need to use #[alloc_error_handler] since v1.68.
// It will automatically call the panic handler.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    asm::bkpt();
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
fn vApplicationStackOverflowHook(_pxTask: FreeRtosTaskHandle, _pcTaskName: FreeRtosCharPtr) {
    asm::bkpt();
}
