#[macro_export]
macro_rules! vec2 {
    ($($x:expr), *) => {
        {
	    let mut temp_vec = Vec::new();
	    $(
	        temp_vec.push($x);
	    )*
	    temp_vec
	 }
    };
}

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec2![4, 5, 6];
    println!("vec1={:?}, vec2={:?}", vec1, vec2);
}
