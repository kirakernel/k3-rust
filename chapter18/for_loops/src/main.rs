fn main() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    println!("");

    let code = "var foo = 'text';";
    for (index, value) in code.char_indices() {
        println!("{} is at index {}", value, index);
    }
}
