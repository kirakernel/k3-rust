fn main() {
   let num1 = 5;
   let num2: Option<i32> = None;

   let sum = match num2 {
        None => num1 + 2,
        Some(n) => num1 + n,
   };

   println!("The sum is: {}", sum);
}
