#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = build_user(String::from("ada01"), String::from("ada01@cs.net"));

    println!("{:?}", user1);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("ada01@stem.net"),
        sign_in_count: 1,
    };

    println!("{:?}", user2);

    let user3 = User {
        email: String::from("ada01@llm.net"),
        ..user2
    };

    println!("{:?}", user3);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
