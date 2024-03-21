#![allow(dead_code)]

fn main() {
    println!("The never type that never returns");
}

fn bar() -> ! {
    panic!();
}
