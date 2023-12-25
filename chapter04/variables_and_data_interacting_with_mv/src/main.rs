fn main() {
    let x = 5;
    let y = x;

    println!("x={}, y={}", x, y);

    let s1 = String::from("hello");

    // let s2 = s1;
    // Variables and Data Interacting with Clone
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
