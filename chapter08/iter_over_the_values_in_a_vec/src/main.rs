fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    println!("Before: {:?}", v);
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("After: {:?}", v);
}
