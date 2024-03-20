static HELLO_WORLD: &str = "Hello, World!";
static mut COUNTER: u32 = 0;

fn main() {
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("\nCOUNTER: {}", COUNTER);
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
