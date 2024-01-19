// use std::fs::File;
// use std::io::{self, Read};

// fn main() {
//     read_username_from_file
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }
// use std::fs::File;
// use std::io::{self, Read};

// fn main() {
//     let mut passwd = File::open("/etc/passwd").unwrap();
//     let mut text = String::new();
//     let result = passwd
//         .read_to_string(&mut text)
//         .expect("couldn't read the file.");

//     println!("{:?}", result);
//     println!("{}", text);
// }

// use std::fs::File;
// use std::io::{self, Read};

// fn main() {}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// use std::fs::File;
// use std::io::{self, Read};

// fn main() {}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();

//     File::open("hello.txt")?.read_to_string(&mut username)?;

//     Ok(username)
// }

use std::fs;
use std::io;

fn main() {
    println!("{}", read_username_from_file("/etc/passwd").unwrap());
    println!("{}", read_username_from_file("hello.txt").unwrap());
}

fn read_username_from_file(s: &str) -> Result<String, io::Error> {
    fs::read_to_string(s)
}
