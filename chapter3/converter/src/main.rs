use std::io;

fn main() {
    let mut temperature = String::new();
    let mut choice = String::new();
    let (f, c) = ("f", "c");

    println!("Type a temperature? ");

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    println!("Do you want to convert to Fahrenheit(F) or Celsius(C)? ");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let temperature: i32 = temperature.trim().parse().expect("it needs a number");

    if choice.trim().to_lowercase() == f {
        println!("Given temperature: {temperature}C");
        println!("{}C is {}F", temperature, fahrenheit(temperature));
    } else if choice.trim().to_lowercase() == c {
        println!("Given temperature: {temperature}F");
        println!("{}F is {}C", temperature, celsius(temperature));
    } else {
        println!("I am sorry but I cannot help you:(");
    }
    println!("Bye!");
}

// Fahrenheit to Celsius is a term that is used to
// convert a given temperature scale to the Fahrenheit
// scale to the Celsius scale.
//
// Fahrenheit to Celsius Formula: C = (F - 32) * 5 / 9
// Where:
// C = Measure of temperature in degress Celsius
// F = Measure of temperature in degress Fahrenheit
fn celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5 / 9
}

// Celsius to Fahrenheit is the conversion of
// temperature from Celsius unit to Fahrenheit unit
//
// The formula to convert Celsius to Fahrenheit
// is given by: F = C * 9/5 + 32
// Where:
// C = Measure of temperature in degress Celsius
// F = Measure of temperature in degress Fahrenheit
fn fahrenheit(celsius: i32) -> i32 {
    (celsius * 9 / 5) + 32
}
