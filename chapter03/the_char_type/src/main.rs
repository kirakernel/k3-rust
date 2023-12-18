fn main() {
    let name = "Amy";
    let first_letter = 'A';
    let last_letter: char = 'Y'; // with explicit type annotation

    println!(
        "First letter, {1}, and Latst letter, {2}. Her name is {0}.",
        name, first_letter, last_letter
    )
}
