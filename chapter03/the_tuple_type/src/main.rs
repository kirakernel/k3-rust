fn main() {
   let tup: (i32, f64, f64) = (3, 0.14, 0.0015);
   println!("{:?}", tup);

   let (x, y, z) = tup;

   println!("x: {}, y: {}, z: {}", tup.0, y, tup.2);
   println!("x: {}, y: {}, z: {}", x, y, z);
}
