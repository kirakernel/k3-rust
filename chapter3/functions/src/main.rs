fn main() {
    println!("Hello, world!");
    another_function();
    print_x(10);
    print_labeled_measurement(5, 'h');
    let x = five();
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", plus_one(x));
}

fn another_function() {
    println!("Another function.")
}

fn print_x(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
