#[derive(Debug)]
struct User {
    _active: bool,
    _username: String,
    _email: String,
    _sign_in_count: u64,
}

fn main() {
    let username = String::from("someusername1");
    let email = String::from("someone@example.com");
    let user1 = build_user(username, email);
    let user2 = User {
        _email: String::from("another@example.com"),
        ..user1
    };

    println!("{:#?}\n===!", user2);
}

fn build_user(_username: String, _email: String) -> User {
    User {
        _active: true,
        _username,
        _email,
        _sign_in_count: 1,
    }
}
