fn main() {
    println!("");
    for i in 0..11 {
        print!("Times table of {i}\n");
        for j in 0..11 {
            println!("{} x {} = {}", i, j, i * j);
        }
        println!("");
    }
}
