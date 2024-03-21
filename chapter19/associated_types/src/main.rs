/*pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}*/

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

struct User<'a> {
    id: &'a str,
    name: &'a str,
    n_fields: u32,
}

impl<'a> User<'a> {
    fn new(id: &'a str, name: &'a str) -> Self {
        Self {
            id,
            name,
            n_fields: 2,
        }
    }
}

impl<'a> Iterator for User<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.n_fields {
            1 => {
                self.n_fields -= 1;
                return Some(self.id);
            }
            2 => {
                self.n_fields -= 1;
                return Some(self.name);
            }
            _ => None,
        }
    }
}

fn main() {
    let user = User::new("01122222225988988", "Amy");

    for elem in user {
        println!("{:?}", elem);
    }

    let counter = Counter::new();
    for i in counter {
        println!("i={}", i);
    }
}
