use a_blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    println!("Draft: {}", post.content());

    post.request_review();
    assert_eq!("", post.content());
    println!("Pending review: {}", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("Published: {}", post.content());
}
