fn main() {
    let x = Some(5);

    let result = match x {
        None => None,
        Some(i) => Some(i + 1),
    };

    println!("Result={}", result.unwrap());
}
