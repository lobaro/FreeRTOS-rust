use ::freertos_rust::*;

fn main() {
    unsafe {
        let res = add(2, 3);
        println!("2 + 3 = {}", res);
        initialiseHeap();

        println!("Starting FreeRTOS app ...");

        Task::new().name("hello").stack_size(128).start(|| {
            loop {
                println!("Hello world!");
                CurrentTask::delay(Duration::ms(100));
            }
        }).unwrap();

        //freertos_rs_spawn_task(task1, null);
        //freertos_rs_vTaskDelay(1000);
        println!("Task registered");



        //let free = freertos_rs_xPortGetFreeHeapSize();
       // println!("Free Memory: {}!", free);

        FreeRtosUtils::start_scheduler();
        loop {
            println!("Loop forever!");
        }
    }
}
