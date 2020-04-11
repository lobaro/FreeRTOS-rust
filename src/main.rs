use ::freertos_rust::*;

fn main() {
    unsafe {
        let res = add(2, 3);
        println!("2 + 3 = {}", res);
        initialiseHeap();
        assert(false);

        // Invokes assert
        //CurrentTask::delay(Duration::ms(100));

        println!("Starting FreeRTOS app ...");
        Task::new().name("hello").stack_size(128).priority(TaskPriority(2)).start(|| {
            loop {
                println!("Hello from Task!");
                CurrentTask::delay(Duration::ms(100));
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
}
