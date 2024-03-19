fn main() {
    let (x, y, z) = (1, 2, 3);
    println!("{}, {}, and {}", x, y, z);

    let (x, y, _) = (1, 2, 3);
    println!("{} and {}", x, y);

    let (x, _, _) = (1, 2, 3);
    println!("{x}");
}
