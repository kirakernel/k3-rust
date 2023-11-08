fn main() {
    for c in "Зд".chars() {
        println!("{c}");
    }

    println!("");
    let s = String::from("hello!");
    for c in s.chars() {
        println!("{c}");
    }

    println!("");
    for b in "Зд".bytes() {
        println!("{b}");
    }

    println!("");
    for b in s.bytes() {
        println!("{b}");
    }
}
