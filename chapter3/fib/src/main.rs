use std::io;

fn main() {
    let mut number = String::new();

    println!("==Generate the nth Fibonacci number==");
    println!("Type a number: ");
    
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = number.trim().parse().expect("a valid number");
    
    fib(number);
}

// In mathematics, the Fibonacci sequence is in which 
// each number is the sum of the two preceding ones.
// https://en.wikipedia.org/wiki/Fibonacci_sequence
// 
// Source of inspiration:
// https://docs.python.org/3/tutorial/controlflow.html#defining-functions
fn fib(n: i32) {
    //let mut sequence = "";
    let mut index = 0;
    let [mut a, mut b] = [0, 1];
    while index < n {
        print!("{} ", a);
        [a, b] = [b, a + b];
         index += 1;
    }
    println!("");
}
