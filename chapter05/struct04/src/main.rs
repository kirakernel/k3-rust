use std::fmt::{self, Display, Formatter};

//#[allow(dead_code)]
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Color (red, green, blue) : #{:02X}{:02X}{:02X}",
            self.0, self.1, self.2
        )
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Point (x, y, z) : A({}, {}, {})", self.0, self.1, self.2)
    }
}

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", black);
    println!("{:?}", origin);
    println!("");
    println!("{}", black);
    println!("{}", origin);
}
