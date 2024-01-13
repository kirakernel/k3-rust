fn main() {
    let mut v = Vec::new();
    println!("Before push: {:?}", v);

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("After push: {:?}", v);
}
