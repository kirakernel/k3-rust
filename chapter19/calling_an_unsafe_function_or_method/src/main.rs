fn main() {
    println!("Unsafe code!!!");
    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {}
