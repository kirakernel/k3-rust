fn main() {
    let x = Box::new(5);
    let y = Box::new(10);

    let sum = *y + *x;

    println!("{} + {} = {}.", x, y, sum);
}
