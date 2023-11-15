//use aggregator::{Summary, Tweet};
use defining_a_trait::aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("up_and_up"),
        content: String::from("why is the world so evil?"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}
