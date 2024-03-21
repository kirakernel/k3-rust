use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let point_a = Point { x: 1, y: 0 };
    let point_b = Point { x: 2, y: 3 };
    println!("{:?}", point_a + point_b);

    let meters = Meters(100);
    let millimeters = Millimeters(1000);

    let result = millimeters + meters;
    println!("result={:?}", result)
}
