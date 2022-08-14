use blog::Post;

fn main() {
    let mut post = Post::new(); // returns DraftPost

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review(); // return PendingReviewPost

    let post = post.approve(); // returns Post

    assert_eq!("I ate a salad for lunch today", post.content());



}
