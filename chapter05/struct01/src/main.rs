use std::fmt::{self, Display, Formatter};

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let status = if self.active {
            "is online. Say someting..."
        } else {
            "is not online. Wait for..."
        };
        write!(f, "@{}<{}> - {}", self.username, self.email, status)
    }
}

fn main() {
    let user01 = User {
        active: true,
        username: "cute_daki".to_string(),
        email: "supercutedaki@cuteness.com".to_string(),
        sign_in_count: 3000,
    };

    println!("User 01:");
    println!("{}", user01);
    println!("{:?}\n", user01);

    let user02 = User {
        active: false,
        username: String::from("neneee"),
        email: String::from("neneee@cute.com"),
        sign_in_count: 4000,
    };

    println!("User 02: ");
    println!("{}", user02);
    println!("{:?}", user02)
}
