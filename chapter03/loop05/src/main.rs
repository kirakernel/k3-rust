fn main() {
    let mut i = 0;

    println!("");
    while i <= 10 {
        println!("Times table of {i}");
        let mut j = 0;
        while j <= 10 {
            println!("{i} x {j} = {product}", product = i * j);
            j += 1;
        }
        println!("");
        i += 1;
    }
}
