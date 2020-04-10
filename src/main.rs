use freertos_rust::{xPortGetFreeHeapSize, add, xPortStartScheduler};

fn main() {
    unsafe {
        let res = add(2, 3);
        let free = xPortGetFreeHeapSize();
        println!("2 + 3 = {} Free: {}!", res, free);


        xPortStartScheduler();
    }

}
