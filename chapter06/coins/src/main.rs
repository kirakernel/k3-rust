#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Luck penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;

    println!("{:?} - {:?} - {:?} - {:?}", penny, nickel, dime, quarter);

    value_in_cents(penny);
    value_in_cents(nickel);
    value_in_cents(dime);
    value_in_cents(quarter);
}
