fn main() {
    // let reference_to_nothing = dangle();
    // println!("Message:<{}>", reference_to_nothing);

    let hello = no_dangle();
    println!("Message:<{}>", hello);
}

/*
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}*/

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
