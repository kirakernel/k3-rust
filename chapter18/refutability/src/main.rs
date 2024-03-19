fn main() {
    let some_option_value = Some(5);
    // let Some(x) = some_option_value;

    if let Some(value) = some_option_value {
        println!("{}", value);
    }

    // if let x = 5 {
    //     println!("{}", x);
    // }

    let x = 5;
    println!("{}", x);
}
