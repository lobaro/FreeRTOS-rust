use freertos_rust::add;

fn main() {
    unsafe {
        let res = add(2, 3);
        println!("2 + 3 = {}!", res);
    }

}
