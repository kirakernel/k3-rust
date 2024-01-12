use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(0, 1);
    map.insert(1, 2);
    map.insert(2, 3);

    let mut map2 = HashMap::new();

    map2.insert(0, 'a');
    map2.insert(1, 'b');
    map2.insert(2, 'c');

    let mut map3 = HashMap::new();

    map3.insert("name", "Nene");
    map3.insert("last_name", "Amano");

    println!("{:?}\n{:?}\n{:?}", map, map2, map3);
}
