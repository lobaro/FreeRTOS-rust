use freertos_rust::{xPortGetFreeHeapSize, add, xPortStartScheduler};

fn main() {
    unsafe {
        let res = add(2, 3);
        println!("2 + 3 = {}", res);

        let free = xPortGetFreeHeapSize();
        println!("Free Memory: {}!", free);

        xPortStartScheduler();
    }
}
