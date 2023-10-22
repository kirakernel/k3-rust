fn main() {
    let name1 = "Sarah";
    let name2 = String::from("Sarah");

    print_message(name1); // already &str
    print_message(&name2);

    println!("{} san konnichiwa", name2);
}

fn print_message(name: &str) {
    println!("Hello, {name}. Nice to meet you!");
}
