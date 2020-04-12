use ::freertos_rust::*;
use std::borrow::{BorrowMut, Borrow};

fn main() {
    unsafe {
        FREERTOS_HOOKS.set_on_assert(|| { println!("Assert hook called") });
    }

    FreeRtosUtils::invoke_assert();

    println!("Starting FreeRTOS app ...");
    Task::new().name("hello").stack_size(128).priority(TaskPriority(2)).start(|| {
        let mut i = 0;
        loop {
            println!("Hello from Task! {}", i);
            CurrentTask::delay(Duration::ms(1000));
            i = i + 1;
        }
    }).unwrap();
    println!("Task registered");
    //let free = freertos_rs_xPortGetFreeHeapSize();
    // println!("Free Memory: {}!", free);
    println!("Starting scheduler");
    FreeRtosUtils::start_scheduler();
    loop {
        println!("Loop forever!");
    }
}
