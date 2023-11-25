/*fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
*/
fn main() {
    // fn square (num: i32) -> i32 { num * num }
    // let square = |num: i32| -> i32 { num * num };
    // let square = |num| { num * num };
    let square = |num| num * num;

    for i in 0..10 {
        println!("{} squared is {}", i, square(i));
    }
}
