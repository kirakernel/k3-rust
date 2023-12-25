fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print hello, world!

    let mut s = String::from("How are you");
    s.push_str("?");

    println!("{}", s);
}
