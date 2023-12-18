fn main() {
    let temperature: i8 = -4;
    println!("{temperature}");
    let hours: u8 = 24;
    println!("{hours}");  

    let temperature: i16 = -4;
    println!("{temperature}");
    let hours: u16 = 24;
    println!("{hours}");  


    let temperature: i32 = -4;
    println!("{temperature}");
    let hours: u32 = 24;
    println!("{hours}");  


    let temperature: i64 = -4;
    println!("{temperature}");
    let hours: u64 = 24;
    println!("{hours}");

    let temperature: i128 = -4;
    println!("{temperature}");
    let hours: u128 = 24;
    println!("{hours}");

    let temperature: isize = -4;
    println!("{temperature}");
    let hours: usize = 24;
    println!("{hours}");


    println!("");
    // Decimal
    let number = 98_222;
    println!("{number}");

    // Hex 
    let number = 0xff;
    println!("{number}");

    // Octal
    let number = 0o77;
    println!("{number}");

    // Binary
    let number = 0b1111_0000;
    println!("{number}");

    // Byte (u8 only)
    let number = b'A';
    println!("{number}");

}
