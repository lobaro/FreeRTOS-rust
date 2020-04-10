use freertos_rust::*;
use core::mem;

fn task1() {

}

fn main() {
    unsafe {
        let res = add(2, 3);
        println!("2 + 3 = {}", res);
        initialiseHeap();

        println!("Delay 1000...");

        Task::new().name("hello").stack_size(128).start(|| {
            loop {
                println!("Hello world!");
                CurrentTask::delay(Duration::infinite());
            }
        }).unwrap();

        //freertos_rs_spawn_task(task1, null);
        //freertos_rs_vTaskDelay(1000);
        println!("1 Second later");



        let free = xPortGetFreeHeapSize();
        println!("Free Memory: {}!", free);


        xPortStartScheduler();
        loop {
            println!("Loop forever!");
        }
    }
}
