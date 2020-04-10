use freertos_rust::*;

fn main() {
    unsafe {
        let res = add(2, 3);
        println!("2 + 3 = {}", res);
        initialiseHeap();

        println!("Delay 1000...");
        freertos_rs_vTaskDelay(1000);
        println!("1 Second later");



        let free = xPortGetFreeHeapSize();
        println!("Free Memory: {}!", free);



        xPortStartScheduler();
    }
}
