#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("Integer: {:?}", both_integer);
    println!("Float: {:?}", both_float);
    println!("Integer and Float: {:?}", integer_and_float);
}
