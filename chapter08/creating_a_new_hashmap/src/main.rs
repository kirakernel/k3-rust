use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    print_score(scores);
}

fn print_score(scores: HashMap<String, i32>) {
    for (time, score) in &scores {
        println!("{}: {} points.", time, score);
    }
}
